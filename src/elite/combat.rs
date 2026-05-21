use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Killer {
    name: String,
    ship: String,
    rank: String,
}
