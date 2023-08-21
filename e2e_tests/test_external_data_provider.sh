#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
EXTERNAL_DATA_PROVIDER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  ARGS="${@:3}"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017' -- $FN_NAME $ARGS
}

echo "[*] invoking mock_sample_data"
invoke_function $EXTERNAL_DATA_PROVIDER_ID mock_sample_data

echo "[*] invoking get_user_reputation_category"
invoke_function $EXTERNAL_DATA_PROVIDER_ID get_user_reputation_category --user_id user001

echo "[*] invoking get_reputation_score"
invoke_function $EXTERNAL_DATA_PROVIDER_ID get_reputation_score --reputation_category 3

echo "[*] invoking get_user_prior_voting_history"
invoke_function $EXTERNAL_DATA_PROVIDER_ID get_user_prior_voting_history --user_id user001

echo "[*] invoking get_round_bonus_map"
invoke_function $EXTERNAL_DATA_PROVIDER_ID get_round_bonus_map
