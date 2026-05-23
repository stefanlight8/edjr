use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CodexEntryCategory {
    #[serde(alias = "$Codex_Category_StellarBodies;")]
    StellarBodies,
    #[serde(alias = "$Codex_Category_Biology;")]
    Biology, // Biological and Geological
    #[serde(alias = "$Codex_Category_Civilisations;")]
    Civilisations,
}
