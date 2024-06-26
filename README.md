# voting-poc

This is an implementation of a new voting mechanism for [Stellar Community Fund](https://medium.com/r/?url=https%3A%2F%2Fcommunityfund.stellar.org%2F) written in Rust for [Soroban](https://medium.com/r/?url=https%3A%2F%2Fsoroban.stellar.org%2F).

## Architecture

This project is implemented using [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
This workspace consists of the following packages:
- `voting_system`
- `external_data_provider`

They are described below. Both of them are smart contracts.

### Voting System

This exposes the API, if users or admins need to do anything with this system, calls should go through this one. It also contains the logic of the system.

The main component of the system is called Neural Governance. It contains all the elements necessary for evaluating the votes.

The Neural Governance contains any number of Layers (at least one), and every Layer may have any number of Neurons (as well as at least one is required for every Layer).

![architecture](image.png)

Every Neuron has a specific logic of how to calculate the Weight of the vote. The Neurons inside of a Layer are executed separately and the order does not matter. Once all the Neurons are processed, the Layer result is calculated using a Layer Aggregator, which is set for every Layer. It may add all the Neurons' results, multiply them, or do any other operation that just takes a sequence of results and outputs one single number which is the Layer's result.

All the Layers are executed sequentially, the order matters in this case as the result of one Layer affects how the Neurons in the next one are evaluated. The result of the last calculated Layer is treated as a result of the Neural Governance.

Neural Governance is executed for every vote calculating its weight. After that, the votes' weights are summed up resulting in a voting power for every submission. This happens at the end of every voting round.

#### Setting up

There are two use cases of the Voting System:
- receiving votes
- tallying votes

If you just want to receive the votes, you just have to deploy the Voting System contract.

In order to set up the Voting System for tallying the votes, you need to do the following things:
- deploy the Voting System contract
- call the `initialize` function which creates a new Neural Governance object and puts it in the storage
- add a layer and set a layer aggregator for it
- add a neuron to that layer

With such a setup, you should be able to tally the votes. You can of course add any number of layers and neurons.

You can also define your own neurons, deploy the Voting System contract, and add them where you want.

Neurons that already exist in the system are described in [this doc](./docs/neurons.md).

#### Custom Neurons

The system is open for any number of Neurons. They can be easily added and used. In order to add a new Neuron, you need to:
- add a new file to [the neurons folder](./src/voting_system/src/neurons/) - the new Neuron has to have `oracle_function`, the easiest way to go is to just copy the contents of the [dummy neuron](./src/voting_system/src/neurons/dummy_neuron.rs)
- fill the `oracle_function` of the new neuron with your custom logic
- add your Neuron module to the [mod file](./src/voting_system/src/neurons/mod.rs)
- in [types](./src/voting_system/src/types.rs)
  - add a field of your Neuron to the `NeuronType` enum
  - add a case of your Neuron to the `neuron_type_from_str` function
- add a case for your Neuron in the `execute_layer` function in [the layer file](./src/voting_system/src/layer.rs)

And That's it, you can now add the new Neuron to any Layer and start using it.

You can check how it is done on [this branch](https://github.com/alejomendoza/voting-poc/tree/new-neuron).

### External Data Provider

This is a contract that may be used by the voting system to fetch the data from wherever outside of this system. The data is kept in the storage and can be set by an admin/anyone with proper access.

### Example of how it works

A good example of how to properly prepare the whole infrastructure to work can be found in the [voting system test](./src/voting_system/src/voting_system_test.rs).

## Neuron template

There is a neuron template in `src/voting_system/src/neurons/dummy_neuron.rs`. In order to add a new neuron, please do the following things:
- copy-paste the template neuron file
- rename this file from template neuron to `[your_neuron_name]`
- implement your neuron's logic

## Useful commands:
- `soroban contract build`
- `cargo test -- --nocapture` - `nocapture` is optional and it will show you the logs. Here's an example of how to properly log in tests:
```
let env = Env::default();
log!(&env, "hello");
env.logs().print();
```

Please note that sometimes you will need to build the contracts explicitly before testing because tests import `.wasm` files (`soroban_sdk::contractimport`).

## Useful scripts

To run e2e tests on localhost using docker, run:
```
./scripts/restart_docker.sh && ./scripts/run.sh
```
