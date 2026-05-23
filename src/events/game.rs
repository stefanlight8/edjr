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
    pub commander: String,
    pub credits: u64,
    #[serde(alias = "FID")]
    pub fid: String,
    pub fuel_capacity: Option<f64>,
    pub fuel_level: Option<f64>,
    pub game_mode: Option<GameMode>,
    pub group: Option<String>,
    pub loan: u64,
    #[serde(default)]
    pub horizons: bool,
    #[serde(default)]
    pub odyssey: bool,
    #[serde(flatten)]
    pub ship: Option<Ship>,
    #[serde(default)]
    pub start_dead: bool,
    #[serde(default)]
    pub start_landed: bool,
    #[serde(alias = "build")]
    pub build: Option<String>,
    #[serde(alias = "gameversion")]
    pub game_version: Option<String>,
    #[serde(alias = "language")]
    pub language: Option<String>,
}
