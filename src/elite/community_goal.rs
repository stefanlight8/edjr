use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoal {
    #[serde(alias = "CGID")]
    pub cgid: u64,
    pub title: String,
}
