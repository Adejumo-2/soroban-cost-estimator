//! soroban-cost-estimator — Estimate Soroban contract costs & track network pricing changes.
//!
//! This crate provides a CLI tool that wraps Stellar's `simulateTransaction` RPC
//! and adds awareness of how the network's resource-pricing configuration changes
//! over time.

// Allow dead code — many modules define types and functions that will be wired up
// as the tool matures (e.g. cache, estimate commands, fee calculation with config).
#![allow(dead_code)]

pub mod cache;
pub mod cli;
pub mod config_snapshot;
pub mod error;
pub mod report;
pub mod rpc;
pub mod wasm;
pub mod xdr_helper;
