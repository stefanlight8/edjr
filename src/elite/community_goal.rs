use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoal {
    #[serde(alias = "CGID")]
    pub cgid: u64,
    pub title: String,
}
