#[cfg(feature = "faction")]
use crate::elite::faction::Faction;
use {
    crate::elite::{allegiance::Allegiance, economy::Economy, goverment::Goverment},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub enum SystemSecurity {
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(alias = "$SYSTEM_SECURITY_low;")]
    Low,
    #[serde(alias = "$SYSTEM_SECURITY_medium;")]
    Medium,
    #[serde(alias = "$SYSTEM_SECURITY_high;")]
    High,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    pub system_allegiance: Option<Allegiance>,
    pub system_economy: Economy,
    pub system_goverment: Goverment,
    pub system_second_economy: Economy,
    pub system_security: SystemSecurity,
    #[cfg(feature = "faction")]
    pub system_faction: Option<Faction>,
}
