use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum PassengerType {
    Tourist,
    Politician,
}
