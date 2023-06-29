use crate::xnap::src::{plmn_id::PlmnId, to_int::bcd_to_uint};
use crate::xnap::src::array_from_length_value::array_from_length_value_3;
use crate::bindings::LengthAndValue;

pub fn decompile_plmn_id(plmn_id_data: *mut LengthAndValue) -> PlmnId {
    let plmn_id_array = array_from_length_value_3(plmn_id_data);
    let plmn_id = convert_plmn_octet_string(&plmn_id_array);
    return plmn_id;
}

pub fn convert_plmn_octet_string(pdu: &[u8]) -> PlmnId {
    let mut octet_string = [0; 3];
    octet_string.copy_from_slice(&pdu[..3]);

    let mcc = ((octet_string[0] & 0x0f) as u32) << 8 | (octet_string[0] & 0xf0) as u32 | (octet_string[1] & 0x0f) as u32;

    let (mnc, mnc_length) = if octet_string[1] & 0xf0 == 0xf0 {
        let mnc = ((octet_string[2] & 0x0f) as u32) << 4 | ((octet_string[2] & 0xf0) >> 4) as u32;
        (mnc, 2)
    } else {
        let mnc = ((octet_string[1] & 0xf0) as u32) << 4 | ((octet_string[2] & 0x0f) as u32) << 4 | ((octet_string[2] & 0xf0) >> 4) as u32;
        (mnc, 3)
    };

    let plmn_id = PlmnId::new(bcd_to_uint(mcc), bcd_to_uint(mnc), mnc_length);
    
    // println!("PlmnId in convert function: {{{}:{}({})}}", plmn_id.mcc, plmn_id.mnc, plmn_id.mnc_length); // "{{" prints "{" (escaping it, like "\{" would in latex)

    return plmn_id;
}