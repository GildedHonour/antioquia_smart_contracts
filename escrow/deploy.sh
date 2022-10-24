#!/bin/sh

./build.sh

echo ">> Deploying contract 'antioquia_escrow'"
near deploy --accountId escrow.antioquia.testnet --wasmFile ./target/wasm32-unknown-unknown/release/antioquia_escrow.wasm