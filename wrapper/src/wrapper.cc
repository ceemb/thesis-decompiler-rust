#include "../inc/wrapper.h"

#include <iostream>
#include <memory>
#include <vector>

/*c++ functions*/
LengthAndValue* c_octetString_to_length_value(const asnOctetString& octetString){
  asnMAXUINT length = 0;
  asnbyte* value = nullptr;
  octetString.getOctetString(&length, &value);

  LengthAndValue* returnValue = (LengthAndValue*)malloc(sizeof(*returnValue));
  returnValue->length = length;
  returnValue->value = value;
  // std::cout << "Length = "<< length << std::endl;
  return returnValue;
}

LengthAndValue* c_bitString_to_length_value(const asnBitString& bitString){
  asnMAXUINT length = 0;
  asnbyte* value = nullptr;
  bitString.getBitString(&length, &value);
  LengthAndValue* returnValue = (LengthAndValue*)malloc(sizeof(*returnValue));
  returnValue->length = length;
  returnValue->value = value;
  // std::cout << "Length = "<< length << std::endl;
  return returnValue;
}

void wrapper_free(LengthAndValue* input){
  free(input);
}

/*End of c++ functions*/

/*Wrappers for asn context*/
struct wrapper_asn_context {
  asnContext* context;
};

struct wrapper_asn_context* wrapper_create_context() {
  struct wrapper_asn_context* c = new wrapper_asn_context();
  c->context = new asnContext();
  c->context->setAbstractSyntax(&XNAP_AbstractSyntax);
  c->context->setAutomaticDecoding(asnTRUE);
  c->context->setConstraintCheckDuringDecoding(asnTRUE);
  c->context->setConstraintCheckDuringEncoding(asnTRUE);

  // std::cout << "Created context 0x" << std::hex << (size_t)c << std::dec << std::endl;

  return c;
}

void wrapper_delete_context(struct wrapper_asn_context* ctxt) {
  // std::cout << "Deleted context 0x" << std::hex << (size_t)ctxt << std::dec << std::endl;

  delete ctxt->context;
  delete ctxt;
}

void* wrapper_asn_decode(struct wrapper_asn_context* ctxt, unsigned char* buffer, unsigned long size) {
  // std::cout << "Decoding message of size " << size << std::endl;
  
  asnMemoryStream inputStream(buffer, size, asnFSTREAM_READ);
  XNAP_XnAP_PDU* asnMessage = new XNAP_XnAP_PDU();
  try {
    // std::cout << "### Asn1 Decode start ###" << std::endl;
    asnMessage->PERdecode(ctxt->context, &inputStream);
    // std::cout << "### Asn1 Decode end ### " << std::endl;
  }
  catch (const asnException& e) {
    // std::cout << "Failed to decode message" << std::endl;
    delete asnMessage;
    return NULL;
  }

  // std::cout << "Returning message 0x" << std::hex << (size_t)asnMessage << std::dec << std::endl;
  return asnMessage;
}

void wrapper_free_asn_message(void* message) {
  // std::cout << "Deleting message 0x" << std::hex << (size_t)message << std::dec << std::endl;
  delete (XNAP_XnAP_PDU*)message;
}
/*End of wrappers for asn context*/

/////////////////////////////////////////////////////////////////

/*Wrappers for creating message*/
void* wrapperXnapPdu_create(XNAP_XnAP_PDU input){
  XNAP_XnAP_PDU *obj;

  // m = (typeof(m))malloc(sizeof(*m));
  // obj = new XNAP_XnAP_PDU(static_cast<const XNAP_XnAP_PDU&>(input));
  obj = new XNAP_XnAP_PDU(static_cast<const XNAP_XnAP_PDU&>(input));

  // m->obj = obj;

  return obj;
}

int wrapper_match_pdu_type(XNAP_XnAP_PDU xnapPdu) {

    switch (xnapPdu.value->alternative) {
        case XNAP_initiating_message: {      
            return 1;
          }
      }
    return 0; // 0 if unsuccessful
}

int wrapper_check_procedure_code(XNAP_XnAP_PDU xnapPdu) {
  auto initiatingMessage = static_cast<const asncXNAP_XnAP_PDU__initiatingMessage*>(xnapPdu.value); //cast to initiating message type

  switch (initiatingMessage->procedureCode) {
          case XNAP_id_xnSetup__V: {
            return 1;
          }
      }

   return 0; // 0 if unsuccessful
}

const XNAP_XnSetupRequest& wrapper_create_setup_request(XNAP_XnAP_PDU xnapPdu) {
  auto& initiatingMessage = static_cast<const asncXNAP_XnAP_PDU__initiatingMessage&>(*xnapPdu.value); // cast to initiating message type

  return static_cast<const XNAP_XnSetupRequest&>(*initiatingMessage.value); 
}

int wrapper_get_count(const XNAP_XnSetupRequest& xnSetupPdu) {
  return xnSetupPdu.protocolIEs.getCount();
}

const asncXNAP_XnSetupRequest__protocolIEss* wrapper_get_first_protocol_IE(XNAP_XnSetupRequest input){
  auto* ie = static_cast<asncXNAP_XnSetupRequest__protocolIEss*>(input.protocolIEs.getFirstElement());
  return ie;
}

int wrapper_get_element_id(asncXNAP_XnSetupRequest__protocolIEss* input){
  // std::cout << "\nGetting element ID..." << std::endl;
  switch(input->id) {
    case XNAP_id_GlobalNG_RAN_node_ID__V: { 
      return 1; 
    }
    case XNAP_id_TAISupport_list__V: {
      return 2;
    }
    case XNAP_id_List_of_served_cells_NR__V: {
      return 3;
    }
  }
  return 0;
}

const asncXNAP_XnSetupRequest__protocolIEss* wrapper_get_next_protocol_IE(const asncXNAP_XnSetupRequest__protocolIEss* currentProtocolIE){
  return static_cast<asncXNAP_XnSetupRequest__protocolIEss*>(currentProtocolIE->getNextElement());
}
/*End of wrappers for creating message*/

/*Wrappers for global gnb*/
const asncXNAP_GlobalNG_RANNode_ID__gNB& wrapper_get_gnbPdu(asncXNAP_XnSetupRequest__protocolIEss* ie){
  // std::cout << "Casting to asncXNAP_GlobalNG_RANNode_ID__gNB" << std::endl;
  auto& pdu = static_cast<const XNAP_GlobalNG_RANNode_ID&>(*ie->value);
  return static_cast<const asncXNAP_GlobalNG_RANNode_ID__gNB&>(*pdu.value);
}

LengthAndValue* wrapper_getgnbIdPdu_string(asncXNAP_GlobalNG_RANNode_ID__gNB& pdu){
  // std::cout << "Casting to asncXNAP_gnb_ID" << std::endl;
  auto& gnbIdPdu = static_cast<const asncXNAP_gnb_ID&>(*pdu.gnb_id);
  return c_bitString_to_length_value(gnbIdPdu.gnb_ID);
}

LengthAndValue* wrapper_get_plmn_from_global_gnb(asncXNAP_GlobalNG_RANNode_ID__gNB& pdu){
  // std::cout << "Entered get plmn_id from asncXNAP_GlobalNG_RANNode_ID__gNB" << std::endl;
  return c_octetString_to_length_value(pdu.plmn_id);
}

/*End of wrappers for global gnb*/

/////////////////////////////////////////////////////////////////

/*Wrappers for TAI support list*/

const XNAP_TAISupport_List& wrapper_get_tai_list_from_protocol_ie(asncXNAP_XnSetupRequest__protocolIEss* ie){
  return static_cast<const XNAP_TAISupport_List&>(*ie->value);
}

const asncXNAP_TAISupport_Lists* wrapper_get_first_support_item(const XNAP_TAISupport_List& mTaiList){

  return static_cast<asncXNAP_TAISupport_Lists*>(mTaiList.value.getFirstElement());
}

int wrapper_get_support_items_count(const XNAP_TAISupport_List& mTaiList){
  return mTaiList.value.getCount();
}

LengthAndValue* wrapper_get_tai_tac(const asncXNAP_TAISupport_Lists* currentSupportItem){
  return c_octetString_to_length_value(currentSupportItem->tac);
}

int wrapper_get_tai_broadcast_plmn_count(const asncXNAP_TAISupport_Lists* currentSupportItem){
  return currentSupportItem->broadcastPLMNs.getCount();
}

const asncXNAP_TAISupport_Item__broadcastPLMNss* wrapper_get_first_tai_broadcast_plmn(const asncXNAP_TAISupport_Lists* currentSupportItem){
  return static_cast<asncXNAP_TAISupport_Item__broadcastPLMNss*>(currentSupportItem->broadcastPLMNs.getFirstElement());
}

LengthAndValue* wrapper_get_plmn_from_tai(const asncXNAP_TAISupport_Item__broadcastPLMNss* currentBroadcastItem){
  return c_octetString_to_length_value(currentBroadcastItem->plmn_id);
}

const asncXNAP_SliceSupport_Lists* wrapper_get_first_xnapsnssai(const asncXNAP_TAISupport_Item__broadcastPLMNss* mBroadcastItem){
  return static_cast<asncXNAP_SliceSupport_Lists*>(mBroadcastItem->tAISliceSupport_List.getFirstElement());
}

int wrapper_get_xnapsnssai_count(const asncXNAP_TAISupport_Item__broadcastPLMNss* mBroadcastItem){
  return mBroadcastItem->tAISliceSupport_List.getCount();
}

LengthAndValue* wrapper_get_xnapsnssai_sst(const asncXNAP_SliceSupport_Lists* mXnapSNssai){
  return c_octetString_to_length_value(mXnapSNssai->sst);
}

bool wrapper_get_sd_presence(const asncXNAP_SliceSupport_Lists* mXnapSNssai){
  return mXnapSNssai->optional.getPresence(asn_XNAP_sd);
}

LengthAndValue* wrapper_get_xnapsnssai_sd(const asncXNAP_SliceSupport_Lists* mXnapSNssai){
  return c_octetString_to_length_value(mXnapSNssai->sd);
}

const asncXNAP_SliceSupport_Lists* wrapper_get_next_xnapsnssai(const asncXNAP_SliceSupport_Lists* currentXnapSNssai){
  return static_cast<asncXNAP_SliceSupport_Lists*>(currentXnapSNssai->getNextElement());
}

const asncXNAP_TAISupport_Item__broadcastPLMNss* wrapper_get_next_broadcast_item(const asncXNAP_TAISupport_Item__broadcastPLMNss* currentBroadcastItem){
  return static_cast<asncXNAP_TAISupport_Item__broadcastPLMNss*>(currentBroadcastItem->getNextElement());
}

const asncXNAP_TAISupport_Lists* wrapper_get_next_tai_support_item(const asncXNAP_TAISupport_Lists* currentSupportItem){
  return static_cast<asncXNAP_TAISupport_Lists*>(currentSupportItem->getNextElement());
}

/*End of wrappers for TAI support list*/

/////////////////////////////////////////////////////////////////

/*Wrappers for served cells list*/
const XNAP_ServedCells_NR& wrapper_get_list_of_served_cells(const asncXNAP_XnSetupRequest__protocolIEss* ie){
  return static_cast<const XNAP_ServedCells_NR&>(*ie->value);
}

const asncXNAP_ServedCells_NRs* wrapper_get_first_served_cell(const XNAP_ServedCells_NR& list){
  return static_cast<asncXNAP_ServedCells_NRs*>(list.value.getFirstElement());
}

int wrapper_get_cells_count(const XNAP_ServedCells_NR& list){
  return list.value.getCount();
}

int wrapper_get_nr_pci(const asncXNAP_ServedCells_NRs* current){
  return current->served_cell_info_NR->nrPCI;
}

const XNAP_NR_CGI& wrapper_get_cell_id(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return *currentServedCellsItem->served_cell_info_NR->cellID;
}

LengthAndValue* wrapper_get_plmn_from_cell_id(const XNAP_NR_CGI& cell_id){
  return c_octetString_to_length_value(cell_id.plmn_id);
}

LengthAndValue* wrapper_get_nrci(const XNAP_NR_CGI& cell_id){
  return c_bitString_to_length_value(cell_id.nr_CI);
}

LengthAndValue* wrapper_get_cell_tac(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return c_octetString_to_length_value(currentServedCellsItem->served_cell_info_NR->tac);
}

const asncXNAP_BroadcastPLMNss* wrapper_get_first_broadcast_plmn(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return static_cast<asncXNAP_BroadcastPLMNss*>(currentServedCellsItem->served_cell_info_NR->broadcastPLMN.getFirstElement());
}

int wrapper_get_broadcast_plmn_count(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return currentServedCellsItem->served_cell_info_NR->broadcastPLMN.getCount();
}

const asncXNAP_BroadcastPLMNss* wrapper_get_next_broadcast_plmn(const asncXNAP_BroadcastPLMNss* currentBroadcastPLMN){
  return static_cast<asncXNAP_BroadcastPLMNss*>(currentBroadcastPLMN->getNextElement());
}

LengthAndValue* wrapper_get_plmn_from_broadcast_plmn(const asncXNAP_BroadcastPLMNss& broadcast_plmn){ 
  return c_octetString_to_length_value(broadcast_plmn.value);
}

bool wrapper_check_mode_is_tdd(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  if ((currentServedCellsItem->served_cell_info_NR->nrModeInfo->operator==(*(currentServedCellsItem->served_cell_info_NR->nrModeInfo))) ==1) {
     return true;
  } else {
    return false;
  }
}

const XNAP_NRFrequencyInfo* wrapper_get_frequency_info(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return static_cast<asncXNAP_NRModeInfo__tdd*>(currentServedCellsItem->served_cell_info_NR->nrModeInfo)->nrFrequencyInfo;
}

int wrapper_get_nr_arfcn(const XNAP_NRFrequencyInfo* frequency_info){
  return frequency_info->nrARFCN;
}

const asncXNAP_NRFrequencyBand_Lists* wrapper_get_first_band_item(const XNAP_NRFrequencyInfo* frequency_info){
  return static_cast<asncXNAP_NRFrequencyBand_Lists*>(frequency_info->frequencyBand_List.getFirstElement());
}

int wrapper_get_band_items_count(const XNAP_NRFrequencyInfo* frequency_info){
  return frequency_info->frequencyBand_List.getCount();
}

int wrapper_get_frequency_band(const asncXNAP_NRFrequencyBand_Lists* currentBandItem){
  return currentBandItem->nr_frequency_band;
}

const asncXNAP_NRFrequencyBand_Lists* wrapper_get_next_band_item(const asncXNAP_NRFrequencyBand_Lists* currentBandItem){
  return static_cast<asncXNAP_NRFrequencyBand_Lists*>(currentBandItem->getNextElement());
}

const XNAP_NRTransmissionBandwidth* wrapper_get_transmission_bandwidth(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return static_cast<asncXNAP_NRModeInfo__tdd*>(currentServedCellsItem->served_cell_info_NR->nrModeInfo)->nrTransmissonBandwidth;
}

int wrapper_get_nrscs(const XNAP_NRTransmissionBandwidth* mNrTransInfo){
  return mNrTransInfo->nRSCS;
}

int wrapper_get_nrnrb(const XNAP_NRTransmissionBandwidth* mNrTransInfo){
  return mNrTransInfo->nRNRB;
}

LengthAndValue* wrapper_get_measurement_timing_configuration(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return c_octetString_to_length_value(currentServedCellsItem->served_cell_info_NR->measurementTimingConfiguration);
}

int wrapper_get_connectivity_support(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
  return currentServedCellsItem->served_cell_info_NR->connectivitySupport->eNDC_Support;
}

const asncXNAP_ServedCells_NRs* wrapper_get_next_served_cell_item(const asncXNAP_ServedCells_NRs* currentServedCellsItem){
    // std::cout << "Entered get next served cell item" << std::endl;
    return static_cast<asncXNAP_ServedCells_NRs*>(currentServedCellsItem->getNextElement());
}

/*End of wrappers for served cells list*/

/////////////////////////////////////////////////////////////////