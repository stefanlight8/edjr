use {crate::elite::allegiance::Allegiance, serde::Deserialize};
#[derive(Debug, Deserialize)]
pub struct DataScannedEvent {
    #[serde(alias = "Type")]
    data_type: String,
    #[serde(alias = "Type_Localised")]
    data_type_display: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatalinkScanEvent {
    message: String,
    #[serde(alias = "Message_Localised")]
    message_display: String,
}

#[derive(Debug, Deserialize)]
pub struct DatalinkVoucherEvent {
    reward: u64,
    payee_faction: Allegiance,
    victim_faction: Option<Allegiance>,
}
