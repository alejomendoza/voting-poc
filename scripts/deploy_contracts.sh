#!/bin/bash

set -e

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

NETWORK_PASSPHRASE=$(cat $CURRENT_DIR/.config/network_passphrase)
RPC_URL=$(cat $CURRENT_DIR/.config/rpc_url)
SECRET_KEY=$(cat $CURRENT_DIR/.config/secret_key)

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
VOTING_SYSTEM_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_system.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase "$NETWORK_PASSPHRASE")
EXTERNAL_DATA_PROVIDER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase "$NETWORK_PASSPHRASE")

rm -rf $CURRENT_DIR/.contracts || true
mkdir $CURRENT_DIR/.contracts

echo $VOTING_SYSTEM_ID > $CURRENT_DIR/.contracts/voting_system_id
echo $EXTERNAL_DATA_PROVIDER_ID > $CURRENT_DIR/.contracts/external_data_provider_id

echo "[*] done"
