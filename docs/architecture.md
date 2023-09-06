# Architecture

## Intro

This project is implemented using [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
This workspace consists of the following packages:
- `voting_system`
- `external_data_provider`

They are described below. Both of them are smart contracts.

## Implementation notes

### Storage

For now, the `instance` option is used everywhere, but it is left as a `TODO` to do research about this once again and choose a proper way of using the storage; useful links:
- https://soroban.stellar.org/docs/fundamentals-and-concepts/persisting-data

## Packages

### Voting System

`#[smart_contract]`

This is a package that will expose the API, if users need to do anything with this system, calls should go through this one. It also contains logic of the system.

### External Data Provider

`#[smart_contract]`

This is a contract that may be used by other contracts that require fetching the data from wherever outside of this system. The data is kept in the storage and can be set by an admin/anyone with a proper access.

## Example how it works

A good example of how to properly prepare the whole infrastructure to work can be found in the [voting system test](../src/voting_system/src/voting_system_test.rs).
