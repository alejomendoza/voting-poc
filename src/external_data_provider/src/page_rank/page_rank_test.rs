use soroban_sdk::{Env, Map, String};

use super::Rank;

#[test]
fn test_simple() {
  let env = Env::default();
  env.budget().reset_unlimited();
  /*
    A->B
    A->C
    B->A
    C->A
    C->B
    D->A

    A - trusted by 3 users
    B - trusted by 2 users
    C - trusted by 1 users
    D - trusted by 0 users
    E - trusted by 0 users and trusts noone
  */

  let page_a = String::from_slice(&env, "A");
  let page_b = String::from_slice(&env, "B");
  let page_c = String::from_slice(&env, "C");
  let page_d = String::from_slice(&env, "D");
  let page_e = String::from_slice(&env, "E");

  let mut page_rank: Rank = Rank::new(&env);

  page_rank.add_page(
    &env,
    page_a.clone(),
    Map::from_array(&env, [(page_b.clone(), ()), (page_c.clone(), ())]),
  );

  page_rank.add_page(
    &env,
    page_b.clone(),
    Map::from_array(&env, [(page_a.clone(), ())]),
  );

  page_rank.add_page(
    &env,
    page_c.clone(),
    Map::from_array(&env, [(page_a.clone(), ()), (page_b.clone(), ())]),
  );

  page_rank.add_page(
    &env,
    page_d.clone(),
    Map::from_array(&env, [(page_a.clone(), ())]),
  );

  let ranks = page_rank.calculate(&env);

  assert!(ranks.get(page_a.clone()).unwrap() == (0, 415));
  assert!(ranks.get(page_b.clone()).unwrap() == (0, 303));
  assert!(ranks.get(page_c.clone()).unwrap() == (0, 213));
  assert!(ranks.get(page_d.clone()).unwrap() == (0, 37));

  page_rank.add_page(&env, page_e.clone(), Map::from_array(&env, []));

  let ranks = page_rank.calculate(&env);

  assert!(ranks.get(page_a.clone()).unwrap() == (0, 337));
  assert!(ranks.get(page_b.clone()).unwrap() == (0, 246));
  assert!(ranks.get(page_c.clone()).unwrap() == (0, 173));
  assert!(ranks.get(page_d.clone()).unwrap() == (0, 30));
  assert!(ranks.get(page_e.clone()).unwrap() == (0, 30));

  // values above might change if you modify the algorithm but the stuff below should always remain the same
  assert!(ranks.get(page_a.clone()).unwrap() > ranks.get(page_b.clone()).unwrap());
  assert!(ranks.get(page_b.clone()).unwrap() > ranks.get(page_c.clone()).unwrap());
  assert!(ranks.get(page_c.clone()).unwrap() > ranks.get(page_d.clone()).unwrap());
}
