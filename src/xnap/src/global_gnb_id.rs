use super::plmn_id::PlmnId;

pub struct GlobalGnbId {
  pub node_id: u64,
  pub plmn_id: PlmnId,
}

impl GlobalGnbId {
  pub fn new(node_id: u64, plmn_id: PlmnId) -> Self {
    Self {
        node_id,
        plmn_id
    }
  }
}

impl Default for GlobalGnbId {
  fn default() -> Self {
      GlobalGnbId {
          node_id: std::u64::MAX, // Sentinel value for node_id
          plmn_id: PlmnId::default(),
      }
  }
}