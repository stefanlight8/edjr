use {
    crate::elite::{allegiance::Allegiance, rank::Rank},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdictionEvent {
    pub interdictor: String,
    #[serde(alias = "Interdictor_Localised")]
    pub interdictor_display: Option<String>,
    pub is_player: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictedEvent {
    pub interdictor: String,
    #[serde(alias = "Interdictor_Localised")]
    pub interdictor_display: Option<String>,
    pub is_player: bool,
    pub submitted: bool,
    pub combat_rank: Option<Rank>,
    pub faction: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictionEvent {
    pub is_player: bool,
    pub success: bool,
    pub power: Option<Allegiance>,
    pub inderdicted: Option<String>,
    pub combat_rank: Option<Rank>,
    pub faction: Option<String>,
}
