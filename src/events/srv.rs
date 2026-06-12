use {
    crate::elite::srv::SrvType,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockSrvEvent {
    #[serde(alias = "ID")]
    pub id: u64,
    #[serde(alias = "SRVType")]
    pub srv_type: SrvType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSrvEvent {
    #[serde(alias = "ID")]
    pub id: u64,
    pub loadout: String,
    pub player_controlled: bool,
    #[serde(alias = "SRVType")]
    pub srv_type: SrvType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SrvDestroyedEvent {
    #[serde(alias = "ID")]
    pub id: u64,
    #[serde(alias = "SRVType")]
    pub srv_type: SrvType,
}
