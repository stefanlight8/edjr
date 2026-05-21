use {
    crate::elite::{allegiance::Allegiance, faction::Faction},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    system_allegiance: Option<Allegiance>,
    system_economy: String,
    #[serde(alias = "SystemEconomy_Localised")]
    system_economy_display: String,
    system_faction: Option<Faction>,
    system_goverment: String,
    #[serde(alias = "SystemGoverment_Localised")]
    system_goverment_display: String,
    system_second_economy: String,
    #[serde(alias = "SystemSecondEconomy_Localised")]
    system_second_economy_display: String,
    system_security: String,
    #[serde(alias = "SystemSecurity_Localised")]
    system_security_display: String,
}
