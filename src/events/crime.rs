use {crate::elite::crime::CrimeType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    pub crime_type: CrimeType,
    pub faction: String,
    pub bounty: Option<u64>,
    pub fine: Option<u64>,
    pub victim: Option<String>,
    #[serde(alias = "Victim_Localised")]
    pub victim_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    pub crime_type: CrimeType,
    pub offender: String,
    pub bounty: Option<u64>,
    pub fine: Option<u64>,
}
