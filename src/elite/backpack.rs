use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BackpackItemType {
    Consumable,
    Component,
    Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackItem {
    count: u64,
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    #[serde(alias = "OwnerID")]
    owner_id: u64,
    #[serde(alias = "type")]
    backpack_type: Option<BackpackItemType>,
}
