use soroban_sdk::{Env, String};

use crate::{layer::Layer, utils::decimal_number_persist::DecimalNumberPersist, NeuralGovernance};

#[test]
fn test_simple() {
  let env = Env::default();
  let layers = &[
    &Layer {
      neurons: &[
          // &Neuron {  }
        ],
      layer_aggregator: &|neuron_votes| {
        neuron_votes
          .iter()
          .reduce(|acc, item| {
            let result = DecimalNumberPersist::from(acc).to_float()
              + DecimalNumberPersist::from(item).to_float();
            DecimalNumberPersist::from(result).as_tuple()
          })
          .expect("failed to reduce neuron votes")
      },
    },
    &Layer {
      neurons: &[],
      layer_aggregator: &|neuron_votes| {
        neuron_votes
          .iter()
          .reduce(|acc, item| {
            let result = DecimalNumberPersist::from(acc).to_float()
              * DecimalNumberPersist::from(item).to_float();
            DecimalNumberPersist::from(result).as_tuple()
          })
          .expect("failed to reduce neuron votes")
      },
    },
  ];
  let neural_governance = NeuralGovernance { layers: layers };
  let result = neural_governance.execute(&env);
  
}
