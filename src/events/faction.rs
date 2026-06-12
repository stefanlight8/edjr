use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionKillBondEvent {
    pub awarding_faction: String,
    #[serde(alias = "AwardingFaction_Localised")]
    pub awarding_faction_display: Option<String>,
    pub reward: u64,
    pub victim_faction: String,
    #[serde(alias = "VictimFaction_Localised")]
    pub victim_faction_display: Option<String>,
}
