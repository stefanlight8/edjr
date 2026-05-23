use {
    crate::elite::ship::{FuelCapacity, Ship, ShipModule},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    pub cargo_capacity: u64,
    pub fuel_capacity: FuelCapacity,
    pub hull_health: f64,
    #[serde(default)]
    pub hull_value: u64,
    pub max_jump_range: f64,
    pub modules: Vec<ShipModule>,
    pub modules_value: Option<u64>,
    pub rebuy: u64,
    #[serde(flatten)]
    pub ship: Ship,
    pub unladen_mass: f64,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuitModule {
    pub class: u8,
    pub module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    pub module_name_display: String,
    pub slot_name: String,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<String>, // TODO?: weapon mods, maybe weapons too
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutEvent {
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: Option<String>, // i guess, TODO: check
    pub modules: Vec<SuitModule>,
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<String>, // TODO?: suit mods
    pub suit_name: String,      // TODO: enum
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_display: String,
}
// I need more information to make enums, because I didn't play
// odyssey content a lot tbh

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwitchSuitLoadoutEvent {
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: Option<String>, // i guess, TODO: check
    pub modules: Vec<SuitModule>,
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<String>, // TODO?: suit mods
    pub suit_name: String,      // TODO: enum
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_display: String,
}
