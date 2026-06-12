use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    pub faction: String,
    pub reward: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEvent {
    pub pilot_name: String,
    #[serde(alias = "PilotName_Localised")]
    pub pilot_name_display: Option<String>,
    pub rewards: Vec<Reward>,
    pub shared_with_others: Option<u64>,
    pub target: String,
    #[serde(alias = "Target_Localised")]
    pub target_display: Option<String>,
    pub total_reward: u64,
    pub victim_faction: String,
    #[serde(alias = "VictimFaction_Localised")]
    pub victim_faction_display: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayBountiesEvent {
    pub all_fines: bool,
    pub amount: u64,
    pub broker_percentage: Option<f64>,
    pub faction: Option<String>,
    #[serde(alias = "ShipID")]
    pub ship_id: u64,
}
