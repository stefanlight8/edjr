use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BodyType {
    Null,
    Planet,
    PlanetaryRing,
    Star,
    Station,
    StellarRing,
    AsteroidCluster,
}
