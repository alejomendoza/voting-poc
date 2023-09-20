use soroban_sdk::{Env, Map, String};

use crate::{decimal_number_wrapper::DecimalNumberWrapper, types::DecimalNumber};

pub struct Rank {
  graph: Map<String, Map<String, ()>>,
}

impl Rank {
  pub fn new(env: &Env) -> Rank {
    Rank {
      graph: Map::new(env),
    }
  }

  pub fn from_pages(env: &Env, pages: Map<String, Map<String, ()>>) -> Rank {
    let mut result = Rank::new(env);
    for (page, links) in pages {
      result.add_page(page, links);
    }
    result
  }

  pub fn add_page(&mut self, page: String, links: Map<String, ()>) {
    self.graph.set(page, links);
  }

  pub fn calculate(&self, env: &Env) -> Map<String, DecimalNumber> {
    // Initialize the PageRank values
    let initial_rank: DecimalNumber = DecimalNumberWrapper::div(
      DecimalNumberWrapper::from((1, 0)),
      DecimalNumberWrapper::from((self.graph.len(), 0)),
    )
    .as_tuple();
    let mut page_ranks: Map<String, DecimalNumber> = Map::new(&env);
    for page in self.graph.keys() {
      page_ranks.set(page, initial_rank);
    }

    // Perform PageRank iterations
    let num_iterations = 10;
    let damping_factor = DecimalNumberWrapper::from((0, 850)).as_tuple();

    for _ in 0..num_iterations {
      let mut new_ranks: Map<String, DecimalNumber> = Map::new(&env);
      let total_pages = self.graph.len();
      // let dead_end_rank = DecimalNumberWrapper::new(0, 0);

      // Calculate PageRank for each page
      for (page, links) in self.graph.clone() {
        let mut rank = DecimalNumberWrapper::sub(
          DecimalNumberWrapper::from((1, 0)),
          DecimalNumberWrapper::from(damping_factor),
        );
        rank = DecimalNumberWrapper::div(rank, DecimalNumberWrapper::from((total_pages, 0)));

        for link in links.keys() {
          let link_count =
            DecimalNumberWrapper::from((self.graph.get(link.clone()).unwrap().len(), 0));
          if link_count.as_raw() == 0 {
            // dead_end_rank += damping_factor * page_ranks[page];
          } else {
            // rank += damping_factor * page_ranks[link] / link_count;
            let mut modifier = DecimalNumberWrapper::mul(
              DecimalNumberWrapper::from(damping_factor),
              DecimalNumberWrapper::from(page_ranks.get(link.clone()).unwrap()),
            );
            modifier = DecimalNumberWrapper::div(modifier, link_count);
            rank = DecimalNumberWrapper::add(rank, modifier);
          }
        }

        new_ranks.set(page, rank.as_tuple());
      }

      // Calculate PageRank for dead-end pages and distribute it equally
      // let redistributed_dead_end_rank = DecimalNumberWrapper::div(dead_end_rank, DecimalNumberWrapper::from((total_pages, 0))).as_tuple();

      // Update PageRank values
      // for (page, rank) in new_ranks {
      //   // *rank += redistributed_dead_end_rank;
      //   let new_rank = DecimalNumberWrapper::add(
      //     DecimalNumberWrapper::from(rank),
      //     DecimalNumberWrapper::from(redistributed_dead_end_rank),
      //   );
      //   new_ranks.set(page, new_rank.as_tuple());
      // }

      page_ranks = new_ranks;
    }
    page_ranks
  }
}

#[cfg(test)]
mod page_rank_test;
