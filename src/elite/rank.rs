use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Rank {
    Harmless = 0,
    MostlyHarmless = 1,
    Novice = 2,
    Competent = 3,
    Expert = 4,
    Master = 5,
    Dangerous = 6,
    Deadly = 7,
    Elite = 8,
    EliteI = 9,
    EliteII = 10,
    EliteIII = 11,
    EliteIV = 12,
    EliteV = 13,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EmpireRank {
    None = 0,
    Outsider = 1,
    Serf = 2,
    Master = 3,
    Squire = 4,
    Knight = 5,
    Lord = 6,
    Baron = 7,
    Viscount = 8,
    Count = 9,
    Earl = 10,
    Marquis = 11,
    Duke = 12,
    Prince = 13,
    King = 14,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FederationRank {
    None = 0,
    Recruit = 1,
    Cadet = 2,
    Midshipman = 3,
    PettyOfficer = 4,
    ChiefPettyOfficer = 5,
    WarrantOfficer = 6,
    Ensign = 7,
    Lieutenant = 8,
    LieutenantCommander = 9,
    PostCommander = 10,
    PostCaptain = 11,
    RearAdmiral = 12,
    ViceAdmiral = 13,
    Admiral = 14,
}
