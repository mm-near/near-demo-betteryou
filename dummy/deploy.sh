#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy  --wasmFile ./target/wasm32-unknown-unknown/release/dummy.wasm --accountId dummy.betteryou.testnet