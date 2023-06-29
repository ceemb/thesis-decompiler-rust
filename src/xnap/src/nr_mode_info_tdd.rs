use super::nr_frequency_info::NrFrequencyInfo;
use super::nr_transmission_bandwidth:: NrTransmissonBandwidth;

pub struct NrModeInfoTdd {
  pub nr_frequency_info: NrFrequencyInfo,
  pub nr_transmission_bandwidth: NrTransmissonBandwidth,
}

impl NrModeInfoTdd {
  pub fn new(nr_frequency_info: NrFrequencyInfo, nr_transmission_bandwidth: NrTransmissonBandwidth) -> Self {
    Self {
      nr_frequency_info,
      nr_transmission_bandwidth,
    }
  }
}