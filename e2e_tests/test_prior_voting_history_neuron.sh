#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
EXTERNAL_DATA_PROVIDER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
PRIOR_VOTING_HISTORY_NEURON_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_prior_voting_history_neuron.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  ARGS="${@:3}"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017' -- $FN_NAME $ARGS
}

echo "[*] invoking mock_sample_data"
invoke_function $EXTERNAL_DATA_PROVIDER_ID mock_sample_data

echo "[*] invoking get_external_data_provider"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID get_external_data_provider

echo "[*] invoking set_external_data_provider"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

echo "[*] invoking get_external_data_provider"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID get_external_data_provider

echo "[*] invoking oracle_function"
ORACLE_RESULT=$(invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID oracle_function --voter_id user001 --_project_id project001 --maybe_previous_layer_vote "[4,500]")
echo $ORACLE_RESULT

echo "[*] invoking set_weight"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID set_weight --new_weight "[1,700]"

echo "[*] invoking weight_function"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID weight_function --raw_neuron_vote $ORACLE_RESULT
