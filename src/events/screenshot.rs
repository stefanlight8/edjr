use {serde::Deserialize, std::path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScreenshotEvent {
    pub filename: PathBuf,
    pub width: u64,
    pub height: u64,
    pub system: Option<String>,
    pub body: Option<String>,
    pub altitude: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
