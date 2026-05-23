use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commander {
    #[serde(alias = "FID")]
    pub fid: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub enum CommanderPackage {
    Default,
}
