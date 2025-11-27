# Integrating Spectre with Your dApp

This guide describes a minimal pattern for wiring Spectre into an existing Solana dApp or wallet flow.

## When to Use Spectre

Spectre is intended for flows where you:

- Want to separate user identity from routing metadata
- Need a stable account to anchor private routes
- Prefer to keep detailed path descriptions off-chain, but auditable

Common examples:

- Private payment routers
- Agent-driven trading flows
- Multi-hop settlement paths

## 1. Initialize a Shadow Account

From your backend or client, derive and initialize the shadow account:

```ts
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SpectreClient } from "../packages/sdk/src";

const connection = new Connection("https://api.mainnet-beta.solana.com");
const programId = new PublicKey("SpEcTre11111111111111111111111111111111111");
const client = new SpectreClient({ connection, programId });

const owner = Keypair.generate().publicKey;
const payer = owner; // in practice, this may be a separate fee payer

const shadow = client.deriveShadowAddress(owner);
console.log("shadow address:", shadow.toBase58());
