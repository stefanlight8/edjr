use {
    crate::elite::{engineer::Engineer, material::Material, module::ModuleEngineering},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEvent {
    pub commodity: Option<String>,
    #[serde(alias = "Commodity_Localised")]
    pub commodity_display: Option<String>,
    #[serde(flatten)]
    pub engineer: Engineer,
    pub quantity: u64,
    pub total_quantity: u64,
    #[serde(alias = "Type")]
    pub contribution_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEvent {
    pub apply_experimental_effect: Option<String>,
    pub ingredients: Vec<Material>,
    #[serde(flatten)]
    pub engineer: Option<Engineer>,
    #[serde(flatten)]
    pub modification: ModuleEngineering,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressEvent {
    #[serde(flatten)]
    pub engineer: Option<Engineer>,
    pub engineers: Option<Vec<Engineer>>,
}
