# voting-poc

## useful commands:
- `soroban contract build`
- `cargo test -- --nocapture` - `nocapture` is optional and it will show you the logs. In order to properly log, please use `log!` macro, plus print all logs at the end of a test (`env.logs().print();`).

Please note that sometimes you will need to build the contracts explicitly before testing, because tests import `.wasm` files (`soroban_sdk::contractimport`).
