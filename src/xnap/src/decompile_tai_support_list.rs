use crate::{c_void};
use crate::message_factory::WrapperProtocolIE;
use super::tai_support_item::TaiSupportItem;
use super::decompile_asn_types::decompile_tac;
use super::broadcast_plmn_in_tai_support::BroadcastPlmnInTAISupport;
use super::xnap_s_nssai::XnapSNssai;
use super::decompile_plmn_id::decompile_plmn_id;
use crate::xnap::src::to_int::get_value_as_int32;
use crate::xnap::src::array_from_length_value::{array_from_length_value_8};
use crate::bindings::{wrapper_get_tai_list_from_protocol_ie, wrapper_get_first_support_item, wrapper_get_support_items_count, wrapper_get_tai_tac, wrapper_get_tai_broadcast_plmn_count, wrapper_get_first_tai_broadcast_plmn, wrapper_get_plmn_from_tai, wrapper_get_first_xnapsnssai, wrapper_get_xnapsnssai_count, wrapper_get_xnapsnssai_sst, wrapper_get_sd_presence, wrapper_get_xnapsnssai_sd, wrapper_get_next_xnapsnssai, wrapper_get_next_broadcast_item, wrapper_get_next_tai_support_item, wrapper_free};
use std::convert::TryInto;

pub struct WrapperBroadcastItem {
    pub obj: *const c_void
}

impl WrapperBroadcastItem {
    fn new(obj: *const c_void) -> Self {
    Self {
        obj: obj,
    }
    }
}

pub struct WrapperXnapSNssai {
    pub obj: *const c_void
}

impl WrapperXnapSNssai {
    fn new(obj: *const c_void) -> Self {
    Self {
        obj: obj,
    }
    }
}

pub fn decompile_xnapsnssai(current_xnapsnssai: &WrapperXnapSNssai) -> XnapSNssai {
    unsafe{
        // println!("Decompiling XnapSNssai");
        let sst_data = wrapper_get_xnapsnssai_sst(current_xnapsnssai.obj); // ok, since Rust's default is move, not copy (as in C++)
        let sst_array = array_from_length_value_8(sst_data);
        // println!("sst length = {}", (*sst_data).length);
        let sst = get_value_as_int32(&sst_array, (*sst_data).length.try_into().unwrap());
        wrapper_free(sst_data);
        let sd_presence = wrapper_get_sd_presence(current_xnapsnssai.obj);
        if sd_presence {
            let sd_data = wrapper_get_xnapsnssai_sd(current_xnapsnssai.obj);
            let sd_array = array_from_length_value_8(sd_data);
            let sd = get_value_as_int32(&sd_array, (*sd_data).length.try_into().unwrap());
            wrapper_free(sd_data);
            return XnapSNssai::new(sst, Some(sd));
        } else {
            return XnapSNssai::new(sst, None);
        }
    }
}

pub fn decompile_broadcast_plmns(current_broadcast_item: &WrapperBroadcastItem) -> BroadcastPlmnInTAISupport {
    unsafe{
        // println!("Decompiling broadcast plmns");
        let plmn_id_data = wrapper_get_plmn_from_tai(current_broadcast_item.obj); //Fetching the plmn id as a LengthAndValue struct to be used as parameter in decompile plmn_id, equivalent of "mBroadcastItem.plmn_id" on row 56 in c++ decompiler (row number outdated?)
        let plmn_id = decompile_plmn_id(plmn_id_data);

        let mut e_tai_slice_support_list: Vec<XnapSNssai> = Vec::new();
        let mut current_xnapsnssai = WrapperXnapSNssai::new(wrapper_get_first_xnapsnssai(current_broadcast_item.obj));
        let count = wrapper_get_xnapsnssai_count(current_broadcast_item.obj);
        // println!("Nr of XnapSNssai = {}", count);
        for _i in 0..count {
            let e_xnapsnssai = decompile_xnapsnssai(&current_xnapsnssai);
            // println!("XnapSnssai sst {}, sd {}", e_xnapsnssai.sst, match e_xnapsnssai.sd {
            //     None => 0,
            //     Some(sd) => sd
            //   });
            e_tai_slice_support_list.push(e_xnapsnssai);
            current_xnapsnssai.obj = wrapper_get_next_xnapsnssai(current_xnapsnssai.obj);
        }

        let e_broadcast_plmns = BroadcastPlmnInTAISupport::new(plmn_id, e_tai_slice_support_list);
        return e_broadcast_plmns;
    }
}

pub fn decompile_tai_support_list(current_protocol_ie: WrapperProtocolIE) -> Vec<TaiSupportItem> {
    unsafe {
        let tai_support_list = wrapper_get_tai_list_from_protocol_ie(current_protocol_ie.obj);
        let mut e_tai_support_list: Vec<TaiSupportItem> = Vec::new();
        let mut current_support_item = wrapper_get_first_support_item(tai_support_list);
        let count = wrapper_get_support_items_count(tai_support_list);
        // println!("Count = {}", count);
        for _i in 0..count {
            let tac_data = wrapper_get_tai_tac(current_support_item);// Fetching the tac to be used as parameter in decompile tac (equivalent of "currentSupportItem->tac" on row 84 in c++ decompiler (row number outdated?)
            let tac = decompile_tac(tac_data);
            
            let count_plmn = wrapper_get_tai_broadcast_plmn_count(current_support_item);
            let mut current_broadcast_item = WrapperBroadcastItem::new(wrapper_get_first_tai_broadcast_plmn(current_support_item));
            let mut e_broadcast_plmns: Vec<BroadcastPlmnInTAISupport> = Vec::new();
            for _j in 0..count_plmn {
                let new_broadcast_plmns = decompile_broadcast_plmns(&current_broadcast_item); // Sending with '&' to allow for "borrowing" of the value instead of handing it over (so it can be used again in getNextItem)
                e_broadcast_plmns.push(new_broadcast_plmns);
                current_broadcast_item.obj = wrapper_get_next_broadcast_item(current_broadcast_item.obj);
            }
            e_tai_support_list.push(TaiSupportItem::new(tac, e_broadcast_plmns));
            current_support_item = wrapper_get_next_tai_support_item(current_support_item);
        }
        return e_tai_support_list;
    }

}