use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CrimeType {
    Assault,
    CollidedAtSpeedInNoFireZone,
    #[serde(alias = "collidedAtSpeedInNoFireZone_hulldamage")]
    CollidedAtSpeedInNoFireZoneHullDamage,
    DockingMajorBlockingAirlock,
    DockingMajorBlockingLandingPad,
    DockingMajorTresspass,
    DockingMinorBlockingAirlock,
    DockingMinorBlockingLandingPad,
    DockingMinorTresspass,
    FireInNoFireZone,
    Interdiction,
    Murder,
    #[serde(alias = "onFoot_damagingDefences")]
    OnFootDamagingDefences,
    #[serde(alias = "onFoot_identityTheft")]
    OnFootIdentifyTheft,
    #[serde(alias = "onFoot_murder")]
    OnFootMurder,
    #[serde(alias = "onFoot_trespass")]
    OnFootTrespass,
    RecklessWeaponsDischarge,
    ShuttleDestruction,
}
