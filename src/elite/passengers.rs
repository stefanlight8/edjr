use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PassengerType {
    Tourist,
    Politician,
}
