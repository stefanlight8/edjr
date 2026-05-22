use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavBeaconScanEvent {
    num_bodies: u64,
    system_address: u64,
}
