use {
    crate::elite::{engineer::Engineer, material::Material, module::ModuleEngineering},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEvent {
    commodity: Option<String>,
    #[serde(alias = "Commodity_Localised")]
    commodity_display: Option<String>,
    #[serde(flatten)]
    engineer: Engineer,
    quantity: u64,
    total_quantity: u64,
    #[serde(alias = "Type")]
    contribution_type: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEvent {
    apply_experimental_effect: Option<String>,
    ingredients: Vec<Material>,
    #[serde(flatten)]
    engineer: Option<Engineer>,
    #[serde(flatten)]
    modification: ModuleEngineering,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressEvent {
    #[serde(flatten)]
    engineer: Option<Engineer>,
    engineers: Option<Vec<Engineer>>,
}
