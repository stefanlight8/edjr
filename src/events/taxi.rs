use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BookTaxiEvent {
    pub cost: u64,
    pub destination_location: String,
    pub destination_system: String,
}
