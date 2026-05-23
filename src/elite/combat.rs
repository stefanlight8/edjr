use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Killer {
    pub name: String,
    pub ship: String,
    pub rank: String,
}
