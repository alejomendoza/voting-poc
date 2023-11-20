use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{Env, Map, String, Vec};

use crate::types::DecimalNumber;

pub struct Rank {
  nodes: Vec<String>,
  edges: Map<String, Vec<String>>,
}

impl Rank {
  pub fn new(env: &Env) -> Self {
    Rank {
      nodes: Vec::new(&env),
      edges: Map::new(&env),
    }
  }

  pub fn from_pages(env: &Env, pages: Map<String, Map<String, ()>>) -> Rank {
    let mut result = Rank::new(&env);
    for (page, links) in pages {
      result.add_page(env, page, links);
    }
    result
  }

  // todo optimize this
  pub fn add_page(&mut self, env: &Env, page: String, links: Map<String, ()>) {
    let mut page_edges = self.edges.get(page.clone()).unwrap_or(Vec::new(&env));
    if !&self.nodes.contains(page.clone()) {
      self.nodes.push_back(page.clone());
    }
    for (link, _) in links {
      if !page_edges.contains(link.clone()) {
        page_edges.push_back(link.clone());
      }
      if !&self.nodes.contains(link.clone()) {
        self.nodes.push_back(link.clone());
      }
    }
    self.edges.set(page.clone(), page_edges);
  }

  pub fn calculate(&self, env: &Env) -> Map<String, DecimalNumber> {
    self.calculate_custom_params(&env, 1000, (0, 850))
  }

  pub fn calculate_custom_params(
    &self,
    env: &Env,
    iterations: u32,
    damping_factor: DecimalNumber,
  ) -> Map<String, DecimalNumber> {
    let mut page_ranks: Map<String, DecimalNumber> = Map::new(env);
    for node in self.nodes.clone() {
      page_ranks.set(
        node,
        DecimalNumberWrapper::div(
          DecimalNumberWrapper::from("1.0"),
          DecimalNumberWrapper::from((self.nodes.len(), 0)),
        )
        .as_tuple(),
      );
    }

    for _ in 0..iterations {
      let mut new_ranks: Map<String, DecimalNumber> = Map::new(env);
      let nodes = self.nodes.clone();
      let edges = self.edges.clone();
      for node in nodes {
        let sub_result = DecimalNumberWrapper::sub(
          DecimalNumberWrapper::from("1.0"),
          DecimalNumberWrapper::from(damping_factor),
        );
        let div_result = DecimalNumberWrapper::div(
          sub_result,
          DecimalNumberWrapper::from((self.nodes.len(), 0)),
        );
        let mut rank = div_result;

        for (other_node, edges_item) in edges.clone() {
          if edges_item.contains(node.clone()) {
            let edge_count: DecimalNumber = (edges_item.len(), 0);
            let mul_result = DecimalNumberWrapper::mul(
              DecimalNumberWrapper::from(damping_factor),
              DecimalNumberWrapper::from(page_ranks.get(other_node).unwrap_or((0, 0))),
            );
            let div_result =
              DecimalNumberWrapper::div(mul_result, DecimalNumberWrapper::from(edge_count));
            rank = DecimalNumberWrapper::add(rank, div_result);
          }
        }

        new_ranks.set(node.clone(), rank.as_tuple());
      }
      page_ranks = new_ranks;
    }

    page_ranks
  }
}

#[cfg(test)]
mod page_rank_test;
