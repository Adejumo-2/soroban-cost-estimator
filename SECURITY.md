# Security Policy

## Disclaimer

This is **unaudited developer tooling**. It wraps Stellar's `simulateTransaction`
RPC to provide fee estimates and network config monitoring. **Do not rely solely
on this tool's output for financial decisions involving mainnet deployments.**
Always verify fee estimates against your target network before mainnet deploy.

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please report it
privately to the maintainers via Telegram (the ecosystem norm):

- **Telegram**: [t.me/stellar_dev](https://t.me/stellar_dev)

We will acknowledge receipt within 48 hours and work to address the issue
promptly. Please do not disclose the vulnerability publicly until we've had
a chance to address it.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Considerations

- This tool makes read-only RPC calls (`simulateTransaction`,
  `getLedgerEntries`). It does not deploy contracts or submit transactions.
- Config snapshots are stored locally in `~/.soroban-cost-estimator/`.
  Protect this directory if it contains sensitive network information.
- The tool does not use or manage secret keys. All simulations use a dummy
  source account (all zeros).
