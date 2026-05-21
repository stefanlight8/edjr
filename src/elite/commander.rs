use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commander {
    #[serde(alias = "FID")]
    fid: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub enum CommanderPackage {
    Default,
}
