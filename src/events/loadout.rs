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
