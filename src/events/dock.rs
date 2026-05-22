use {
    crate::elite::station::{Station, StationType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LandingPads {
    large: u64,
    medium: u64,
    small: u64,
}

#[derive(Debug, Deserialize)]
pub enum Reason {
    Distance,
    Hostile,
    NoReason,
    NoSpace,
    Offences,
    RestrictedAccess,
    TooLarge,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockedEvent {
    dist_from_star_ls: Option<f64>,
    landing_pads: Option<LandingPads>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    star_system: String,
    #[serde(flatten)]
    station: Option<Station>,
    system_address: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    active_fine: bool,
    #[serde(default)]
    cockpit_breach: bool,
    #[serde(default)]
    taxi: bool,
    #[serde(default)]
    wanted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelledEvent {
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGrantedEvent {
    landing_pad: u64,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDeniedEvent {
    reason: Reason,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    landing_pads: Option<LandingPads>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeoutEvent {
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UndockedEvent {
    station_name: String,
    station_type: StationType,
    #[serde(alias = "MarketID")]
    market_id: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    taxi: bool,
}
