#![no_std]

use soroban_sdk::Env;
use types::DecimalNumber;

mod layer;
mod neuron;
mod types;
mod utils;

use crate::layer::Layer;

// #[contract]
pub struct NeuralGovernance<'a> {
  layers: &'a [&'a Layer<'a>],
}

impl NeuralGovernance<'_> {
  pub fn execute(&self, env: &Env) -> DecimalNumber {
    let mut current_layer_result: Option<DecimalNumber> = None;
    if self.layers.is_empty() {
        panic!("no layers detected");
    }
    for layer in self.layers {
      let layer_result = layer.execute(env, current_layer_result);
      current_layer_result = Some((layer.layer_aggregator)(layer_result));
    }
    current_layer_result.expect("current layer result must hold a value (maybe there are no layers defined?)")
  }
}

// #[contractimpl]
// impl NeuralGovernance {
//     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
//         vec![&env, symbol_short!("Hello"), to]
//     }
// }

#[cfg(test)]
mod test;
