use soroban_sdk::{Env, Map, String};

use super::Rank;

#[test]
fn test_simple() {
  let env = Env::default();

  let page_a = String::from_slice(&env, "A");
  let page_b = String::from_slice(&env, "B");
  let page_c = String::from_slice(&env, "C");
  let page_d = String::from_slice(&env, "D");

  let mut rank: Rank = Rank::new(&env);

  rank.add_page(
    page_a.clone(),
    Map::from_array(&env, [(page_b.clone(), ()), (page_d.clone(), ())]),
  );

  rank.add_page(
    page_b.clone(),
    Map::from_array(&env, [(page_a.clone(), ())]),
  );

  rank.add_page(
    page_c.clone(),
    Map::from_array(&env, [(page_a.clone(), ()), (page_b.clone(), ())]),
  );

  rank.add_page(
    page_d.clone(),
    Map::from_array(&env, [(page_c.clone(), ())]),
  );

  let ranks = rank.calculate(&env);

  assert!(ranks.get(page_a.clone()).unwrap() == (0, 344));
  assert!(ranks.get(page_b.clone()).unwrap() == (0, 185));
  assert!(ranks.get(page_c.clone()).unwrap() == (0, 339));
  assert!(ranks.get(page_d.clone()).unwrap() == (0, 181));
}
