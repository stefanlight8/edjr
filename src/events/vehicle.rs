use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestockVehicleEvent {
    #[serde(alias = "ID")]
    pub id: Option<u64>,
    pub loadout: String,
    pub cost: u64,
    pub count: u64,
    #[serde(alias = "Type")]
    pub vehicle: String,
    #[serde(alias = "Type_Localised")]
    pub vehicle_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSwitchEvent {
    pub to: String,
}
