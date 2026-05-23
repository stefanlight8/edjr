use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestockVehicleEvent {
    #[serde(alias = "ID")]
    id: Option<u64>,
    loadout: String,
    cost: u64,
    count: u64,
    #[serde(alias = "Type")]
    vehicle: String,
    #[serde(alias = "Type_Localised")]
    vehicle_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSwitchEvent {
    to: String,
}
