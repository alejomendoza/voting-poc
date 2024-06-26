#![allow(non_upper_case_globals)]

use crate::{
  external_data_provider_contract,
  types::{
    DecimalNumber, LayerAggregator, NeuronType, VotingSystemError, DEFAULT_WEIGHT,
    INITIAL_VOTING_POWER,
  },
  VotingSystem,
};

use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{contracttype, Env, Map, String, Vec};

use crate::layer::Layer;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuralGovernance {
  pub layers: Vec<Layer>,
  pub current_layer_id: u32,
}

impl NeuralGovernance {
  pub fn add_layer(&mut self, env: Env) -> u32 {
    self.layers.push_back(Layer {
      id: self.current_layer_id,
      neurons: Map::new(&env),
      aggregator: LayerAggregator::Unknown,
    });
    let result = self.current_layer_id;
    self.current_layer_id += 1;
    result
  }

  pub fn remove_layer(&mut self, layer_id: u32) -> Result<(), VotingSystemError> {
    let index = self.get_layer_index(layer_id)?;
    self.layers.remove(index);
    Ok(())
  }

  fn get_layer_index(&self, layer_id: u32) -> Result<u32, VotingSystemError> {
    let mut i = 0;
    let mut index = None;
    for layer in self.layers.iter() {
      if layer.id == layer_id {
        index = Some(i);
        break;
      }
      i += 1;
    }
    Ok(index.ok_or(VotingSystemError::NoSuchLayer)?)
  }

  pub fn add_neuron(&mut self, layer_id: u32, neuron: NeuronType) -> Result<(), VotingSystemError> {
    let index = self.get_layer_index(layer_id)?;
    let mut new_layer = self.layers.get(index).unwrap().clone();
    new_layer
      .neurons
      .set(neuron, DecimalNumberWrapper::from(DEFAULT_WEIGHT).as_raw());
    self.layers.remove(index);
    self.layers.insert(index, new_layer.clone());

    Ok(())
  }

  pub fn remove_neuron(
    &mut self,
    layer_id: u32,
    neuron: NeuronType,
  ) -> Result<(), VotingSystemError> {
    let index = self.get_layer_index(layer_id)?;
    let mut new_layer = self.layers.get(index).unwrap().clone();
    new_layer.neurons.remove(neuron);
    self.layers.remove(index);
    self.layers.insert(index, new_layer.clone());

    Ok(())
  }

  pub fn set_layer_aggregator(
    &mut self,
    layer_id: u32,
    aggregator: LayerAggregator,
  ) -> Result<(), VotingSystemError> {
    let index = self.get_layer_index(layer_id)?;
    let mut new_layer = self.layers.get(index).unwrap().clone();
    new_layer.aggregator = aggregator;
    self.layers.remove(index);
    self.layers.insert(index, new_layer.clone());
    Ok(())
  }

  pub fn set_neuron_weight(
    &mut self,
    layer_id: u32,
    neuron: NeuronType,
    weight: DecimalNumber,
  ) -> Result<(), VotingSystemError> {
    let index = self.get_layer_index(layer_id)?;
    let mut new_layer = self.layers.get(index).unwrap().clone();
    new_layer
      .neurons
      .set(neuron, DecimalNumberWrapper::from(weight).as_raw());
    self.layers.remove(index);
    self.layers.insert(index, new_layer.clone());
    Ok(())
  }

  pub fn execute_neural_governance(
    &self,
    env: Env,
    voter_id: String,
    submission_id: String,
  ) -> Result<(u32, u32), VotingSystemError> {
    let mut current_layer_result: (u32, u32) = INITIAL_VOTING_POWER;

    if self.layers.is_empty() {
      return Err(VotingSystemError::NoLayersExist);
    }
    let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_address);
    for layer in self.layers.clone() {
      let layer_result: Vec<(u32, u32)> = layer.execute_layer(
        env.clone(),
        voter_id.clone(),
        submission_id.clone(),
        current_layer_result,
        &external_data_provider_client,
      )?;
      current_layer_result = layer.run_layer_aggregator(layer_result)?;
    }
    Ok(current_layer_result)
  }
}
