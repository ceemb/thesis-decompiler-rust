#ifndef _WRAPPER_H_
#define _WRAPPER_H_

#include <vector>


#ifdef __cplusplus
extern "C" {
#endif
struct C_XNAP_XnAP_PDU;
typedef struct C_XNAP_XnAP_PDU C_XNAP_XnAP_PDU_t;

struct wrapper_asn_context;

typedef struct {
    unsigned int length;
    unsigned char* value;
} LengthAndValue;
void wrapper_free(LengthAndValue* input);

/*Wrappers for asn context*/
struct wrapper_asn_context* wrapper_create_context();
void wrapper_delete_context(struct wrapper_asn_context* ctxt);

void* wrapper_asn_decode(struct wrapper_asn_context* ctxt, unsigned char* buffer, unsigned long size);
void wrapper_free_asn_message(void* message);
/*End of wrappers for asn context*/

/*Wrappers for creating message*/
void* wrapperXnapPdu_create(XNAP_XnAP_PDU input);
int wrapper_match_pdu_type(XNAP_XnAP_PDU xnapPdu);
int wrapper_check_procedure_code(XNAP_XnAP_PDU xnapPdu);
const XNAP_XnSetupRequest& wrapper_create_setup_request(XNAP_XnAP_PDU input);
int wrapper_get_count(const XNAP_XnSetupRequest& xnSetupPdu);
const asncXNAP_XnSetupRequest__protocolIEss* wrapper_get_first_protocol_IE(XNAP_XnSetupRequest input);
int wrapper_get_element_id(asncXNAP_XnSetupRequest__protocolIEss* input);
const asncXNAP_XnSetupRequest__protocolIEss* wrapper_get_next_protocol_IE(const asncXNAP_XnSetupRequest__protocolIEss* currentProtocolIE);
/*End of wrappers for creating message*/

/*Wrappers for global gnb*/
const asncXNAP_GlobalNG_RANNode_ID__gNB& wrapper_get_gnbPdu(asncXNAP_XnSetupRequest__protocolIEss* ie);
LengthAndValue* wrapper_getgnbIdPdu_string(asncXNAP_GlobalNG_RANNode_ID__gNB& pdu);
LengthAndValue* wrapper_get_plmn_from_global_gnb(asncXNAP_GlobalNG_RANNode_ID__gNB& pdu);
/*End of wrappers for global gnb*/

/*Wrappers for TAI support list*/
const XNAP_TAISupport_List& wrapper_get_tai_list_from_protocol_ie(asncXNAP_XnSetupRequest__protocolIEss* ie);
const asncXNAP_TAISupport_Lists* wrapper_get_first_support_item(const XNAP_TAISupport_List& mTaiList);
int wrapper_get_support_items_count(const XNAP_TAISupport_List& mTaiList);
LengthAndValue* wrapper_get_tai_tac(const asncXNAP_TAISupport_Lists* currentSupportItem);
int wrapper_get_tai_broadcast_plmn_count(const asncXNAP_TAISupport_Lists* currentSupportItem);
const asncXNAP_TAISupport_Item__broadcastPLMNss* wrapper_get_first_tai_broadcast_plmn(const asncXNAP_TAISupport_Lists* currentSupportItem);
LengthAndValue* wrapper_get_plmn_from_tai(const asncXNAP_TAISupport_Item__broadcastPLMNss* currentBroadcastItem);
const asncXNAP_SliceSupport_Lists* wrapper_get_first_xnapsnssai(const asncXNAP_TAISupport_Item__broadcastPLMNss* mBroadcastItem);
int wrapper_get_xnapsnssai_count(const asncXNAP_TAISupport_Item__broadcastPLMNss* mBroadcastItem);
LengthAndValue* wrapper_get_xnapsnssai_sst(const asncXNAP_SliceSupport_Lists* mXnapSNssai);
bool wrapper_get_sd_presence(const asncXNAP_SliceSupport_Lists* mXnapSNssai);
LengthAndValue* wrapper_get_xnapsnssai_sd(const asncXNAP_SliceSupport_Lists* mXnapSNssai);
const asncXNAP_SliceSupport_Lists* wrapper_get_next_xnapsnssai(const asncXNAP_SliceSupport_Lists* currentXnapSNssai);
const asncXNAP_TAISupport_Item__broadcastPLMNss* wrapper_get_next_broadcast_item(const asncXNAP_TAISupport_Item__broadcastPLMNss* currentBroadcastItem);
const asncXNAP_TAISupport_Lists* wrapper_get_next_tai_support_item(const asncXNAP_TAISupport_Lists* currentSupportItem);
/*End of wrappers for TAI support list*/

/*Wrappers for served cells list*/
const XNAP_ServedCells_NR& wrapper_get_list_of_served_cells(const asncXNAP_XnSetupRequest__protocolIEss* ie);
const asncXNAP_ServedCells_NRs* wrapper_get_first_served_cell(const XNAP_ServedCells_NR& list);
int wrapper_get_cells_count(const XNAP_ServedCells_NR& list);
int wrapper_get_nr_pci(const asncXNAP_ServedCells_NRs* current);
const XNAP_NR_CGI& wrapper_get_cell_id(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
LengthAndValue* wrapper_get_plmn_from_cell_id(const XNAP_NR_CGI& cell_id);
LengthAndValue* wrapper_get_nrci(const XNAP_NR_CGI& cell_id);
LengthAndValue* wrapper_get_cell_tac(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
const asncXNAP_BroadcastPLMNss* wrapper_get_first_broadcast_plmn(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
int wrapper_get_broadcast_plmn_count(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
const asncXNAP_BroadcastPLMNss* wrapper_get_next_broadcast_plmn(const asncXNAP_BroadcastPLMNss* currentBroadcastPLMN);
LengthAndValue* wrapper_get_plmn_from_broadcast_plmn(const asncXNAP_BroadcastPLMNss& broadcast_plmn); 
bool wrapper_check_mode_is_tdd(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
const XNAP_NRFrequencyInfo* wrapper_get_frequency_info(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
int wrapper_get_nr_arfcn(const XNAP_NRFrequencyInfo* frequency_info);
const asncXNAP_NRFrequencyBand_Lists* wrapper_get_first_band_item(const XNAP_NRFrequencyInfo* frequency_info);
int wrapper_get_band_items_count(const XNAP_NRFrequencyInfo* frequency_info);
int wrapper_get_frequency_band(const asncXNAP_NRFrequencyBand_Lists* currentBandItem);
const asncXNAP_NRFrequencyBand_Lists* wrapper_get_next_band_item(const asncXNAP_NRFrequencyBand_Lists* currentBandItem);
const XNAP_NRTransmissionBandwidth* wrapper_get_transmission_bandwidth(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
int wrapper_get_nrscs(const XNAP_NRTransmissionBandwidth* mNrTransInfo);
int wrapper_get_nrnrb(const XNAP_NRTransmissionBandwidth* mNrTransInfo);
LengthAndValue* wrapper_get_measurement_timing_configuration(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
int wrapper_get_connectivity_support(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
const asncXNAP_ServedCells_NRs* wrapper_get_next_served_cell_item(const asncXNAP_ServedCells_NRs* currentServedCellsItem);
/*End of wrappers for served cells list*/

#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* _WRAPPER_H_ */
