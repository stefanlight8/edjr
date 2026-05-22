use {crate::elite::ship::Ship, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub enum GameMode {
    Open,
    Solo,
    Group,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {
    commander: String,
    credits: u64,
    #[serde(alias = "FID")]
    fid: String,
    fuel_capacity: Option<f64>,
    fuel_level: Option<f64>,
    game_mode: Option<GameMode>,
    group: Option<String>,
    loan: u64,
    #[serde(default)]
    horizons: bool,
    #[serde(default)]
    odyssey: bool,
    #[serde(flatten)]
    ship: Option<Ship>,
    #[serde(default)]
    start_dead: bool,
    #[serde(default)]
    start_landed: bool,
    #[serde(alias = "build")]
    build: Option<String>,
    #[serde(alias = "gameversion")]
    game_version: Option<String>,
    #[serde(alias = "language")]
    language: Option<String>,
}
