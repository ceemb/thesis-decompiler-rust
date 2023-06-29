use super::broadcast_plmn_in_tai_support::BroadcastPlmnInTAISupport;

pub struct TaiSupportItem {
    pub tac: u32,
    pub broadcast_plmns: Vec<BroadcastPlmnInTAISupport>,
}

impl TaiSupportItem {
  pub fn new(tac: u32, broadcast_plmns: Vec<BroadcastPlmnInTAISupport>) -> Self {
    Self {
        tac,
        broadcast_plmns,
    }
  }
}