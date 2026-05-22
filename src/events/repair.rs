use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AfmuRepairsEvent {
    fully_repaired: bool,
    health: f64,
    module: String,
    #[serde(alias = "Module_Localised")]
    module_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairEvent {
    cost: u64,
    items: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairAllEvent {
    cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairDroneEvent {
    hull_repaired: f64,
    cockpit_repaired: Option<f64>,
    corrosion_repaired: Option<f64>,
}
