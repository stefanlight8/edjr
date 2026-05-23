use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AfmuRepairsEvent {
    pub fully_repaired: bool,
    pub health: f64,
    pub module: String,
    #[serde(alias = "Module_Localised")]
    pub module_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairEvent {
    pub cost: u64,
    pub items: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairAllEvent {
    pub cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepairDroneEvent {
    pub hull_repaired: f64,
    pub cockpit_repaired: Option<f64>,
    pub corrosion_repaired: Option<f64>,
}
