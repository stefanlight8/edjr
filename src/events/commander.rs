use {
    crate::elite::commander::{Commander, CommanderPackage},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommanderEvent {
    #[serde(flatten)]
    commander: Commander,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommanderEvent {
    #[serde(flatten)]
    commander: CommanderEvent,
    package: CommanderPackage,
}
