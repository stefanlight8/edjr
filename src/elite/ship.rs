use {crate::elite::module::ModuleEngineering, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    #[serde(alias = "ShipType")]
    pub ship: String,
    #[serde(default, alias = "Ship_Localised", alias = "ShipType_Localised")]
    pub ship_display: String,
    #[serde(alias = "ShipID")]
    pub ship_id: u64,
    pub ship_ident: Option<String>,
    #[serde(alias = "Name")]
    pub ship_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipModule {
    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: u8,
    pub health: f64,
    pub engineering: Option<ModuleEngineering>,
    pub ammo_in_clip: Option<u64>,
    pub ammo_in_hopper: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}
