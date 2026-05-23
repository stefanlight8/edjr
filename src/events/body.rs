use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveBodyEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub star_system: String,
    pub system_address: u64,
}
