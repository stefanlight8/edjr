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
    channel: Channel,
    from: String,
    #[serde(alias = "From_Localised")]
    from_display: Option<String>,
    message: String,
    #[serde(alias = "Message_Localised")]
    message_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendTextEvent {
    message: String,
    sent: bool,
    to: String,
    #[serde(alias = "To_Localised")]
    to_display: Option<String>,
}
