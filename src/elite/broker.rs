use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrokerType {
    Guardian,
    Salvation,
    Sirius,
}
