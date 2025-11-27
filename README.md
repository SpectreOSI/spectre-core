# Spectre Relayer

Spectre Relayer is a small TypeScript service that forwards Solana RPC
requests while applying privacy filters.

## Features

- Strips non-essential headers and tracking metadata
- Simple policy engine for rate limiting and IP rotation
- Pluggable transports (HTTP, WebSocket, custom providers)

> Status: experimental. Not production-ready.

## Quick start

```bash
pnpm install
pnpm dev
The default configuration points to https://api.mainnet-beta.solana.com.
Override via environment variables:

bash
Copy code
RPC_URL=https://your.rpc.url pnpm dev
License
MIT 
