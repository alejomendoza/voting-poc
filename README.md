# voting-poc

## Neuron template

There is a neuron template in `src/template_neuron`. In order to add new neuron, please do the following things:
- copy-paste the template neuron folder
- rename the folder and all the names inside it (from template neuron to [your_neuron_name])
- add its entry (workspace member) to the `Cargo.toml` in the root folder of the project
- implement your neuron's logic

## Useful commands:
- `soroban contract build`
- `cargo test -- --nocapture` - `nocapture` is optional and it will show you the logs. In order to properly log, please use `log!` macro, plus print all logs at the end of a test (`env.logs().print();`).

Please note that sometimes you will need to build the contracts explicitly before testing because tests import `.wasm` files (`soroban_sdk::contractimport`).

## Architecture

The architecture of the system is described [here](./docs/architecture.md)
