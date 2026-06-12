use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commander {
    #[serde(alias = "FID")]
    pub fid: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommanderPackage {
    Default,
}
