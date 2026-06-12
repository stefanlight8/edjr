use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvitedToSquadronEvent {
    pub squadron_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinedSquadronEvent {
    #[serde(alias = "SquadronID")]
    pub squadron_id: u64,
    pub squadron_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftSquadronEvent {
    #[serde(alias = "SquadronID")]
    pub squadron_id: Option<u64>,
    pub squadron_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronCreatedEvent {
    #[serde(alias = "SquadronID")]
    pub squadron_id: u64,
    pub squadron_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronStartupEvent {
    #[serde(alias = "SquadronID")]
    pub squadron_id: Option<u64>,
    pub squadron_name: Option<String>,
    pub current_rank: u64,
    pub current_rank_name: Option<String>,
    #[serde(alias = "CurrentRankName_Localised")]
    pub current_rank_name_display: Option<String>,
}
