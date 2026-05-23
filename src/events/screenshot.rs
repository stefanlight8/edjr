use {serde::Deserialize, std::path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScreenshotEvent {
    filename: PathBuf,
    width: u64,
    height: u64,
    system: Option<String>,
    body: Option<String>,
    altitude: Option<f64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}
