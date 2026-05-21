use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoal {
    #[serde(alias = "CGID")]
    cgid: u64,
    title: String,
}
