use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyWeaponEvent {
    class: u64,
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: String,
    price: u64,
    #[serde(alias = "SuitModuleID")]
    suit_module_id: u64,
    // TODO: WeaponMods
}
