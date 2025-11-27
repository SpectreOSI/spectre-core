# Contributing to Spectre

We welcome contributions from the community. This document outlines the basic guidelines for contributing to Spectre.

## Code of Conduct

By participating in this project, you agree to uphold the Spectre code of conduct.

## Development Workflow

Spectre uses a monorepo structure, with:

- `programs/` — on-chain Solana programs (Rust)
- `packages/` — client libraries and SDKs (TypeScript)
- `examples/` — minimal usage demos
- `docs/` & `guides/` — documentation

Before submitting a PR:

1. Make sure your branch is rebased on `main`
2. Run Rust and TypeScript builds:
   ```bash
   anchor build
   npm run build
 
