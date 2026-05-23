use {
    crate::elite::{allegiance::Allegiance, genus::Genus},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataScannedEvent {
    #[serde(alias = "Type")]
    data_type: String,
    #[serde(alias = "Type_Localised")]
    data_type_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkScanEvent {
    message: String,
    #[serde(alias = "Message_Localised")]
    message_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkVoucherEvent {
    reward: u64,
    payee_faction: Allegiance,
    victim_faction: Option<Allegiance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Discover {
    num_bodies: u64,
    system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEvent {
    base_value: u64,
    bonus: u64,
    discovered: Vec<Discover>,
    total_earnings: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BioData {
    genus: Genus,
    species: String,
    #[serde(alias = "Species_Localised")]
    species_display: String,
    variant: String,
    #[serde(alias = "Variant_Localised")]
    variant_display: String,
    value: u64,
    bonus: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SellfOrganicDataEvent {
    #[serde(alias = "BioData")] // TODO: alias or bio_data?
    biodata: Vec<BioData>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
}
