use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Allegiance {
    PilotsFederation,
    Alliance,
    Empire,
    Federation,
    Independent,
    Guardian,
    Thargoid,
    #[serde(other)]
    None,
    // this could be problematic if there's an other allegiance about we don't know since then it will be ignored and just none
    // this needed by frontier who could make a none value, but make an empty string in DatalinkVoucher in VictimFaction field
}
