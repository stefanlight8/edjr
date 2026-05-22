use {
    crate::elite::commander::{Commander, CommanderPackage},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct CommanderEvent {
    #[serde(flatten)]
    commander: Commander,
}

#[derive(Debug, Deserialize)]
pub struct NewCommanderEvent {
    #[serde(flatten)]
    commander: CommanderEvent,
    package: CommanderPackage,
}
