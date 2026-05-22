use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvitedToSquadronEvent {
    squadron_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinedSquadronEvent {
    #[serde(alias = "SquadronID")]
    squadron_id: u64,
    squadron_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftSquadronEvent {
    #[serde(alias = "SquadronID")]
    squadron_id: Option<u64>,
    squadron_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronCreatedEvent {
    #[serde(alias = "SquadronID")]
    squadron_id: u64,
    squadron_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronStartupEvent {
    #[serde(alias = "SquadronID")]
    squadron_id: Option<u64>,
    squadron_name: Option<String>,
    current_rank: u64,
    current_rank_name: Option<String>,
    #[serde(alias = "CurrentRankName_Localised")]
    current_rank_name_display: Option<String>,
}
