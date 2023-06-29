// use super::*; // replaced by rows below (not using glob operator *), but not cleaned up

use crate::{c_void, WrapperXnapPdu};
use super::global_gnb_id::{GlobalGnbId};
use super::decompile_gnb_id::{decompile_global_ng_ran_node_id_gnb};
use super::tai_support_item::{TaiSupportItem};
use super::decompile_tai_support_list::{decompile_tai_support_list};
use super::served_cell_info::ServedCellInfo;
use super::decompile_served_cells::decompile_served_cells;
use super::xn_setup_request::XnSetupRequest;
use crate::bindings::{wrapper_match_pdu_type, wrapper_check_procedure_code, wrapper_create_setup_request, wrapper_get_first_protocol_IE, wrapper_get_element_id, wrapper_get_next_protocol_IE, wrapper_get_count};

use crate::xnap::src::xn_setup_request::PrintMessage;

pub struct InitiatingMessage {
    pub obj: *mut c_void
  }
  
  impl InitiatingMessage {
    fn new(obj: *mut c_void) -> Self {
      Self {
        obj: obj,
      }
    }
  }

pub struct WrapperProtocolIE {
  pub obj: *const c_void
}
  
  impl WrapperProtocolIE {
    fn new(obj: *const c_void) -> Self {
      Self {
        obj: obj,
      }
    }
  }

pub fn create_setup_request(input: InitiatingMessage) -> XnSetupRequest { // input is asncXNAP_XnAP_PDU__initiatingMessage
  // println!("Creating setup request message!");
  // cast to XNAP_XnSetupRequest
  unsafe {
    let mut e_served_cells_list: Vec<ServedCellInfo> = Vec::new();
    let mut tai_support_list: Vec<TaiSupportItem> = Vec::new();
    let mut redacted_1: <redacted> = <redacted>;

    let xn_setup_request = wrapper_create_setup_request(input.obj); // in: XNAP_XnAP_PDU. cast to asncXNAP_XnAP_PDU__initiatingMessage, then XNAP_XnSetupRequest
    let mut global_gnb = GlobalGnbId::default();
    // println!("GlobalGnb: NodeId = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", global_gnb.node_id, global_gnb.plmn_id.mcc, global_gnb.plmn_id.mnc, global_gnb.plmn_id.mnc_length);
    let mut current_protocol_ie = wrapper_get_first_protocol_IE(xn_setup_request); //getfirst returns a protocol_ie that is put into a ProtocolIe struct
    let count = wrapper_get_count(xn_setup_request);
    for _i in 0..count {
      let element_id = wrapper_get_element_id(current_protocol_ie); //returns enum ProtocolIeType
      // println!("Element ID = {}", element_id);
      match element_id {
        1 => { // GlobalRanNodeId
          // println!("Element of type GlobalRanNodeId");
          global_gnb = decompile_global_ng_ran_node_id_gnb(WrapperProtocolIE::new(current_protocol_ie));
          // println!("GlobalGnb: NodeId = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", global_gnb.node_id, global_gnb.plmn_id.mcc, global_gnb.plmn_id.mnc, global_gnb.plmn_id.mnc_length);
        },
        2 => { // TaiSupportList
          // println!("Element of type TAISupportList");
          tai_support_list = decompile_tai_support_list(WrapperProtocolIE::new(current_protocol_ie));
          // println!("TaiSupportItem: Tac = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", tai_support_list[0].tac, tai_support_list[0].broadcast_plmns[0].plmn_id.mcc, tai_support_list[0].broadcast_plmns[0].plmn_id.mnc, tai_support_list[0].broadcast_plmns[0].plmn_id.mnc_length);
        },
        3 => { // ListOfServedCells
          // println!("Element of type ListOfServedCells");
          e_served_cells_list = decompile_served_cells(WrapperProtocolIE::new(current_protocol_ie));
        },
        _ => {
          // println!("Not of any type");
          break;
        },
      }
      current_protocol_ie=wrapper_get_next_protocol_IE(current_protocol_ie);
    }
    e_served_cells_list.iter_mut()
      .zip(redacted_1.0.iter())
      .filter(|(a, b)| a.cell_id == b.cell_id)
      .for_each(|(a, b)| {
        let mut_a = &mut *a;
        mut_a.add_x(&b.clone());
      });
      // println!("GlobalGnb: NodeId = {}, PlmnId mcc = {}, PlmnId mnc = {}, PlmnId mnc_length = {}", global_gnb.node_id, global_gnb.plmn_id.mcc, global_gnb.plmn_id.mnc, global_gnb.plmn_id.mnc_length);
    let e_xn_setup_request: XnSetupRequest = XnSetupRequest::new(global_gnb, tai_support_list, e_served_cells_list, redacted_1.1);
    return e_xn_setup_request;
  }
}

pub fn create (input: WrapperXnapPdu) -> impl PrintMessage {
  unsafe {
    let message_type = wrapper_match_pdu_type(input.obj);
    match message_type {
      _initiating => {
        // println!("Initiating message!");
        let initiating_message = InitiatingMessage::new(input.obj); // struct for type safety; we now know that input.obj is an initiating message

        let procedure_code = wrapper_check_procedure_code(input.obj); // sending input xnap_xnap_pdu, via asncXNAP_XnAP_PDU__initiatingMessage, returns that the message is a setup message

        match procedure_code {
          _setup_req => {
            // getting asncXNAP_XnAP_PDU__initiatingMessage, cast to XNAP_XnSetupRequest (and put into a struct)
            // let _setup_request = XnSetupRequest::new(wrapper_create_setup_request(initiating_message.obj));
            return create_setup_request(initiating_message);
          },
        };
      },
    }
  }
}