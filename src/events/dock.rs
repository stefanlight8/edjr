use {
    crate::elite::station::{Station, StationType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LandingPads {
    pub large: u64,
    pub medium: u64,
    pub small: u64,
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
    pub dist_from_star_ls: Option<f64>,
    pub landing_pads: Option<LandingPads>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub star_system: String,
    #[serde(flatten)]
    pub station: Option<Station>,
    pub system_address: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub active_fine: bool,
    #[serde(default)]
    pub cockpit_breach: bool,
    #[serde(default)]
    pub taxi: bool,
    #[serde(default)]
    pub wanted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelledEvent {
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: String,
    pub station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGrantedEvent {
    pub landing_pad: u64,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: String,
    pub station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDeniedEvent {
    pub reason: Reason,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: String,
    pub station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    pub landing_pads: Option<LandingPads>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: String,
    pub station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeoutEvent {
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: String,
    pub station_type: StationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UndockedEvent {
    pub station_name: String,
    pub station_type: StationType,
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub taxi: bool,
}
