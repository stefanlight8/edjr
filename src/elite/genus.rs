use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Genus {
    #[serde(alias = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(alias = "$Codex_Ent_Brancae_Name;")]
    BrainTrees,
    #[serde(alias = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,
    #[serde(alias = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,
    #[serde(alias = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,
    #[serde(alias = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystallineShards,
    #[serde(alias = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,
    #[serde(alias = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,
    #[serde(alias = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,
    #[serde(alias = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,
    #[serde(alias = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,
    #[serde(alias = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,
    #[serde(alias = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,
}
