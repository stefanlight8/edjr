use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Local,
    Npc,
    Player,
    StarSystem,
    Wing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTextEvent {
    pub channel: Channel,
    pub from: String,
    #[serde(alias = "From_Localised")]
    pub from_display: Option<String>,
    pub message: String,
    #[serde(alias = "Message_Localised")]
    pub message_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendTextEvent {
    pub message: String,
    pub sent: bool,
    pub to: String,
    #[serde(alias = "To_Localised")]
    pub to_display: Option<String>,
}
