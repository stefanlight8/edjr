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
    pub count: u64,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: Option<String>,
    #[serde(alias = "OwnerID")]
    pub owner_id: u64,
    #[serde(alias = "type")]
    pub backpack_type: Option<BackpackItemType>,
}
