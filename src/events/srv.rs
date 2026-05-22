use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockSrvEvent {
    #[serde(alias = "ID")]
    id: u64,
    #[serde(alias = "SRVType")]
    srv_type: String,
    #[serde(alias = "SRVType_Localised")]
    srv_type_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSrvEvent {
    #[serde(alias = "ID")]
    id: u64,
    loadout: String,
    player_controlled: bool,
    #[serde(alias = "SRVType")]
    srv_type: String,
    #[serde(alias = "SRVType_Localised")]
    srv_type_display: String,
}
