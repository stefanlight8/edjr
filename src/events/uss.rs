use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UssDropEvent {
    #[serde(alias = "USSThreat")]
    uss_threat: u8,
    #[serde(alias = "USSType")]
    uss_type: String,
    #[serde(alias = "USSType_Localised")]
    uss_type_display: String,
}
