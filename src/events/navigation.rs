use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavBeaconScanEvent {
    pub num_bodies: u64,
    pub system_address: u64,
}
