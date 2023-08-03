use crate::types::{DecimalNumber, ProjectUUID, UserUUID};
use soroban_sdk::Env;

pub mod trust_graph_neuron;

pub trait Neuron {
  fn oracle_function(
    &self,
    env: &Env,
    voter_id: UserUUID,
    project_id: ProjectUUID,
    previous_layer_vote: &Option<DecimalNumber>,
  ) -> DecimalNumber;
  fn weight_function(&self, env: &Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber;
}
