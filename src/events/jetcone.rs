use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeBoostEvent {
    pub boost_value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    pub module: String,
    #[serde(alias = "Module_Localised")]
    pub module_display: String,
}
