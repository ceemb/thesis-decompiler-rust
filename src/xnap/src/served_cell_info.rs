use super::nr_cgi::NrCgi;
use super::plmn_id::PlmnId;
use super::nr_mode_info_tdd::NrModeInfoTdd;

pub struct ServedCellInfo {
  pub nr_pci: i32,
  pub cell_id: NrCgi,
  pub tac: u32,
  pub broadcast_plmns: Vec<PlmnId>,
  pub nr_mode_info_tdd: NrModeInfoTdd,
  pub measurement_timing_configuration: [u8; 8],
  pub connectivity_support: i32,
}

impl ServedCellInfo {
  pub fn new(nr_pci: i32, cell_id: NrCgi, tac: u32, broadcast_plmns: Vec<PlmnId>, nr_mode_info_tdd: NrModeInfoTdd, measurement_timing_configuration: [u8; 8], connectivity_support: i32) -> Self {
    Self {
      nr_pci,
      cell_id,
      tac,
      broadcast_plmns,
      nr_mode_info_tdd,
      measurement_timing_configuration,
      connectivity_support,
    }
  }
}