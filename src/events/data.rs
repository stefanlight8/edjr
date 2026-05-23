use {
    crate::elite::{allegiance::Allegiance, genus::Genus},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataScannedEvent {
    #[serde(alias = "Type")]
    pub data_type: String,
    #[serde(alias = "Type_Localised")]
    pub data_type_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkScanEvent {
    pub message: String,
    #[serde(alias = "Message_Localised")]
    pub message_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkVoucherEvent {
    pub reward: u64,
    pub payee_faction: Allegiance,
    pub victim_faction: Option<Allegiance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Discover {
    pub num_bodies: u64,
    pub system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEvent {
    pub base_value: u64,
    pub bonus: u64,
    pub discovered: Vec<Discover>,
    pub total_earnings: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BioData {
    pub genus: Genus,
    pub species: String,
    #[serde(alias = "Species_Localised")]
    pub species_display: String,
    pub variant: String,
    #[serde(alias = "Variant_Localised")]
    pub variant_display: String,
    pub value: u64,
    pub bonus: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SellfOrganicDataEvent {
    #[serde(alias = "BioData")] // TODO: alias or bio_data?
    pub biodata: Vec<BioData>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
}
