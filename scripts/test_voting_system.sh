#!/bin/bash

set -e

CURRENT_DIR=$(git rev-parse --show-toplevel)/scripts

VOTING_SYSTEM_ID=$(cat $CURRENT_DIR/.contracts/voting_system_id)
EXTERNAL_DATA_PROVIDER_ID=$(cat $CURRENT_DIR/.contracts/external_data_provider_id)

source $(git rev-parse --show-toplevel)/scripts/invoke_function.sh

echo "[*] invoking initialize"
invoke_function $VOTING_SYSTEM_ID initialize

echo "[*] invoking add_layer"
invoke_function $VOTING_SYSTEM_ID add_layer
echo "[*] invoking set_layer_aggregator"
invoke_function $VOTING_SYSTEM_ID set_layer_aggregator --layer_id 0 --aggregator Sum

echo "[*] invoking add_layer"
invoke_function $VOTING_SYSTEM_ID add_layer
echo "[*] invoking set_layer_aggregator"
invoke_function $VOTING_SYSTEM_ID set_layer_aggregator --layer_id 1 --aggregator Product

echo "[*] invoking add_neuron: Dummy"
invoke_function $VOTING_SYSTEM_ID add_neuron --layer_id 0 --neuron Dummy
echo "[*] invoking add_neuron: AssignedReputation"
invoke_function $VOTING_SYSTEM_ID add_neuron --layer_id 0 --neuron AssignedReputation
echo "[*] invoking add_neuron: PriorVotingHistory"
invoke_function $VOTING_SYSTEM_ID add_neuron --layer_id 0 --neuron PriorVotingHistory

echo "[*] invoking add_neuron: Dummy"
invoke_function $VOTING_SYSTEM_ID add_neuron --layer_id 1 --neuron Dummy

echo "[*] invoking get_neural_governance"
invoke_function $VOTING_SYSTEM_ID get_neural_governance

echo "[*] invoking add_project"
invoke_function $VOTING_SYSTEM_ID add_project --project_id project001

echo "[*] invoking vote user001 project001 Yes"
invoke_function $VOTING_SYSTEM_ID vote --voter_id user001 --project_id project001 --vote Yes

# external data provider has to be added and immediately used because we use temporary storage there
echo "[*] invoking mock_sample_data"
invoke_function $EXTERNAL_DATA_PROVIDER_ID mock_sample_data

echo "[*] invoking set_external_data_provider"
invoke_function $VOTING_SYSTEM_ID set_external_data_provider --external_data_provider_address $EXTERNAL_DATA_PROVIDER_ID

echo "[*] invoking tally"
invoke_function $VOTING_SYSTEM_ID tally
