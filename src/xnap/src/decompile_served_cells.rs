use crate::{c_void};
use crate::message_factory::WrapperProtocolIE;
use super::served_cell_info::ServedCellInfo;
use super::plmn_id::PlmnId;
use super::nr_mode_info_tdd::NrModeInfoTdd;
use super::nr_frequency_info::NrFrequencyInfo;
use super::nr_transmission_bandwidth:: NrTransmissonBandwidth;
use crate::xnap::src::{to_int::deserialize_64bit, nr_cgi::NrCgi, decompile_asn_types::decompile_tac};
use crate::xnap::src::decompile_plmn_id::{decompile_plmn_id};
use crate::xnap::src::array_from_length_value::{array_from_length_value_8};
use crate::bindings::{wrapper_get_list_of_served_cells, wrapper_get_first_served_cell, wrapper_get_cells_count, wrapper_get_nr_pci, wrapper_get_cell_id, wrapper_get_plmn_from_cell_id, wrapper_get_nrci, wrapper_get_cell_tac, wrapper_get_next_served_cell_item, wrapper_get_first_broadcast_plmn, wrapper_get_broadcast_plmn_count, wrapper_get_next_broadcast_plmn, wrapper_get_plmn_from_broadcast_plmn, wrapper_check_mode_is_tdd, wrapper_get_frequency_info, wrapper_get_nr_arfcn, wrapper_get_first_band_item, wrapper_get_band_items_count, wrapper_get_frequency_band, wrapper_get_next_band_item, wrapper_get_transmission_bandwidth, wrapper_get_nrscs, wrapper_get_nrnrb, wrapper_get_measurement_timing_configuration, wrapper_get_connectivity_support, LengthAndValue};

pub struct WrapperCellId {
    pub obj: *const c_void
}

impl WrapperCellId {
    pub fn new(obj: *const c_void) -> Self {
    Self {
        obj: obj,
    }
    }
}

pub struct WrapperFrequencyInfo {
    pub obj: *const c_void
}

impl WrapperFrequencyInfo {
    fn new(obj: *const c_void) -> Self {
    Self {
        obj: obj,
    }
    }
}

pub struct WrapperTransmissionBandwidth {
    pub obj: *const c_void
}

impl WrapperTransmissionBandwidth {
    fn new(obj: *const c_void) -> Self {
    Self {
        obj: obj,
    }
    }
}

pub fn decompile_cell_id(cell_id: WrapperCellId) -> NrCgi {
    // println!("Entered Decompile cell id");
    unsafe{
        let plmn_id_data: *mut LengthAndValue = wrapper_get_plmn_from_cell_id(cell_id.obj); // Fetching the plmn id string to be used as parameter in decompile plmn, equivalent of "mCellId.plmn_id" on row 34 in c++ decompiler (row number outdated?) //returns LengthAndValue containing PlmnId
        let plmn_id = decompile_plmn_id(plmn_id_data);
        let nr_ci_data = wrapper_get_nrci(cell_id.obj); //Fetching the nr_CI as a LengthAndValue struct to be used as parameters in Deserialize64bit, equivalent of "mCellId.nr_CI" on row 35 in c++ decompiler (row number outdated?) // nr_ci is of type LengthAndValue*
        let nr_ci_array = array_from_length_value_8(nr_ci_data);
        let nr_ci = deserialize_64bit(&nr_ci_array, nr_ci_data);
        return NrCgi::new(plmn_id, nr_ci);
    }
}

pub fn decompile_nr_mode_info_tdd_frequency(frequency_info: WrapperFrequencyInfo) -> NrFrequencyInfo {
    // println!("Decompiling Frequency info");
    unsafe{
        let nr_arfcn = wrapper_get_nr_arfcn(frequency_info.obj);
        let mut e_frequency_band_list: Vec<i32> = Vec::new();
        let mut current_band_item = wrapper_get_first_band_item(frequency_info.obj);
        let band_count = wrapper_get_band_items_count(frequency_info.obj);
        for _i in 0..band_count {
            let nr_frequency_band = wrapper_get_frequency_band(current_band_item);
            e_frequency_band_list.push(nr_frequency_band);
            current_band_item = wrapper_get_next_band_item(current_band_item);
        }
        return NrFrequencyInfo::new(nr_arfcn, e_frequency_band_list);
    }
}

pub fn decompile_nr_mode_info_tdd_transmission(transmission_bandwidth: WrapperTransmissionBandwidth) -> NrTransmissonBandwidth {
    unsafe{
        // println!("Decompiling transmission bandwidth");
        let n_rscs = wrapper_get_nrscs(transmission_bandwidth.obj);
        let n_rnrb = wrapper_get_nrnrb(transmission_bandwidth.obj);
        return NrTransmissonBandwidth::new(n_rscs, n_rnrb);
    }

}

pub fn decompile_served_cells(current_protocol_ie: WrapperProtocolIE) -> Vec<ServedCellInfo> {
    // println!("Decompiling served cells");
    unsafe{
        let mut e_served_cell_info_list: Vec<ServedCellInfo> = Vec::new();
        let served_cells_list = wrapper_get_list_of_served_cells(current_protocol_ie.obj); // Fetching the cells-list to be used as parameter in getFirst and getCount
        let mut current_served_cells_item = wrapper_get_first_served_cell(served_cells_list);
        let count = wrapper_get_cells_count(served_cells_list);
        // println!("List of served cells is of length {}", count);
        for _i in 0..count {

            let nr_pci = wrapper_get_nr_pci(current_served_cells_item);

            let cell_id = wrapper_get_cell_id(current_served_cells_item); // Fetching the cell id to be used as parameter in decompile cell id, equivalent of "currentServedCellsItem->served_cell_info_NR->cellID" on row 80 in c++ decompiler (row number outdated?)
            let e_nr_cgi = decompile_cell_id(WrapperCellId::new(cell_id));
            // println!("Getting cell tac");
            let cell_tac_data = wrapper_get_cell_tac(current_served_cells_item); //Fetching the tac to be used as parameter in decompile tac, equivalent of "currentServedCellsItem->served_cell_info_NR->tac" on row 82 in c++ decompiler (row number outtdated?)
            let cell_tac = decompile_tac(cell_tac_data);

            let mut e_broadcast_plmns: Vec<PlmnId> = Vec::new();
            let mut current_broadcast_plmn = wrapper_get_first_broadcast_plmn(current_served_cells_item);
            let plmn_count = wrapper_get_broadcast_plmn_count(current_served_cells_item);

            for _j in 0..plmn_count {
                let plmn_data = wrapper_get_plmn_from_broadcast_plmn(current_broadcast_plmn);
                let plmn_id = decompile_plmn_id(plmn_data);
                e_broadcast_plmns.push(plmn_id);
                current_broadcast_plmn = wrapper_get_next_broadcast_plmn(current_broadcast_plmn);
            }

            let mode_info_is_tdd = wrapper_check_mode_is_tdd(current_served_cells_item);
            if !mode_info_is_tdd {
                panic!("Mode info, not tdd");
            }
            let frequency_info = wrapper_get_frequency_info(current_served_cells_item); //Fetching the frequency info to be used as parameter in decompile_nr_mode_info_tdd_frequency, equivalent of:

            // "static_cast<asncXNAP_NRModeInfo__tdd*>(currentServedCellsItem->served_cell_info_NR->nrModeInfo)->nrFrequencyInfo"    

            // on row 99 in c++ decompiler (row number outdated?)

            let e_frequency_info = decompile_nr_mode_info_tdd_frequency(WrapperFrequencyInfo::new(frequency_info));
            
            
            let bandwidth = wrapper_get_transmission_bandwidth(current_served_cells_item); //Fetching the transmission bandwidth to be used as parameter in decompile_nr_mode_info_tdd_transmission, equivalent of:

            // "static_cast<asncXNAP_NRModeInfo__tdd*>(currentServedCellsItem->served_cell_info_NR->nrModeInfo)->nrTransmissonBandwidth"    

            // on row 100 in c++ decompiler (row number outdated?)

            let e_transmission_bandwidth = decompile_nr_mode_info_tdd_transmission(WrapperTransmissionBandwidth::new(bandwidth));
            let e_nr_mode_info_tdd = NrModeInfoTdd::new(e_frequency_info, e_transmission_bandwidth);

            let measurement_timing_configuration = wrapper_get_measurement_timing_configuration(current_served_cells_item); // Fetching the measurement_timing_configuration, equivalent of "currentServedCellsItem->served_cell_info_NR->measurementTimingConfiguration" on row 105 + rows 16-18 in decompile_asn_types.cc because we cannot return asnOctetString. (row numbers outdated?) // Currently returns LengthAndValue struct.
            let e_measurement_timing_configuration = array_from_length_value_8(measurement_timing_configuration);

            let connectivity_support = wrapper_get_connectivity_support(current_served_cells_item);

            e_served_cell_info_list.push(ServedCellInfo::new(nr_pci, e_nr_cgi, cell_tac, e_broadcast_plmns, e_nr_mode_info_tdd, e_measurement_timing_configuration, connectivity_support));
            current_served_cells_item = wrapper_get_next_served_cell_item(current_served_cells_item);
        }
        return e_served_cell_info_list;
    }
}