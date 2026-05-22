use {crate::elite::crew::CrewRole, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewAssignEvent {
    #[serde(alias = "CrewID")]
    crew_id: u64,
    name: String,
    role: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewLaunchFighterEvent {
    crew: String,
    #[serde(default)]
    telepresense: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberJoinsEvent {
    crew: String,
    #[serde(default)]
    telepresense: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberQuitsEvent {
    crew: String,
    #[serde(default)]
    telepresense: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberRoleChangeEvent {
    crew: String,
    role: CrewRole,
    #[serde(default)]
    telepresense: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeCrewRoleEvent {
    role: CrewRole,
    #[serde(default)]
    telepresence: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndCrewSessionEvent {
    on_crime: bool,
    #[serde(default)]
    telepresence: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinACrewEvent {
    captain: String,
    #[serde(default)]
    telepresence: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NpcCrewPaidWageEvent {
    amount: u64,
    npc_crew_id: u64,
    npc_crew_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuitACrewEvent {
    captain: String,
}
