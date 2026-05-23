use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UseConsumableEvent {
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
}
