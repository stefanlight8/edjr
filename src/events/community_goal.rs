use {crate::elite::community_goal::CommunityGoal, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityGoalEvent {
    current_goals: Vec<CommunityGoal>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityGoalDiscardEvent {
    #[serde(alias = "CGID")]
    cgid: u64,
    name: String,
    system: String,
}

#[derive(Debug, Deserialize)]
pub struct CommunityGoalJoinEvent {
    #[serde(alias = "CGID")]
    cgid: u64,
    name: String,
    system: String,
}

#[derive(Debug, Deserialize)]
pub struct CommunityGoalRewardEvent {
    #[serde(alias = "CGID")]
    cgid: u64,
    name: String,
    reward: u64,
    system: String,
}
