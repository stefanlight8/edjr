use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    faction: String,
    reward: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEvent {
    pilot_name: String,
    #[serde(alias = "PilotName_Localised")]
    pilot_name_display: Option<String>,
    rewards: Vec<Reward>,
    shared_with_others: Option<u64>,
    target: String,
    #[serde(alias = "Target_Localised")]
    target_display: Option<String>,
    total_reward: u64,
    victim_faction: String,
    #[serde(alias = "VictimFaction_Localised")]
    victim_faction_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayBountiesEvent {
    all_fines: bool,
    amount: u64,
    broker_percentage: Option<f64>,
    faction: Option<String>,
    #[serde(alias = "ShipID")]
    ship_id: u64,
}
