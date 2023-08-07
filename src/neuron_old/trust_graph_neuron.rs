use graph::prelude::{page_rank, CsrLayout, DirectedCsrGraph, GraphBuilder, PageRankConfig};
use soroban_sdk::{Env, Map, String, Vec, log};

use crate::{
  types::{DecimalNumber, ProjectUUID, UserUUID},
  utils::decimal_number_persist::DecimalNumberPersist,
};

use super::Neuron;

type MappingId = u32;

pub struct TrustGraphNeuron {
  // the format will be a vector of edges, but it's possible to construct it
  // from the format map(uid => list(uids))
  // trust_graph: Vec<(UserUUID, UserUUID)>,
  trust_graph: Option<DirectedCsrGraph<MappingId>>, // TODO make it non optional
  // this mapping is necessary because graph library cannot handle soroban types
  mapping_num_to_uid: Map<MappingId, UserUUID>,
  mapping_uid_to_num: Map<UserUUID, MappingId>,
}

impl TrustGraphNeuron {
  pub fn new(env: &Env, edges: Vec<(UserUUID, UserUUID)>) -> TrustGraphNeuron {
    let mut result = TrustGraphNeuron {
      trust_graph: None,
      mapping_num_to_uid: Map::new(env),
      mapping_uid_to_num: Map::new(env),
    };
    // TODO fix this code, maybe create a separate structure/wrapper for this graph
    // prepare mapping

    // set of user ids (created as map since soroban doesn't have set per se)
    // we need a set of unique ids that appeared in the input `edges`
    let mut set: Map<UserUUID, Option<bool>> = Map::new(env);
    for edge in edges.clone() {
      // set.set does this: create or if exists, update, so that's ok
      set.set(edge.0, None);
      set.set(edge.1, None);
    }

    // creates mapping of following numbers (0,1,2,...) to each node's user uuid
    // mapping_num_to_uid: { 0: 'john', 1: 'alice', 2: 'jessie', ... }
    // mapping_uid_to_num: { 'john': 0, 'alice': 1, 'jessie': 2, ... }
    // I created 2 maps instead of one vec of tuples so the lookups are faster
    let mut mapping_id = 0;
    for item in set {
      let item = item.0;
      result.mapping_num_to_uid.set(mapping_id, item.clone());
      result.mapping_uid_to_num.set(item, mapping_id);
      mapping_id += 1;
    }
    // create mapped edges, we have to operate on `MappingId`,
    // which is not a soroban type so the lib graph will work with it
    let mut mapped_edges: Vec<(MappingId, MappingId)> = Vec::new(env);
    for edge in edges {
      mapped_edges.push_back((
        result.mapping_uid_to_num.get(edge.0).unwrap(),
        result.mapping_uid_to_num.get(edge.1).unwrap(),
      ));
    }

    // create graph
    let graph: DirectedCsrGraph<MappingId> = GraphBuilder::new()
      .csr_layout(CsrLayout::Sorted)
      .edges(mapped_edges)
      .build();
    result.trust_graph = Some(graph);
    result
  }

  // TODO maybe convert this to `From` or sth
  pub fn new_from_map(env: &Env, map_edges: Map<UserUUID, Vec<UserUUID>>) -> TrustGraphNeuron {
    let mut vec_edges: Vec<(UserUUID, UserUUID)> = Vec::new(env);
    for (trusting_user_id, trusted_users_ids) in map_edges {
      for trusted_user_id in trusted_users_ids {
        vec_edges.push_back((trusting_user_id.clone(), trusted_user_id));
      }
    }
    TrustGraphNeuron::new(env, vec_edges)
  }

  fn compute_trust_score(&self, env: &Env) -> Map<UserUUID, DecimalNumber> {
    // max_iterations, tolerance, damping_factor
    let config = PageRankConfig::new(100, 1e-6, 0.85);

    let (ranks, _iterations, _error) = page_rank(self.trust_graph.as_ref().unwrap(), config);

    let min = (&ranks)
      .into_iter()
      .min_by(|a, b| a.partial_cmp(b).unwrap())
      .unwrap();
    let max = (&ranks)
      .into_iter()
      .max_by(|a, b| a.partial_cmp(b).unwrap())
      .unwrap();

    let mut trust_score: Map<UserUUID, DecimalNumber> = Map::new(&env);
    let mut mapping_id = 0;
    loop {
      let user_uuid = self.mapping_num_to_uid.get(mapping_id);
      if let Some(id) = user_uuid {
        let user_rank = ranks[mapping_id as usize];
        let calculated_rank = (user_rank - min) / (max - min);
        trust_score.set(id, DecimalNumberPersist::from(calculated_rank).as_tuple());
      } else {
        break;
      }

      mapping_id += 1;
    }
    trust_score
  }
}

impl Neuron for TrustGraphNeuron {
  fn oracle_function(
    &self,
    env: &Env,
    voter_id: UserUUID,
    _project_id: ProjectUUID,
    _previous_layer_vote: &Option<DecimalNumber>,
  ) -> DecimalNumber {
    log!(&env, "======================= Hello {}", voter_id);
    self
      .compute_trust_score(env)
      .get(voter_id)
      .expect("no trust score found for requested user id")
  }

  fn weight_function(&self, _env: &Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber {
    // TODO what here?
    raw_neuron_vote
  }
}
