use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveBodyEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    star_system: String,
    system_address: u64,
}
