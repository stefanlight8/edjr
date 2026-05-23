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
    pub name: String,
    pub percent: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    pub ice: f64,
    pub metal: f64,
    pub rock: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub percent: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parent {
    pub null: Option<u64>,
    pub planet: Option<u64>,
    pub ring: Option<u64>,
    pub star: Option<u64>,
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
    pub name: String,
    pub inner_rad: f64,
    pub outer_rad: f64,
    #[serde(alias = "MassMT")]
    pub mass_mt: f64,
    pub ring_class: RingClass,
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
    pub star_type: String,
    pub luminosity: String,
    #[serde(alias = "MassEM")]
    pub mass_em: f64,
    #[serde(alias = "Age_MY")]
    pub age_my: u64,
    pub stellar_mass: Option<f64>,
    pub subclass: Option<u64>, // maybe u8/u16, or enum, because those numbers usually not big in scheme
    pub absolute_magnitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Planet {
    pub planet_class: Option<String>,
    pub atmosphere: Option<String>,
    pub atmosphere_composition: Option<Vec<Atmosphere>>,
    pub atmosphere_type: Option<String>,
    pub landable: Option<bool>,
    pub materials: Option<Vec<Material>>,
    pub composition: Option<Composition>,
    pub reserve_level: Option<ReserveLevel>,
    pub surface_gravity: Option<f64>,
    pub surface_pressure: Option<f64>,
    pub terraform_state: Option<TerraformState>,
    pub volcanism: Option<String>,
    #[serde(default)]
    pub was_footfalled: bool,
}

// TODO: maybe planet class enum and move everything
// to elite if its not associated to scan event?
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    pub scan_type: ScanType,
    pub body_name: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub star_system: String,
    pub system_address: u64,
    #[serde(flatten)]
    pub star: Option<Star>,
    #[serde(flatten)]
    pub planet: Option<Planet>,
    pub ascending_node: Option<f64>,
    pub axial_tilt: Option<f64>,
    #[serde(alias = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    pub eccentricity: Option<f64>,
    pub mean_anomaly: Option<f64>,
    pub orbital_inclination: Option<f64>,
    pub orbital_period: Option<f64>,
    pub parents: Option<Vec<Parent>>,
    pub periapsis: Option<f64>,
    pub rings: Option<Vec<Ring>>,
    pub rotation_period: Option<f64>,
    pub surface_temperature: Option<f64>,
    pub semi_major_axis: Option<f64>,
    pub radius: Option<f64>,
    #[serde(default)]
    pub tidal_lock: bool,
    #[serde(default)]
    pub was_discovered: bool,
    #[serde(default)]
    pub was_mapped: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanBaryCentreEvent {
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub ascending_node: f64,
    pub eccentricity: f64,
    pub orbital_inclination: f64,
    pub orbital_period: f64,
    pub periapsis: f64,
    pub semi_major_axis: f64,
    pub star_system: String,
    pub system_address: u64,
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
    pub body_id: u64,
    pub genus: Genus,
    pub scan_type: OrganicScanType,
    pub species: String,
    #[serde(alias = "Species_Localised")]
    pub species_display: String,
    pub variant: String,
    #[serde(alias = "Variant_Localised")]
    pub variant_display: String,
    pub system_address: u64,
    #[serde(default)]
    pub was_logged: bool,
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
    pub scan_type: TargetScanType,
}
