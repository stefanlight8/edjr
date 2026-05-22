use {crate::elite::module::ModuleEngineering, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    ship: String,
    #[serde(default, alias = "Ship_Localised")]
    ship_display: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
    ship_ident: Option<String>,
    ship_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipModule {
    slot: String,
    item: String,
    on: bool,
    priority: u8,
    health: f64,
    engineering: Option<ModuleEngineering>,
    ammo_in_clip: Option<u64>,
    ammo_in_hopper: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    main: f64,
    reserve: f64,
}
