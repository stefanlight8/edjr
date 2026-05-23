use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
    pub engineer_modifications: Option<String>,
    pub level: Option<u8>,
    pub quality: Option<f64>,
    #[serde(default)]
    pub hot: bool,
    pub slot: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    pub label: String,
    pub value: Option<f64>,
    pub original_value: Option<f64>,
    pub less_is_good: Option<u8>,
    pub value_str: Option<String>,
    #[serde(alias = "ValueStr_Localised")]
    pub value_str_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleEngineering {
    #[serde(alias = "BlueprintID")]
    pub blueprint_id: Option<u64>,
    pub blueprint_name: String,
    pub level: u8,
    pub modifier: Option<Vec<Modifier>>,
    pub engineer: Option<String>,
    #[serde(alias = "EngineerID")]
    pub engineer_id: Option<u64>,
    pub experimental_effect: Option<String>,
    #[serde(alias = "ExperimentalEffect_Localised")]
    pub experimental_effect_display: Option<String>,
}
