use {
    crate::elite::{allegiance::Allegiance, rank::Rank},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdictionEvent {
    interdictor: String,
    #[serde(alias = "Interdictor_Localised")]
    interdictor_display: Option<String>,
    is_player: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictedEvent {
    interdictor: String,
    #[serde(alias = "Interdictor_Localised")]
    interdictor_display: Option<String>,
    is_player: bool,
    submitted: bool,
    combat_rank: Option<Rank>,
    faction: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictionEvent {
    is_player: bool,
    success: bool,
    power: Option<Allegiance>,
    inderdicted: Option<String>,
    combat_rank: Option<Rank>,
    faction: Option<String>,
}
