use {crate::elite::combat::Killer, serde::Deserialize};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEvent {
    killer_name: Option<String>,
    #[serde(alias = "KillerName_Localised")]
    killer_name_display: Option<String>,
    killer_rank: Option<String>, // lol frontier there uses a string instead of their combat rank...
    killer_ship: Option<String>,
    killers: Option<Vec<Killer>>,
}
