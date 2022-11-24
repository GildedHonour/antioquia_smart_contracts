#!/bin/sh

./build.sh

echo ">> Deploying contract 'antioquia_donation'"
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/antioquia_donation.wasm