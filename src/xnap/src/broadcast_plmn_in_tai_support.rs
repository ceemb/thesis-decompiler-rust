use super::plmn_id::PlmnId;
use super::xnap_s_nssai::XnapSNssai;

pub struct BroadcastPlmnInTAISupport {
    pub plmn_id: PlmnId,
    pub tai_slice_support_list: Vec<XnapSNssai>,
}

impl BroadcastPlmnInTAISupport {
    pub fn new(plmn_id: PlmnId, tai_slice_support_list: Vec<XnapSNssai>) -> Self {
    Self {
        plmn_id,
        tai_slice_support_list,
    }
    }
}