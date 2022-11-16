#!/bin/sh

./build.sh

echo ">> Deploying contract 'antioquia_lottery'"
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/antioquia_lottery.wasm