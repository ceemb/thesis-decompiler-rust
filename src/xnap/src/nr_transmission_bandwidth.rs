pub struct NrTransmissonBandwidth {
    pub n_rscs: i32,
    pub n_rnrb: i32,
  }
  
  impl NrTransmissonBandwidth {
    pub fn new(n_rscs: i32, n_rnrb: i32) -> Self {
      Self {
        n_rscs,
        n_rnrb,
      }
    }
  }