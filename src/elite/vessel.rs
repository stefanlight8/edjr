use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Vessel {
    Ship,
    #[serde(alias = "SRV")]
    Srv,
}
