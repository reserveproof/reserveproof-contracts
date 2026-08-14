# Security Model: ReserveProof Contracts

This document outlines the security guarantees and threat model for the ReserveProof smart contracts on Stellar/Soroban.

## Authentication & Authorization

### Admin Functions
All admin-only functions require `require_auth()` on the caller:
- `initialize()` — Sets initial admins (authorization required)
- `add_admin()` — Adds new admin (admin auth required)
- `remove_admin()` — Removes admin (admin auth required)

**Guarantee:** Only accounts in the admin registry can call admin functions. Multi-admin support allows distributed trust.

### Attestor Functions
Attestors are managed per-issuer and validated by:
- `submit_attestation()` — Caller must be in issuer's attestor list
- `co_sign_attestation()` — Caller must be in issuer's attestor list

**Guarantee:** Only authorized attestors can submit or co-sign attestations.

### Public Functions
Read-only functions (`get_*`) are permissionless:
- `get_issuer()` — Public read, no auth required
- `get_attestation()` — Public read, no auth required
- `get_latest_attestation()` — Public read, no auth required
- `get_reserve_ratio()` — Public read, no auth required
- `is_stale()` — Public read, no auth required
- `flag_stale()` — Permissionless, anyone can flag stale issuers

## Data Integrity

### Multi-Signature Validation
- Requires `min_signers` threshold per issuer
- Signers must be unique (no duplicate co-signatures)
- Attestation state transitions atomically: `Pending` → `Finalized`
- **Guarantee:** Cannot finalize without meeting signer threshold

### Reserve Ratio Calculations
- Fixed-point arithmetic: basis points (0-10000)
- Formula: `(reserve_balance / outstanding_supply) * 10000`
- **Overflow Protection:** Rust u128 type prevents integer overflow
- **Division Safety:** Returns `None` if no attestation exists or supply is zero

### Staleness Detection
- Compares ledger time to attestation timestamp
- Window configured per-issuer (`stale_after_seconds`)
- **Guarantee:** Cannot be bypassed; managed by blockchain time

## Event Emissions

All events are emitted deterministically and include sufficient data for off-chain indexing:

| Event | Data | Use Case |
|-------|------|----------|
| `EVENT_ISSUER_REGISTERED` | issuer address | Indexer discovery |
| `EVENT_ISSUER_UPDATED` | issuer address | Issuer status changes |
| `EVENT_ATTESTATION_SUBMITTED` | issuer, attestation_id | Attestation tracking |
| `EVENT_ATTESTATION_COSIGNED` | issuer, attestation_id, signer | Multi-sig progress |
| `EVENT_ATTESTATION_FINALIZED` | attestation_id | Finalization confirmation |
| `EVENT_ISSUER_FLAGGED_STALE` | issuer | Staleness notification |

**Guarantee:** All state changes are auditable via events.

## Secrets & Sensitive Data

**No secrets stored in contract code:**
- No private keys
- No API credentials
- No authentication tokens
- RPC endpoints configured off-chain via environment variables

**Configuration via environment:**
- `SOROBAN_RPC_URL` — RPC endpoint (set in deployment)
- `SOROBAN_NETWORK_PASSPHRASE` — Network identifier (public)
- `RESERVEPROOF_CONTRACT_ID` — Contract address (public)

## Threat Model

### In-Scope (Protected)
- Unauthorized admin operations → Protected by `require_auth()`
- Unauthorized attestations → Protected by attestor list validation
- Multi-sig threshold bypass → Protected by atomic state transitions
- Reserve ratio manipulation → Protected by immutable calculation logic
- Signature spoofing → Protected by Stellar native auth

### Out-of-Scope (Accepted Risks)
- Contract upgrade vulnerabilities → Mitigated by testnet validation before deployment
- Ledger time manipulation → Accepted as blockchain-level risk
- Private key compromise → Owner responsibility
- RPC endpoint compromise → Operator responsibility (use trusted RPC)

## Security Updates

If a vulnerability is discovered:
1. Contact security@reserveproof.dev with details
2. Do not open public GitHub issues
3. Patch will be deployed after verification

## Dependency Security

- `soroban-sdk` v27 — Maintained by Stellar, regularly audited
- No external dependencies beyond Soroban SDK

Dependencies are audited via:
```bash
cargo audit
```

## Testing & Validation

All contract functions are covered by integration tests:
- Admin operations
- Issuer registry
- Attestation workflows
- Staleness detection
- Reserve ratio calculations

Code is compiled in release mode with optimizations:
```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true          # Link-time optimization
strip = true        # Remove debug symbols
```

## Deployment Checklist

Before mainnet deployment:
- [ ] Contract passes all integration tests
- [ ] Security review completed
- [ ] Testnet deployment verified
- [ ] Admins identified and authorized
- [ ] Initial issuers prepared
- [ ] RPC endpoints configured
- [ ] Monitoring and alerting set up
