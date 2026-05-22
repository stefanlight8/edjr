use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UseConsumableEvent {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: String,
}
