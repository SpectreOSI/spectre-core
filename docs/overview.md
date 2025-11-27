# Spectre Core Overview

Spectre Core provides the on-chain primitives for Spectre, a private payments and routing layer on Solana.

The program focuses on two responsibilities:

- **Shadow Accounts** – per-user PDAs that hold routing state and metadata
- **Route Commitments** – hashes that describe private payment paths, verifiable off-chain

## Shadow Accounts

Shadow accounts are program-derived accounts keyed by the user’s public key:

- Seed: `["shadow", owner]`
- Fields:
  - `owner` – public key that controls the shadow account
  - `routing_tag` – 32 bytes of opaque data used by off-chain relayers
  - `bump` – PDA bump seed

They do not hold funds directly. Instead, they provide a stable anchor for Spectre’s routing layer and external relayers.

## Route Commitments

Route commitments are simple records tying a shadow account to a hash of routing data:

- Seed: `["route", shadow]`
- Fields:
  - `shadow` – associated shadow account
  - `commitment` – `blake3(sha256(payload))` over arbitrary routing payload bytes

The payload is never stored on-chain; only the commitment is. This allows off-chain infrastructure to prove consistency between observed routes and on-chain state without revealing internal details.

## Design Goals

- **Low surface area** – minimal set of instructions and accounts
- **Relayer-friendly** – compatible with off-chain routing engines and agents
- **Auditable** – clear, explicit hashing strategy for commitments
- **Composable** – intended to be embedded into higher-level payment flows and vault systems
