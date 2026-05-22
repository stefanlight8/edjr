use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BookTaxiEvent {
    cost: u64,
    destination_location: String,
    destination_system: String,
}
