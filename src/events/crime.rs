use {crate::elite::crime::CrimeType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    crime_type: CrimeType,
    faction: String,
    bounty: Option<u64>,
    fine: Option<u64>,
    victim: Option<String>,
    #[serde(alias = "Victim_Localised")]
    victim_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    crime_type: CrimeType,
    offender: String,
    bounty: Option<u64>,
    fine: Option<u64>,
}
