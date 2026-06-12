use {
    crate::elite::combat::Killer,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEvent {
    pub killer_name: Option<String>,
    #[serde(alias = "KillerName_Localised")]
    pub killer_name_display: Option<String>,
    pub killer_rank: Option<String>, // lol frontier there uses a string instead of their combat rank...
    pub killer_ship: Option<String>,
    pub killers: Option<Vec<Killer>>,
}
