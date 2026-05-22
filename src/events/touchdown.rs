use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TouchdownEvent {
    system_address: u64,
    body: Option<String>,
    #[serde(alias = "BodyID")]
    body_id: Option<u64>,
    latitude: f64,
    longitude: f64,
    nearest_destination: Option<String>,
    #[serde(alias = "NearestDestination_Localised")]
    nearest_destination_display: Option<String>,
    #[serde(default)]
    on_planet: bool,
    #[serde(default)]
    on_station: bool,
    #[serde(default)]
    player_controlled: bool,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    taxi: bool,
}
