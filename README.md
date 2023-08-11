# voting-poc

## Neuron template

There is a neuron template in `src/template_neuron`, you can just copy-paste the entire folder, change all the names and implement custom logic. Also, you need to add its entry to the root's  `Cargo.toml`.

## Useful commands:
- `soroban contract build`
- `cargo test -- --nocapture` - `nocapture` is optional and it will show you the logs. In order to properly log, please use `log!` macro, plus print all logs at the end of a test (`env.logs().print();`).

Please note that sometimes you will need to build the contracts explicitly before testing because tests import `.wasm` files (`soroban_sdk::contractimport`).

## Architecture

The architecture of the system is described [here](./docs/architecture.md)
