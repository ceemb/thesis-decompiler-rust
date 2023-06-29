use core::ffi::{c_void};
use libc::{c_int, c_char, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wrapper_asn_context;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LengthAndValue {
    pub length: c_uint,
    pub value: *const c_char,
}



#[link(name = "wrapper", kind = "static")]
extern "C" {
pub fn wrapper_free(input: *mut LengthAndValue);

/*Wrappers for asn context*/
pub fn wrapper_create_context() -> *mut c_void;
pub fn wrapper_delete_context(ctxt: *mut c_void);
pub fn wrapper_asn_decode(ctxt: *mut c_void, buffer: *mut u8, size: u64) -> *mut c_void;
pub fn wrapper_free_asn_message(message: *mut c_void);
/*End of wrappers for asn context*/

/*Wrappers for creating message*/
pub fn wrapperXnapPdu_create(input: *mut c_void) -> *mut c_void;
pub fn wrapper_match_pdu_type(xnapPdu: *mut c_void) -> c_int;
pub fn wrapper_check_procedure_code(xnapPdu: *const c_void) -> *const c_int;
pub fn wrapper_create_setup_request(input: *mut c_void) -> *const c_void;
pub fn wrapper_get_first_protocol_IE(input: *const c_void) -> *const c_void;
pub fn wrapper_get_element_id(input: *const c_void) -> c_int;
pub fn wrapper_get_next_protocol_IE(input: *const c_void) -> *const c_void;
pub fn wrapper_get_count(input: *const c_void) -> c_int;
/*End of wrappers for creating message*/

/*Wrappers for global gnb*/
pub fn wrapper_get_gnbPdu(input: *const c_void) -> *const c_void;
pub fn wrapper_getgnbIdPdu_string(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_plmn_from_global_gnb(input: *const c_void) -> *mut LengthAndValue;
/*End of wrappers for global gnb*/

/*Wrappers for TAI support list*/
pub fn wrapper_get_tai_list_from_protocol_ie(input: *const c_void) -> *const c_void;
pub fn wrapper_get_first_support_item(input: *const c_void) -> *const c_void;
pub fn wrapper_get_support_items_count(input: *const c_void) -> c_int;
pub fn wrapper_get_tai_tac(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_tai_broadcast_plmn_count(input: *const c_void) -> c_int;
pub fn wrapper_get_first_tai_broadcast_plmn(input: *const c_void) -> *const c_void;
pub fn wrapper_get_plmn_from_tai(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_first_xnapsnssai(input: *const c_void) -> *const c_void;
pub fn wrapper_get_xnapsnssai_count(input: *const c_void) -> c_int;
pub fn wrapper_get_xnapsnssai_sst(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_sd_presence(input: *const c_void) -> bool;
pub fn wrapper_get_xnapsnssai_sd(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_next_xnapsnssai(input: *const c_void) -> *const c_void;
pub fn wrapper_get_next_broadcast_item(input: *const c_void) -> *const c_void;
pub fn wrapper_get_next_tai_support_item(input: *const c_void) -> *const c_void;
/*End of wrappers for TAI support list*/

/*Wrappers for served cells list*/
pub fn wrapper_get_list_of_served_cells(input: *const c_void) -> *const c_void;
pub fn wrapper_get_first_served_cell(input: *const c_void) -> *const c_void;
pub fn wrapper_get_cells_count(input: *const c_void) -> c_int;
pub fn wrapper_get_nr_pci(current: *const c_void) -> c_int;
pub fn wrapper_get_cell_id(input: *const c_void) -> *const c_void;
pub fn wrapper_get_plmn_from_cell_id(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_nrci(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_cell_tac(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_first_broadcast_plmn(input: *const c_void) -> *const c_void;
pub fn wrapper_get_broadcast_plmn_count(input: *const c_void) -> c_int;
pub fn wrapper_get_next_broadcast_plmn(input: *const c_void) -> *const c_void;
pub fn wrapper_get_plmn_from_broadcast_plmn(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_check_mode_is_tdd(input: *const c_void) -> bool;
pub fn wrapper_get_frequency_info(input: *const c_void) -> *const c_void;
pub fn wrapper_get_nr_arfcn(input: *const c_void) -> c_int;
pub fn wrapper_get_first_band_item(input: *const c_void) -> *const c_void;
pub fn wrapper_get_band_items_count(input: *const c_void) -> c_int;
pub fn wrapper_get_frequency_band(input: *const c_void) -> c_int;
pub fn wrapper_get_next_band_item(input: *const c_void) -> *const c_void;
pub fn wrapper_get_transmission_bandwidth(input: *const c_void) -> *const c_void;
pub fn wrapper_get_nrscs(input: *const c_void) -> c_int;
pub fn wrapper_get_nrnrb(input: *const c_void) -> c_int;
pub fn wrapper_get_measurement_timing_configuration(input: *const c_void) -> *mut LengthAndValue;
pub fn wrapper_get_connectivity_support(input: *const c_void) -> c_int;
pub fn wrapper_get_next_served_cell_item(input: *const c_void) -> *const c_void;

/*End of wrappers for served cells list*/
}
