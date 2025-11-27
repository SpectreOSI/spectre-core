# Routing & Privacy Model

Spectre does not attempt to fully hide all transaction details on-chain. Instead, it reduces linkability by separating three concerns:

1. **User identity (wallet keys)**
2. **Routing state (shadow accounts)**
3. **Path description (off-chain payloads + on-chain commitments)**

This model is designed to work alongside existing Solana wallets, RPC providers, and off-chain relayers.

---

## Shadow Initialization

A client first initializes a shadow account for a given owner:

- Instruction: `init_shadow_account`
- Accounts:
  - `shadow_account` (PDA) – `["shadow", owner]`
  - `owner` – user public key (not required to sign)
  - `payer` – fee payer
  - `system_program`

This creates a minimal account whose sole job is to anchor future routing state.

---

## Updating the Routing Tag

The `set_routing_tag` instruction allows the owner to rotate opaque routing metadata:

- Only the `owner` signer may update the tag
- No assumptions are made about the contents of the tag
- Off-chain systems are free to interpret this field as:
  - relay group identifiers
  - circuit references
  - policy hashes
  - or any other scheme

Because the tag is just bytes, the on-chain program remains simple and upgrade-safe.

---

## Route Commitments

The `commit_route` instruction stores a commitment for a route payload:

1. Client prepares a `payload` off-chain (e.g. encoded path, fees, or constraints)
2. Program computes:
   - `sha256(payload)`
   - `blake3(sha256(payload))`
3. The resulting hash is stored in a `RouteCommitment` account derived from:
   - Seeds: `["route", shadow]`

This two-step hashing pattern is chosen to be explicit and deterministic, while still compatible with existing tooling.

---

## Integration Pattern

A typical integration flow:

1. Wallet or agent initializes a shadow account
2. Off-chain routing engine constructs a route payload
3. Client submits `commit_route` with the payload bytes
4. Off-chain components reference the on-chain commitment when executing actual transfers through other programs (DEXes, vaults, etc.)

Spectre Core does not hold user funds or enforce economic policy. It is intended to be a thin, auditable layer that other protocols and relayers can build on top of.
