#!/bin/sh

./build.sh

echo ">> Deploying contract 'antioquia_donation'"
near deploy --accountId donation.antioquia.testnet --wasmFile ./target/wasm32-unknown-unknown/release/antioquia_donation.wasm