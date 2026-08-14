# Contributing to ReserveProof Contracts

## Local Development Setup

### Prerequisites
- Rust 1.84+ with `wasm32v1-none` target
- Stellar CLI (https://developers.stellar.org/docs/build/smart-contracts/getting-started)

### Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

### Testing

```bash
cargo test
```

To run tests against a local Soroban sandbox:

1. Start the Soroban test environment (follow the Stellar docs)
2. Run integration tests (documented in test suite)

## Code Style

- Follow Rust idioms and conventions
- Use `rustfmt` for formatting: `cargo fmt`
- Check lints: `cargo clippy`

## Commits

This project follows Conventional Commits:
- `feat(contracts): add issuer registry`
- `fix(contracts): require min_signers threshold`
- `test(contracts): add staleness-window test`

Each commit should be focused and leave the code in a working, tested state. Push after every commit.

## Pull Requests

- One discrete piece of work per PR
- Reference any related issues
- Include a brief description of what changed and why
- Ensure CI passes (cargo test + cargo build for WASM)

## Security

- No floating point in reserve ratio calculations — use fixed-point (basis points) only
- All public functions must document their auth requirements
- Multi-sig/admin paths require explicit `require_auth()` checks
