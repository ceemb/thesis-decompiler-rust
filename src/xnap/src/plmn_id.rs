#[derive(PartialEq, Copy, Clone)]
pub struct PlmnId {
  pub mcc: u32,
  pub mnc: u32,
  pub mnc_length: u32,
}

impl PlmnId {
  pub fn new(mcc: u32, mnc: u32, mnc_length: u32) -> Self {
    Self {
        mcc,
        mnc,
        mnc_length
    }
  }
}

impl Default for PlmnId {
  fn default() -> Self {
      PlmnId {
          mcc: std::u32::MAX, // Sentinel value for mcc
          mnc: std::u32::MAX, // Sentinel value for mnc
          mnc_length: std::u32::MAX, // Sentinel value for mnc_length
      }
  }
}