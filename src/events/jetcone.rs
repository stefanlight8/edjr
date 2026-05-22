use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeBoostEvent {
    boost_value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    module: String,
    #[serde(alias = "Module_Localised")]
    module_display: String,
}
