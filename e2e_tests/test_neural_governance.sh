#!/bin/bash

set -e

CONFIG_DIR="$(git rev-parse --show-toplevel)/e2e_tests/config.sh"
source $CONFIG_DIR

echo "[*] building contracts"
soroban contract build

echo "[*] deploying contracts"
NEURAL_GOVERNACE_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_neural_governance.wasm --source $SECRET_KEY --rpc-url $RPC_URL --network-passphrase 'Standalone Network ; February 2017')
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

echo "[*] invoking set_layer_aggregator"
invoke_function $LAYER_ID set_layer_aggregator --aggregator SUM

# prepare neurons
echo "[*] invoking set_external_data_provider for ASSIGNED_REPUTATION_NEURON_ID"
invoke_function $ASSIGNED_REPUTATION_NEURON_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

echo "[*] invoking set_external_data_provider for PRIOR_VOTING_HISTORY_NEURON_ID"
invoke_function $PRIOR_VOTING_HISTORY_NEURON_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

# add neurons
echo "[*] invoking add_neuron(s)"
invoke_function $LAYER_ID add_neuron --neuron_address $TEMPLATE_NEURON_ID
invoke_function $LAYER_ID add_neuron --neuron_address $ASSIGNED_REPUTATION_NEURON_ID
invoke_function $LAYER_ID add_neuron --neuron_address $PRIOR_VOTING_HISTORY_NEURON_ID

echo "[*] invoking get_layers"
invoke_function $NEURAL_GOVERNACE_ID get_layers

echo "[*] invoking add_layer"
invoke_function $NEURAL_GOVERNACE_ID add_layer --layer_address $LAYER_ID
# invoke_function $NEURAL_GOVERNACE_ID add_layer --layer_address $LAYER_ID # limit exceeded

echo "[*] invoking get_layers"
invoke_function $NEURAL_GOVERNACE_ID get_layers

echo "[*] invoking execute_neural_governance"
invoke_function $NEURAL_GOVERNACE_ID execute_neural_governance --voter_id user001 --project_id project001
