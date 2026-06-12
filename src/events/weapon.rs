use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyWeaponEvent {
    pub class: u64,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
    pub price: u64,
    #[serde(alias = "SuitModuleID")]
    pub suit_module_id: u64,
    // TODO: WeaponMods
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SellWeaponEvent {
    pub class: u64,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
    pub price: u64,
    #[serde(alias = "SuitModuleID")]
    pub suit_module_id: u64,
    // TODO: WeaponMods
}
