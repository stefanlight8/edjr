use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum FriendStatus {
    Added,
    Declined,
    Lost,
    Offline,
    Online,
    Requested,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriendsEvent {
    name: String,
    status: FriendStatus,
}
