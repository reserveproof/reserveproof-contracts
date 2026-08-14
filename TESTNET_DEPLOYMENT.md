# Testnet Deployment Record

## Current Deployment

- **Contract ID**: `CD2YPKQI64LUQJAVENRW7OLTD57AXBNQ7ANVA673HQNWVP5ZRDUZLIG6`
- **Network**: Stellar Testnet
- **RPC URL**: `https://soroban-testnet.stellar.org`
- **Network Passphrase**: `Test SDF Network ; September 2015`
- **Deployer / Admin**: `GC2IQ4B6EWQFQUY2COZRR5O34B6JJUMET6NPS53LAOZ22T4HHQ3BLJJR`
- **Deployed**: 2026-08-14
- **Deploy tx**: https://stellar.expert/explorer/testnet/tx/2fa7ce8a60bb7d759b93bb2bad4b12a6107a70cab38da6ae1b89a78675186395
- **Init tx**: https://stellar.expert/explorer/testnet/tx/87e128d9046d13cd0789e867301158f49cd42bd981ff450e828b54eb5a322f22
- **Explorer**: https://lab.stellar.org/r/testnet/contract/CD2YPKQI64LUQJAVENRW7OLTD57AXBNQ7ANVA673HQNWVP5ZRDUZLIG6

## Verification

```bash
stellar contract invoke \
  --id CD2YPKQI64LUQJAVENRW7OLTD57AXBNQ7ANVA673HQNWVP5ZRDUZLIG6 \
  --source deployer \
  --network testnet \
  -- get_issuer \
  --issuer GC2IQ4B6EWQFQUY2COZRR5O34B6JJUMET6NPS53LAOZ22T4HHQ3BLJJR
# -> null (contract live, no issuers registered yet)
```

## Notes

This is a testnet deployment for development/staging use. The deployer keypair's
secret key is held locally by whoever ran the deployment and is **not** committed
to this repository. Before mainnet deployment, follow the full checklist in
[DEPLOYMENT.md](./DEPLOYMENT.md).
