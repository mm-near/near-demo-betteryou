#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy  --wasmFile ./target/wasm32-unknown-unknown/release/wakeup.wasm --account_id language.betteryou.testnet