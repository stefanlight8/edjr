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
