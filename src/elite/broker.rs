use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrokerType {
    Guardian,
    Salvation,
    Sirius,
}
