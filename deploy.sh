#!/usr/bin/env bash

WASM_PATH="$(find ./target/wasm32-unknown-unknown/release/ -maxdepth 1 -name "*.wasm")"

near deploy \
  --wasmFile "$WASM_PATH" \
  --accountId "$1" \
  --initFunction new \
  --initArgs '{"owner_id": "testaccount15.testnet"}'