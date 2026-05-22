use {
    crate::elite::{
        allegiance::Allegiance, economy::Economy, faction::Faction, goverment::Goverment,
    },
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
    system_allegiance: Option<Allegiance>,
    system_economy: Economy,
    system_faction: Option<Faction>,
    system_goverment: Goverment,
    system_second_economy: Economy,
    system_security: SystemSecurity,
}
