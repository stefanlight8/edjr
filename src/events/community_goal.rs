use {
    crate::elite::community_goal::CommunityGoal,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEvent {
    pub current_goals: Vec<CommunityGoal>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalDiscardEvent {
    #[serde(alias = "CGID")]
    pub cgid: u64,
    pub name: String,
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalJoinEvent {
    #[serde(alias = "CGID")]
    pub cgid: u64,
    pub name: String,
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalRewardEvent {
    #[serde(alias = "CGID")]
    pub cgid: u64,
    pub name: String,
    pub reward: u64,
    pub system: String,
}
