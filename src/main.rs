use clap::Parser;
use soroban_cost_estimator::cache;
use soroban_cost_estimator::cli;
use soroban_cost_estimator::config_snapshot;
use soroban_cost_estimator::error;
use soroban_cost_estimator::report;
use soroban_cost_estimator::rpc;
use soroban_cost_estimator::wasm;
use soroban_cost_estimator::xdr_helper;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    if let Err(err) = run(args).await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

async fn run(args: cli::Cli) -> error::AppResult<()> {
    match args.command {
        cli::Command::Estimate {
            wasm,
            network,
            rpc_url,
            r#fn,
            id,
            args,
            json,
        } => {
            cmd_estimate(
                &wasm,
                &network,
                rpc_url.as_deref(),
                id.as_deref(),
                r#fn.as_deref(),
                &args,
                json,
            )
            .await
        }
    }
}
fn missing_simulation_data(resp: &rpc::simulate::SimulateTransactionResponse) -> bool {
    resp.cost.is_none() && resp.transaction_data.is_none() && resp.latest_ledger.is_none()
}

/// Extract resource usage from a simulation response.
///
/// Legacy RPC versions report CPU/memory in a `cost` object; modern versions
/// dropped `cost` and carry the resources (CPU instructions, footprint entry
/// counts, disk read/write bytes) inside `transactionData` XDR. Memory bytes
/// are only reported by the legacy `cost` object — modern RPC responses do
/// not expose them, so they report 0.
///
/// Returns (cpu_insns, mem_bytes, read_entries, write_entries, read_bytes,
/// write_bytes). Entry/byte counts are u32 to match `CostReport`.
fn response_resources(
    response: &rpc::simulate::SimulateTransactionResponse,
) -> error::AppResult<(u64, u64, u32, u32, u32, u32)> {
    let resources = rpc::simulate::parse_transaction_data_resources(&response.transaction_data)?;
    let (cpu, mem) = match &response.cost {
        Some(cost) => (cost.cpu_insns, cost.mem_bytes),
        None => (resources.map(|r| r.cpu_insns).unwrap_or(0), 0),
    };
    let entries = |count: usize| u32::try_from(count).unwrap_or(u32::MAX);
    let bytes = |count: u64| u32::try_from(count).unwrap_or(u32::MAX);
    Ok((
        cpu,
        mem,
        resources.map(|r| entries(r.read_entries)).unwrap_or(0),
        resources.map(|r| entries(r.write_entries)).unwrap_or(0),
        resources.map(|r| bytes(r.read_bytes)).unwrap_or(0),
        resources.map(|r| bytes(r.write_bytes)).unwrap_or(0),
    ))
}

/// Fetch fee rates from the network config (compute + ledger cost + bandwidth).
///
/// Returns a `FeeRates` struct with raw config rates. These are passed to
/// `compute_fee_breakdown` which does the proper `(units * rate) / scale`
/// math to preserve precision.
///
/// If any of the three `ConfigSetting*` sources cannot be fetched or
/// decoded, its rate(s) fall back to 0 and a warning is printed to stderr
/// — a silent zero rate would understate the non-refundable fee, so it must
/// never pass unannounced.
async fn fetch_fee_rates(client: &rpc::client::RpcClient) -> report::fee_calc::FeeRates {
    let mut degraded: Vec<&'static str> = Vec::new();

    let raw_compute =
        rpc::config::fetch_config_setting(client, rpc::config::ConfigSettingId::ContractComputeV0)
            .await;

    let raw_ledger_cost = rpc::config::fetch_config_setting(
        client,
        rpc::config::ConfigSettingId::ContractLedgerCostV0,
    )
    .await;

    let raw_bandwidth = rpc::config::fetch_config_setting(
        client,
        rpc::config::ConfigSettingId::ContractBandwidthV0,
    )
    .await;

    // ConfigSettingContractComputeV0.fee_rate_per_instructions_increment
    // is stroops per 10,000 instructions (not per instruction).
    let compute_per_10k = match raw_compute {
        Ok(raw) => match xdr_helper::decode_config_entry_xdr(&raw.config_xdr) {
            Ok(stellar_xdr::ConfigSettingEntry::ContractComputeV0(s)) => {
                s.fee_rate_per_instructions_increment
            }
            _ => {
                degraded.push("ContractComputeV0");
                0
            }
        },
        Err(_) => {
            degraded.push("ContractComputeV0");
            0
        }
    };

    // ConfigSettingContractLedgerCostV0: per-entry read/write fees and the
    // per-KB disk read fee — all part of the non-refundable fee in
    // stellar-core's resource fee model.
    let (read_entry, write_entry, read_1kb) = match raw_ledger_cost {
        Ok(raw) => match xdr_helper::decode_config_entry_xdr(&raw.config_xdr) {
            Ok(stellar_xdr::ConfigSettingEntry::ContractLedgerCostV0(s)) => (
                s.fee_disk_read_ledger_entry,
                s.fee_write_ledger_entry,
                s.fee_disk_read1_kb,
            ),
            _ => {
                degraded.push("ContractLedgerCostV0");
                (0, 0, 0)
            }
        },
        Err(_) => {
            degraded.push("ContractLedgerCostV0");
            (0, 0, 0)
        }
    };

    // ConfigSettingContractBandwidthV0.fee_tx_size1_kb
    // is stroops per 1KB of tx size (not per byte).
    let bandwidth_per_kb = match raw_bandwidth {
        Ok(raw) => match xdr_helper::decode_config_entry_xdr(&raw.config_xdr) {
            Ok(stellar_xdr::ConfigSettingEntry::ContractBandwidthV0(s)) => s.fee_tx_size1_kb,
            _ => {
                degraded.push("ContractBandwidthV0");
                0
            }
        },
        Err(_) => {
            degraded.push("ContractBandwidthV0");
            0
        }
    };

    if !degraded.is_empty() {
        eprintln!(
            "Warning: fee rate source(s) {} unavailable — affected rate(s) set to 0 (non-refundable fee understated)",
            degraded.join(", ")
        );
    }

    report::fee_calc::FeeRates {
        fee_per_10k_insns: compute_per_10k,
        fee_per_read_entry: read_entry,
        fee_per_write_entry: write_entry,
        fee_per_read_1kb: read_1kb,
        fee_per_1kb: bandwidth_per_kb,
    }
}

/// `estimate` command: simulate a single invocation and print cost report.
async fn cmd_estimate(
    wasm_path: &str,
    network: &str,
    rpc_url: Option<&str>,
    contract_id: Option<&str>,
    fn_name: Option<&str>,
    args: &[String],
    json_flag: bool,
) -> error::AppResult<()> {
    use sha2::Digest;

    let wasm_info = wasm::parser::load_wasm(std::path::Path::new(wasm_path))?;
    let endpoint = rpc::client::resolve_endpoint(network, rpc_url)?;
    let client = rpc::client::RpcClient::new(&endpoint);

    let sc_vals: Vec<stellar_xdr::ScVal> = args
        .iter()
        .map(|a| xdr_helper::parse_arg_scval(a))
        .collect();

    // Raw XDR bytes: the transaction size for the bandwidth fee must be the
    // XDR byte count, not the base64 length (base64 inflates it by ~33%).
    let tx_xdr =
        xdr_helper::build_simulation_tx_envelope(&wasm_info.bytes, contract_id, fn_name, &sc_vals)?;
    let tx_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &tx_xdr);

    let response = rpc::simulate::simulate_transaction(&client, &tx_b64).await?;

    let wasm_hash = hex::encode(sha2::Sha256::digest(&wasm_info.bytes));

    // Guard: a simulation that returns no cost data and no ledger is almost
    // certainly a misconfigured request (bad --id, wrong network, or an RPC
    // schema drift), not a free transaction. Fail loudly instead of silently
    // printing an all-zero report.
    if missing_simulation_data(&response) {
        return Err(error::AppError::SimulationFailed(
            "simulation returned no cost data and no latest ledger — check --id, --fn, and the RPC endpoint".to_string(),
        ));
    }

    // Resource usage: legacy RPCs report it in `cost`, modern ones carry it
    // in `transactionData` XDR. See `response_resources`.
    let (cpu_instructions, memory_bytes, read_entries, write_entries, read_bytes, write_bytes) =
        response_resources(&response)?;

    let latest_ledger: u32 = response
        .latest_ledger
        .and_then(|l| u32::try_from(l).ok())
        .unwrap_or(0);

    // Authoritative total resource fee: prefer the RPC's `minResourceFee`
    // (the minimum the network charges), fall back to the XDR-derived
    // `transaction_data.resource_fee` when it is omitted. Both carry the
    // same value on a successful simulation. A malformed source is ignored
    // in favor of the other, since the total only feeds the breakdown display.
    let total_fee_stroops = rpc::simulate::parse_resource_fee(&response.min_resource_fee)
        .unwrap_or(None)
        .or(rpc::simulate::parse_transaction_data_resource_fee(
            &response.transaction_data,
        )?)
        .unwrap_or(0);

    // Fetch the real fee rates from the network
    let fee_rates = fetch_fee_rates(&client).await;

    let fee = report::fee_calc::compute_fee_breakdown(
        total_fee_stroops,
        cpu_instructions,
        read_entries,
        write_entries,
        read_bytes,
        tx_xdr.len() as u32,
        fee_rates,
    );

    let function_name = fn_name.unwrap_or("(wasm upload)");

    let report = report::cost_report::CostReport {
        function: function_name.to_string(),
        wasm_hash: wasm_hash.clone(),
        cpu_instructions,
        memory_bytes,
        tx_size: tx_xdr.len() as u32,
        read_entries,
        write_entries,
        read_bytes,
        write_bytes,
        fee: fee.clone(),
        ledger: latest_ledger,
        network: network.to_string(),
    };

    // Save to cache
    let _ = cache::save_estimate(
        &wasm_hash,
        function_name,
        args,
        network,
        latest_ledger,
        fee.total_stroops,
        cpu_instructions,
        memory_bytes,
    );

    if json_flag {
        println!("{}", report::cost_report::format_report_json(&report));
    } else {
        println!("{}", report::cost_report::format_report_table(&report));
    }

    Ok(())
}

