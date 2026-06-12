use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FriendStatus {
    Added,
    Declined,
    Lost,
    Offline,
    Online,
    Requested,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriendsEvent {
    pub name: String,
    pub status: FriendStatus,
}
