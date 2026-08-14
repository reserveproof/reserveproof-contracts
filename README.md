# ReserveProof Contracts

Soroban smart contracts for on-chain proof-of-reserves attestation registry on Stellar.

## Overview

ReserveProof enables Stellar anchors and stablecoin issuers to publish periodic, signed reserve attestations (bank balance vs. outstanding supply) on-chain. The contracts provide:

- **Issuer Registry**: Register issuers with their attestation parameters
- **Attestation Lifecycle**: Submit and co-sign attestations with configurable multi-sig thresholds
- **Reserve Ratio Calculation**: Compute reserve/supply ratios in basis points
- **Staleness Watchdog**: Permissionless function to flag stale attestations when they exceed the configured window

## Legal Disclaimer

ReserveProof is a transparency and verification tool, not a substitute for a licensed, independent financial audit. It's designed to complement traditional audits with continuous, machine-verifiable attestations between audit cycles. This is not legal or financial advice — issuers should confirm what satisfies their jurisdiction's reserve-reporting requirements with qualified counsel.

## Building

Requires:
- Rust 1.84+
- Stellar CLI (https://developers.stellar.org/docs/build/smart-contracts/getting-started)

```bash
rustup target add wasm32v1-none
cargo build --target wasm32v1-none --release
```

## Testing

```bash
cargo test
```

Run tests against a local Stellar testnet sandbox (see CONTRIBUTING.md for setup).

## Contract Functions

### Admin
- `initialize(admin: Address)` — Initialize the contract with an admin
- `add_admin(caller: Address, new_admin: Address)` — Add an admin (requires existing admin auth)
- `remove_admin(caller: Address, admin_to_remove: Address)` — Remove an admin (requires existing admin auth)

### Issuer Registry
- `register_issuer(caller: Address, issuer: Address, name: String, asset: Address, attestation_window_seconds: u64, required_attestors: Vec<Address>, min_signers: u32)` — Register an issuer (requires admin auth)
- `update_issuer_status(caller: Address, issuer: Address, status: IssuerStatus)` — Update issuer status (requires admin auth)
- `update_attestors(caller: Address, issuer: Address, required_attestors: Vec<Address>, min_signers: u32)` — Update the attestor set (requires admin auth)
- `get_issuer(issuer: Address) -> Option<IssuerEntry>` — Fetch issuer details

### Attestation Lifecycle
- `submit_attestation(caller: Address, issuer: Address, reserve_balance: i128, outstanding_supply: i128, supporting_doc_hash: BytesN<32>) -> BytesN<32>` — Submit a new attestation (caller must be in issuer's required_attestors; returns attestation_id)
- `co_sign_attestation(caller: Address, attestation_id: BytesN<32>)` — Co-sign an existing attestation (caller must be a required attestor; finalizes if min_signers threshold is met)
- `get_latest_attestation(issuer: Address) -> Option<Attestation>` — Fetch the latest finalized attestation
- `get_reserve_ratio(issuer: Address) -> Option<i128>` — Get reserve/supply ratio in basis points (reserve_balance * 10000 / outstanding_supply)

### Staleness Watchdog
- `is_stale(issuer: Address) -> bool` — Check if the latest attestation is stale (beyond attestation_window_seconds)
- `flag_stale(issuer: Address)` — Flag an issuer as stale if the window has elapsed (permissionless; emits issuer_flagged_stale event)

## Deployment

See CONTRIBUTING.md for testnet deployment steps.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT License — see [LICENSE](LICENSE)