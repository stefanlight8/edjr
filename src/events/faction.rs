use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionKillBondEvent {
    awarding_faction: String,
    #[serde(alias = "AwardingFaction_Localised")]
    awarding_faction_display: Option<String>,
    reward: u64,
    victim_faction: String,
    #[serde(alias = "VictimFaction_Localised")]
    victim_faction_display: Option<String>,
}
