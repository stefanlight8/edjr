use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TouchdownEvent {
    pub system_address: u64,
    pub body: Option<String>,
    #[serde(alias = "BodyID")]
    pub body_id: Option<u64>,
    pub latitude: f64,
    pub longitude: f64,
    pub nearest_destination: Option<String>,
    #[serde(alias = "NearestDestination_Localised")]
    pub nearest_destination_display: Option<String>,
    #[serde(default)]
    pub on_planet: bool,
    #[serde(default)]
    pub on_station: bool,
    #[serde(default)]
    pub player_controlled: bool,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub taxi: bool,
}
