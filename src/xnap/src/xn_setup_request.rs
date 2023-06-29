use super::global_gnb_id::GlobalGnbId;
use super::tai_support_item::TaiSupportItem;
use super::served_cell_info::ServedCellInfo;

pub trait PrintMessage {
    fn print_message(&self);
}

pub struct XnSetupRequest {
    pub global_gnb_id: GlobalGnbId,
    pub tai_support_list: Vec<TaiSupportItem>,
    pub served_cells_list: Vec<ServedCellInfo>,
}
impl XnSetupRequest {
    pub fn new(
        global_gnb_id: GlobalGnbId,
        tai_support_list: Vec<TaiSupportItem>,
        served_cells_list: Vec<ServedCellInfo>) -> Self {
        Self {
            global_gnb_id,
            tai_support_list,
            served_cells_list,
        }
    }
}

impl PrintMessage for XnSetupRequest {
    fn print_message(&self) {
        println!("\nPrinting message:");
        println!("GlobalGnb: NodeId = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", self.global_gnb_id.node_id, self.global_gnb_id.plmn_id.mcc, self.global_gnb_id.plmn_id.mnc, self.global_gnb_id.plmn_id.mnc_length);
        println!("TaiSupportItem[0]: Tac = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", self.tai_support_list[0].tac, self.tai_support_list[0].broadcast_plmns[0].plmn_id.mcc, self.tai_support_list[0].broadcast_plmns[0].plmn_id.mnc, self.tai_support_list[0].broadcast_plmns[0].plmn_id.mnc_length);
        println!("ServedcellsList[0]: NrPci = {}, CellId.nr_ci = {}, Tac = {}, ConnectivitySupport = {}", self.served_cells_list[0].nr_pci, self.served_cells_list[0].cell_id.nr_ci, self.served_cells_list[0].tac, self.served_cells_list[0].connectivity_support);
    }
}