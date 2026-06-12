use {
    crate::elite::commander::{Commander, CommanderPackage},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommanderEvent {
    #[serde(flatten)]
    pub commander: Commander,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommanderEvent {
    #[serde(flatten)]
    pub commander: CommanderEvent,
    pub package: CommanderPackage,
}
