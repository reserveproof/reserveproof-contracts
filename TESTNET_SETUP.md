# Testnet Setup for ReserveProof

This guide covers setting up testnet accounts for development and testing of ReserveProof.

## Prerequisites

- Stellar CLI installed
- Access to Stellar Testnet RPC endpoint

## Account Setup

### 1. Create Test Accounts

Create accounts for:
- Admin account (contract deployer)
- Issuer 1 (e.g., USDC anchor)
- Issuer 2 (e.g., EURC anchor)
- Attestor 1 (e.g., bank/custodian for issuer 1)
- Attestor 2 (e.g., auditor for issuer 1)
- Attestor 3 (e.g., bank/custodian for issuer 2)

Use the following to generate a keypair:

```bash
stellar keys generate --testnet my_keypair
```

Each keypair should be saved securely. Export the public key and note it down.

### 2. Fund Accounts

Use the Stellar Testnet Friendbot to fund accounts:

```bash
curl "https://friendbot.stellar.org?addr=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
```

Replace the address with each account's public key.

### 3. Contract Deployment Account

Designate one account as the contract admin/deployer. This account will:
- Deploy the contract
- Initialize it with itself as the initial admin
- Register issuers and attestors

## Environment Variables

Create a `.env` file in the `reserveproof-contracts` directory:

```
ADMIN_PUBLIC_KEY=GXXXXX...
ADMIN_SECRET_KEY=SXXXXX...
ISSUER_1_PUBLIC_KEY=GXXXXX...
ISSUER_1_SECRET_KEY=SXXXXX...
ISSUER_2_PUBLIC_KEY=GXXXXX...
ISSUER_2_SECRET_KEY=SXXXXX...
ATTESTOR_1_PUBLIC_KEY=GXXXXX...
ATTESTOR_1_SECRET_KEY=SXXXXX...
ATTESTOR_2_PUBLIC_KEY=GXXXXX...
ATTESTOR_2_SECRET_KEY=SXXXXX...
ATTESTOR_3_PUBLIC_KEY=GXXXXX...
ATTESTOR_3_SECRET_KEY=SXXXXX...
```

**IMPORTANT**: Never commit `.env` files with real secrets to git. Use `.env.local` for local development and inject secrets via CI/CD environment variables in production.

## Deployment

See Phase 1 integration tests for deployment examples using the Stellar CLI.

## Resetting Testnet Accounts

If you need to reset an account (e.g., after a failed test), you can create a new keypair and fund it again using Friendbot.
