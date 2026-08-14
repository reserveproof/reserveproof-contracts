# Deployment Guide: ReserveProof Contracts

This guide covers deploying the ReserveProof smart contract to Stellar Soroban testnet and production networks.

## Prerequisites

1. **Rust & Soroban SDK**
   ```bash
   rustup install stable
   rustup target add wasm32v1-none
   cargo install soroban-cli
   ```

2. **Stellar CLI**
   ```bash
   brew install stellar-cli  # macOS
   # or download from https://github.com/stellar/stellar-cli
   ```

3. **Testnet Setup**
   ```bash
   soroban config network add testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

4. **Credentials**
   - Create or import a keypair:
   ```bash
   soroban config identity generate deployer
   ```
   - Fund testnet account: https://stellar.org/laboratory

## Compiling the Contract

```bash
# Build the contract WASM binary
cargo build --target wasm32v1-none --release

# Output: target/wasm32v1-none/release/reserveproof_contracts.wasm
```

## Testnet Deployment

### Step 1: Build the Contract
```bash
cargo build --target wasm32v1-none --release
export WASM_FILE="target/wasm32v1-none/release/reserveproof_contracts.wasm"
```

### Step 2: Deploy the Contract
```bash
CONTRACT_ID=$(soroban contract deploy \
  --wasm "$WASM_FILE" \
  --source deployer \
  --network testnet)

echo "Contract deployed to: $CONTRACT_ID"
```

### Step 3: Initialize the Contract
```bash
ADMIN_KEYPAIR="$(soroban config identity show deployer)"

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$ADMIN_KEYPAIR"

echo "Contract initialized with admin: $ADMIN_KEYPAIR"
```

### Step 4: Verify Deployment
```bash
# Check contract is callable
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- get_issuer \
  --issuer "GBUDQ6HKJM47U3Z5NQFZLONV6FQMEXOQTBGMF3FOU3MZT5OXVSRX3P3L"

# Should return: {"discriminant":0}  (None)
```

## Production Deployment

Production deployment follows the same steps but uses the production network:

```bash
soroban config network add mainnet \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

Then deploy with `--network mainnet` instead of `--network testnet`.

### Pre-Production Checklist

Before deploying to mainnet:

- [ ] Contract passes all integration tests
- [ ] Security review completed and approved
- [ ] Testnet deployment verified for at least 7 days
- [ ] Initial admins and issuers prepared
- [ ] RPC endpoints configured and tested
- [ ] Monitoring and alerting set up
- [ ] Emergency procedures documented
- [ ] Rollback plan prepared
- [ ] Change log entry created
- [ ] Team sign-off obtained

## Contract Management

### Add Administrator

```bash
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- add_admin \
  --caller "$ADMIN_KEYPAIR" \
  --new_admin "GNEW_ADMIN_ADDRESS"
```

### Register Issuer

```bash
ISSUER_ADDRESS="GISSUER_ADDRESS"
ASSET_ADDRESS="GASSET_ADDRESS"
ATTESTOR_1="GATTEST1"

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- register_issuer \
  --caller "$ADMIN_KEYPAIR" \
  --issuer "$ISSUER_ADDRESS" \
  --name "MyIssuer" \
  --asset "$ASSET_ADDRESS" \
  --attestation_window_seconds 86400 \
  --required_attestors "[\"$ATTESTOR_1\"]" \
  --min_signers 1
```

### Check Issuer Status

```bash
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- get_issuer \
  --issuer "$ISSUER_ADDRESS"
```

## Monitoring & Maintenance

### Contract Events

Monitor these events via event indexer:

```bash
# Subscribe to events for a contract
soroban events \
  --id "$CONTRACT_ID" \
  --network testnet
```

Key events to monitor:
- `EVENT_ISSUER_REGISTERED` — New issuers added
- `EVENT_ISSUER_FLAGGED_STALE` — Stale attestations detected
- `EVENT_ATTESTATION_FINALIZED` — Attestation lifecycle

### Performance Monitoring

1. **RPC Latency**: Monitor soroban-rpc response times
2. **Contract Storage**: Track instance storage usage (limit: 1MB)
3. **Transaction Throughput**: Monitor attestation submission rate

### Upgrade Strategy

The contract is not upgradeable on mainnet. For updates:
1. Deploy new contract instance
2. Update SDK to point to new contract ID
3. Migrate state if needed (manual process)
4. Transition issuers to new contract

## Troubleshooting

### Issue: "Contract not found"
- Verify CONTRACT_ID is correct
- Verify network is correct (`soroban config network ls`)
- Contract may not be finalized yet (wait 1-2 blocks)

### Issue: "Caller is not an admin"
- Verify caller identity: `soroban config identity show deployer`
- Verify admin was initialized during deployment

### Issue: "Issuer not found"
- Verify issuer was registered: `get_issuer <address>`
- Verify on correct network (testnet vs mainnet)

### Issue: High transaction fees
- Normal for contract interactions (~100-1000 stroops)
- Compare against baseline transaction (transfer) to identify anomalies

## Security Considerations

1. **Key Management**
   - Never commit private keys to git
   - Use environment variables: `SOROBAN_SECRET_KEY`
   - Rotate deployer keys after mainnet deployment

2. **RPC Endpoint**
   - Use official Stellar RPC endpoints
   - Don't use public RPC in production for sensitive operations
   - Consider running your own Soroban RPC instance

3. **Access Control**
   - Distribute admin keys to multiple signers
   - Use threshold multisig for mainnet operations
   - Document all admin operations

## Support

For issues, questions, or security concerns:
- GitHub Issues: https://github.com/reserveproof/reserveproof-contracts/issues
- Security Email: security@reserveproof.dev
- Documentation: https://docs.reserveproof.dev
