use {
    crate::elite::ship::{FuelCapacity, Ship, ShipModule},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    cargo_capacity: u64,
    fuel_capacity: FuelCapacity,
    #[serde(default)]
    hot: bool,
    hull_health: f64,
    #[serde(default)]
    hull_value: u64,
    max_jump_range: f64,
    modules: Vec<ShipModule>,
    #[serde(default)]
    modules_value: u64,
    rebuy: u64,
    #[serde(flatten)]
    ship: Ship,
    unladen_mass: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuitModule {
    class: u8,
    module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    module_name_display: String,
    slot_name: String,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    weapon_mods: Vec<String>, // TODO?: weapon mods, maybe weapons too
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutEvent {
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: Option<String>, // i guess, TODO: check
    modules: Vec<SuitModule>,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_mods: Vec<String>, // TODO?: suit mods
    suit_name: String,      // TODO: enum
    #[serde(rename = "SuitName_Localised")]
    suit_name_display: String,
}
// I need more information to make enums, because I didn't play
// odyssey content a lot tbh

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwitchSuitLoadoutEvent {
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: Option<String>, // i guess, TODO: check
    modules: Vec<SuitModule>,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_mods: Vec<String>, // TODO?: suit mods
    suit_name: String,      // TODO: enum
    #[serde(rename = "SuitName_Localised")]
    suit_name_display: String,
}
