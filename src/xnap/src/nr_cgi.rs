use super::plmn_id::PlmnId;
#[derive(PartialEq, Copy, Clone)]
pub struct NrCgi {
    pub plmn_id: PlmnId,
    pub nr_ci: u64,
  }
    
    impl NrCgi {
      pub fn new(plmn_id: PlmnId, nr_ci: u64) -> Self {
        Self {
            plmn_id,
            nr_ci
        }
      }
    }