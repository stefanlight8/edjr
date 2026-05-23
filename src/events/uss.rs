use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UssDropEvent {
    #[serde(alias = "USSThreat")]
    pub uss_threat: u8,
    #[serde(alias = "USSType")]
    pub uss_type: String,
    #[serde(alias = "USSType_Localised")]
    pub uss_type_display: String,
}
