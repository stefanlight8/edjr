use {crate::elite::genus::Genus, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub enum ScanType {
    AutoScan,
    Basic,
    Detailed,
    NavBeaconDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Atmosphere {
    name: String,
    percent: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    ice: f64,
    metal: f64,
    rock: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    name: String,
    percent: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parent {
    null: Option<u64>,
    planet: Option<u64>,
    ring: Option<u64>,
    star: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub enum ReserveLevel {
    CommonResources,
    DepletedResources,
    LowResources,
    MajorResources,
    PristineResources,
}

#[derive(Debug, Deserialize)]
pub enum RingClass {
    #[serde(alias = "eRingClass_Icy")]
    Icy,
    #[serde(alias = "eRingClass_MetalRich")]
    MetalRich,
    #[serde(alias = "eRingClass_Metalic")]
    Metalic,
    #[serde(alias = "eRingClass_Rocky")]
    Rocky,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ring {
    name: String,
    inner_rad: f64,
    outer_rad: f64,
    #[serde(alias = "MassMT")]
    mass_mt: f64,
    ring_class: RingClass,
}

#[derive(Debug, Deserialize)]
pub enum TerraformState {
    Terraformable,
    Terraforming,
    Terraformed,
    #[serde(other)]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Star {
    star_type: String,
    luminosity: String,
    #[serde(alias = "MassEM")]
    mass_em: f64,
    #[serde(alias = "Age_MY")]
    age_my: u64,
    stellar_mass: Option<f64>,
    subclass: Option<u64>, // maybe u8/u16, or enum, because those numbers usually not big in scheme
    absolute_magnitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Planet {
    planet_class: Option<String>,
    atmosphere: Option<String>,
    atmosphere_composition: Option<Vec<Atmosphere>>,
    atmosphere_type: Option<String>,
    landable: Option<bool>,
    materials: Option<Vec<Material>>,
    composition: Option<Composition>,
    reserve_level: Option<ReserveLevel>,
    surface_gravity: Option<f64>,
    surface_pressure: Option<f64>,
    terraform_state: Option<TerraformState>,
    volcanism: Option<String>,
    #[serde(default)]
    was_footfalled: bool,
}

// TODO: maybe planet class enum and move everything
// to elite if its not associated to scan event?
// TODO: separate star/planet/any other object information
// if it's possible
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    scan_type: ScanType,
    body_name: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    star_system: String,
    system_address: u64,
    #[serde(flatten)]
    star: Option<Star>,
    #[serde(flatten)]
    planet: Option<Planet>,
    ascending_node: Option<f64>,
    axial_tilt: Option<f64>,
    #[serde(alias = "DistanceFromArrivalLS")]
    distance_from_arrival_ls: f64,
    eccentricity: Option<f64>,
    mean_anomaly: Option<f64>,
    orbital_inclination: Option<f64>,
    orbital_period: Option<f64>,
    parents: Option<Vec<Parent>>,
    periapsis: Option<f64>,
    rings: Option<Vec<Ring>>,
    rotation_period: Option<f64>,
    surface_temperature: Option<f64>,
    semi_major_axis: Option<f64>,
    radius: Option<f64>,
    #[serde(default)]
    tidal_lock: bool,
    #[serde(default)]
    was_discovered: bool,
    #[serde(default)]
    was_mapped: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanBaryCentreEvent {
    #[serde(alias = "BodyID")]
    body_id: u64,
    ascending_node: f64,
    eccentricity: f64,
    orbital_inclination: f64,
    orbital_period: f64,
    periapsis: f64,
    semi_major_axis: f64,
    star_system: String,
    system_address: u64,
}

#[derive(Debug, Deserialize)]
pub enum OrganicScanType {
    Analyse,
    Log,
    Sample,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanOrganicEvent {
    #[serde(alias = "Body")]
    body_id: u64,
    genus: Genus,
    scan_type: OrganicScanType,
    species: String,
    #[serde(alias = "Species_Localised")]
    species_display: String,
    variant: String,
    #[serde(alias = "Variant_Localised")]
    variant_display: String,
    system_address: u64,
    #[serde(default)]
    was_logged: bool,
}

#[derive(Debug, Deserialize)]
pub enum TargetScanType {
    Cargo,
    Crime,
}

// I don't know where it should be
// because as I remember it's event when
// you're begin scanned, so I can't put it
// here or in target-associated events ig
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScannedEvent {
    scan_type: TargetScanType,
}
