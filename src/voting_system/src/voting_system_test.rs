use soroban_sdk::Env;

#[test]
pub fn test() {
  let env = Env::default();
  env.budget().reset_unlimited();

  // ...
}
