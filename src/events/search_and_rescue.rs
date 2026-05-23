use {serde::Deserialize, std::path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SalvageType {
    DamagedEscapePod,
    OccupiedCryoPod,
    UssCargoBlackBox,
    WreckageComponents,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchAndRescueEvent {
    count: u64,
    reward: u64,
    #[serde(alias = "Name")]
    salvage: SalvageType,
    #[serde(alias = "MarketID")]
    market_id: u64,
}
