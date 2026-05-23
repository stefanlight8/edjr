use serde::Deserialize;

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
    pub count: u64,
    pub reward: u64,
    #[serde(alias = "Name")]
    pub salvage: SalvageType,
    #[serde(alias = "MarketID")]
    pub market_id: u64,
}
