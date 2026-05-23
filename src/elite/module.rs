use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: String,
    engineer_modifications: Option<String>,
    level: Option<u8>,
    quality: Option<f64>,
    #[serde(default)]
    hot: bool,
    slot: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    label: String,
    value: Option<f64>,
    original_value: Option<f64>,
    less_is_good: Option<u8>,
    value_str: Option<String>,
    #[serde(alias = "ValueStr_Localised")]
    value_str_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleEngineering {
    #[serde(alias = "BlueprintID")]
    blueprint_id: Option<u64>,
    blueprint_name: String,
    level: u8,
    modifier: Option<Vec<Modifier>>,
    engineer: Option<String>,
    #[serde(alias = "EngineerID")]
    engineer_id: Option<u64>,
    experimental_effect: Option<String>,
    #[serde(alias = "ExperimentalEffect_Localised")]
    experimental_effect_display: Option<String>,
}
