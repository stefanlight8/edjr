use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BodyType {
    Null,
    Planet,
    PlanetaryRing,
    Star,
    Station,
    StellarRing,
    AsteroidCluster,
}
