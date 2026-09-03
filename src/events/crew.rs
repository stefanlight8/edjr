use {
    crate::elite::crew::CrewRole,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewAssignEvent {
    #[serde(alias = "CrewID")]
    pub crew_id: u64,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewFireEvent {
    #[serde(alias = "CrewID")]
    pub crew_id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewLaunchFighterEvent {
    pub crew: String,
    #[serde(default)]
    pub telepresense: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberJoinsEvent {
    pub crew: String,
    #[serde(default)]
    pub telepresense: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberQuitsEvent {
    pub crew: String,
    #[serde(default)]
    pub telepresense: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberRoleChangeEvent {
    pub crew: String,
    pub role: CrewRole,
    #[serde(default)]
    pub telepresense: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeCrewRoleEvent {
    pub role: CrewRole,
    #[serde(default)]
    pub telepresence: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndCrewSessionEvent {
    pub on_crime: bool,
    #[serde(default)]
    pub telepresence: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinACrewEvent {
    pub captain: String,
    #[serde(default)]
    pub telepresence: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NpcCrewPaidWageEvent {
    pub amount: u64,
    pub npc_crew_id: u64,
    pub npc_crew_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuitACrewEvent {
    pub captain: String,
}
