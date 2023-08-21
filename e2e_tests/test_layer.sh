#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
LAYER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_layer.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
EXTERNAL_DATA_PROVIDER_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
TEMPLATE_NEURON_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_template_neuron.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
ASSIGNED_REPUTATION_NEURON_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_assigned_reputation_neuron.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
PRIOR_VOTING_HISTORY_NEURON_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_prior_voting_history_neuron.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')

function invoke_function {
  CONTRACT_ID="$1"
  FN_NAME="$2"
  ARGS="${@:3}"
  soroban contract invoke --id $CONTRACT_ID --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017' -- $FN_NAME $ARGS
}

echo "[*] invoking get_layer_aggregator"
invoke_function $LAYER_ID get_layer_aggregator

echo "[*] invoking set_layer_aggregator"
invoke_function $LAYER_ID set_layer_aggregator --aggregator SUM

echo "[*] invoking get_layer_aggregator"
invoke_function $LAYER_ID get_layer_aggregator

echo "[*] invoking get_neurons"
invoke_function $LAYER_ID get_neurons

# prepare neurons
echo "[*] invoking set_external_data_provider for ASSIGNED_REPUTATION_NEURON"
invoke_function $ASSIGNED_REPUTATION_NEURON_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

echo "[*] invoking set_external_data_provider for PRIOR_VOTING_HISTORY_NEURON"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

# add neurons
echo "[*] invoking add_neuron(s)"
invoke_function $LAYER_ID add_neuron --neuron_address $TEMPLATE_NEURON_ID
invoke_function $LAYER_ID add_neuron --neuron_address $ASSIGNED_REPUTATION_NEURON_ID
invoke_function $LAYER_ID add_neuron --neuron_address $PRIOR_VOTING_HISTORY_NEURON_ID

echo "[*] invoking get_neurons"
invoke_function $LAYER_ID get_neurons

echo "[*] invoking execute_layer"
VOTES=$(invoke_function $LAYER_ID execute_layer --voter_id user001 --project_id project001 --previous_layer_vote "[3,650]")
echo $VOTES

echo "[*] invoking run_layer_aggregator"
invoke_function $LAYER_ID run_layer_aggregator --neuron_votes $VOTES

