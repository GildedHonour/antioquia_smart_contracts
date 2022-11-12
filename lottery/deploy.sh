#!/bin/sh

./build.sh

echo ">> Deploying contract 'antioquia_lottery'"
near deploy --accountId lottery.antioquia.testnet --wasmFile ./target/wasm32-unknown-unknown/release/antioquia_lottery.wasm