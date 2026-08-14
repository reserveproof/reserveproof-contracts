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

## Implementation Status

### Phase 1: MVP ✓ Complete
- **Admin Management**: Multi-admin authorization (initialize, add_admin, remove_admin)
- **Issuer Registry**: Register issuers with attestation parameters; update status and attestor sets
- **Single-Attestor Submission**: Submit attestations that finalize immediately (min_signers=1)
- **Reserve Ratio Calculation**: Fixed-point arithmetic in basis points (no floating point)
- **Testing**: 3/3 unit tests passing

### Phase 2: Multi-Attestor + Events ✓ Complete
- **Multi-Attestor Co-Signing**: Multiple signers can co-sign attestations with configurable thresholds
- **Threshold Finalization**: Attestation finalizes when signers >= min_signers
- **Staleness Detection**: `is_stale()` checks if attestation exceeds window; `flag_stale()` is permissionless
- **Event Emissions**: 
  - `EVENT_ATTESTATION_FINALIZED(issuer, attestation_id)` when threshold is met
  - `EVENT_ISSUER_FLAGGED_STALE(issuer)` when staleness is detected
- **Testing**: 3/3 unit tests + 4/4 integration tests passing

### Next: Testnet Deployment
- Deploy WASM to Stellar Testnet
- Document contract ID in this README
- End-to-end testing with mock issuers

## Contract Functions

### Admin (Phase 1)
- **`initialize(admin: Address)`** — Initialize the contract with an initial admin
  - Auth: Self-signed by admin
  - Returns: None
  
- **`add_admin(caller: Address, new_admin: Address)`** — Add a new admin to the multi-admin set
  - Auth: Requires existing admin
  - Returns: None
  
- **`remove_admin(caller: Address, admin_to_remove: Address)`** — Remove an admin
  - Auth: Requires existing admin
  - Returns: None

### Issuer Registry (Phase 1)
- **`register_issuer(caller: Address, issuer: Address, name: Symbol, asset: Address, attestation_window_seconds: u64, required_attestors: Vec<Address>, min_signers: u32)`** — Register an issuer
  - Auth: Requires admin
  - Parameters:
    - `issuer`: Address of the issuer (e.g., stablecoin issuer account)
    - `name`: Human-readable name (Symbol, max 32 chars)
    - `asset`: Address of the asset being backed
    - `attestation_window_seconds`: How often attestations must be submitted
    - `required_attestors`: List of authorized attestor addresses
    - `min_signers`: Minimum required co-signatures for finalization
  - Returns: None
  
- **`update_issuer_status(caller: Address, issuer: Address, status: IssuerStatus)`** — Suspend or activate an issuer
  - Auth: Requires admin
  - Parameters: `status` is `Active` or `Suspended`
  - Returns: None
  
- **`update_attestors(caller: Address, issuer: Address, required_attestors: Vec<Address>, min_signers: u32)`** — Update attestor set
  - Auth: Requires admin
  - Returns: None
  
- **`get_issuer(issuer: Address) -> Option<IssuerEntry>`** — Fetch issuer details
  - Auth: Public (no auth required)
  - Returns: IssuerEntry or None

### Attestation Lifecycle (Phase 1)
- **`submit_attestation(caller: Address, issuer: Address, reserve_balance: i128, outstanding_supply: i128, supporting_doc_hash: BytesN<32>) -> BytesN<32>`** — Submit a reserve attestation
  - Auth: Caller must be in issuer's required_attestors
  - Parameters:
    - `reserve_balance`: Amount of reserves (smallest units of the reporting currency)
    - `outstanding_supply`: Amount of tokens in circulation
    - `supporting_doc_hash`: SHA-256 hash of off-chain audit document
  - Behavior: If `min_signers=1`, attestation finalizes immediately and becomes current
  - Returns: `attestation_id` (SHA-256 hash used for co-signing)
  
- **`co_sign_attestation(caller: Address, attestation_id: BytesN<32>)`** — Co-sign an existing attestation
  - Auth: Caller must be a required attestor
  - Behavior: Increments signer count; finalizes if threshold met (Phase 2)
  - Returns: None
  
- **`get_latest_attestation(issuer: Address) -> Option<Attestation>`** — Fetch the current finalized attestation
  - Auth: Public
  - Returns: Latest finalized Attestation or None
  
- **`get_reserve_ratio(issuer: Address) -> Option<i128>`** — Get current reserve ratio
  - Auth: Public
  - Returns: `(reserve_balance * 10000) / outstanding_supply` in basis points, or None

### Staleness Watchdog (Phase 2) ✓
- **`is_stale(issuer: Address) -> bool`** — Check staleness
  - Auth: Public
  - Returns: true if latest attestation age exceeds attestation_window_seconds
  
- **`flag_stale(issuer: Address)`** — Permissionless staleness flag
  - Auth: None (permissionless — anyone can call)
  - Emits: `EVENT_ISSUER_FLAGGED_STALE(issuer)` if staleness detected

## Events

The contract emits the following events (indexed by Soroban):

- **`EVENT_ATTESTATION_FINALIZED(issuer: Address, attestation_id: BytesN<32>)`**
  - Emitted when an attestation reaches the min_signers threshold and is finalized
  - Either from `submit_attestation` (single-attestor) or `co_sign_attestation` (multi-attestor)
  - Useful for dashboards to track when reserve data is locked in

- **`EVENT_ISSUER_FLAGGED_STALE(issuer: Address)`**
  - Emitted when `flag_stale()` is called and the issuer's attestation exceeds the attestation window
  - Permissionless — anyone can trigger this event
  - Dashboard can use this to alert users that reserve data is stale

## Deployment

See CONTRIBUTING.md for testnet deployment steps.

See CONTRIBUTING.md for testnet deployment steps.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT License — see [LICENSE](LICENSE)