use crate::{c_void};
use crate::xnap::src::array_from_length_value::array_from_length_value_8;
use crate::xnap::src::{to_int::deserialize_64bit};
use crate::message_factory::WrapperProtocolIE;
use super::decompile_plmn_id::decompile_plmn_id;
use super::global_gnb_id::GlobalGnbId;
use crate::bindings::{wrapper_get_gnbPdu, wrapper_getgnbIdPdu_string, LengthAndValue, wrapper_get_plmn_from_global_gnb};

pub fn decompile_global_ng_ran_node_id_gnb(current_protocol_ie: WrapperProtocolIE) -> GlobalGnbId {
  // println!("Decompile global gnb id");
  unsafe{
    let pdu = wrapper_get_gnbPdu(current_protocol_ie.obj);
    let node_id = decompile_node_id(pdu);

    let plmn_id_data = wrapper_get_plmn_from_global_gnb(pdu);

    let plmn_id = decompile_plmn_id(plmn_id_data);
    // println!("{}", (*plmn_id_data).length);

    return GlobalGnbId::new(node_id, plmn_id);
  }
}

pub fn decompile_node_id(pdu: *const c_void) -> u64 {
  unsafe {
    let node_id_data: *mut LengthAndValue = wrapper_getgnbIdPdu_string(pdu);
    let node_id_array = array_from_length_value_8(node_id_data);
    let node_id = deserialize_64bit(&node_id_array, node_id_data);
    return node_id;
  }
}