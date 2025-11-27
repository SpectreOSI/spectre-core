#!/usr/bin/env bash
set -euo pipefail

# Simple helper script to build and deploy Spectre Core to a local validator.

PROGRAM_DIR="programs/spectre_core"
NETWORK_URL="http://localhost:8899"
PROGRAM_ID=${PROGRAM_ID:-"SpEcTre11111111111111111111111111111111111"}

echo "[spectre] starting local validator (ctrl+c to stop)"
solana-test-validator --reset --quiet &
VALIDATOR_PID=$!

sleep 5

echo "[spectre] building program..."
cd "$PROGRAM_DIR"
anchor build

SO_PATH="target/deploy/spectre_core.so"

echo "[spectre] deploying spectre_core to $NETWORK_URL with program id $PROGRAM_ID"
solana program deploy \
  -u "$NETWORK_URL" \
  "$SO_PATH" \
  --program-id "$PROGRAM_ID"

echo "[spectre] deployment complete."

kill "$VALIDATOR_PID" || true
