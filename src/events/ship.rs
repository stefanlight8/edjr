use {crate::elite::ship::Ship, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetUserShipNameEvent {
    #[serde(flatten)]
    pub ship: Ship,
    pub user_ship_id: String,
    pub user_ship_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(alias = "Name", alias = "Type")]
    pub name: String,
    #[serde(alias = "Name_Localised", alias = "Type_Localised")]
    pub name_display: Option<String>,
    pub count: Option<u64>,
    #[serde(alias = "OwnerID")]
    pub owner_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEvent {
    pub components: Option<Vec<Cargo>>,
    pub consumables: Option<Vec<Cargo>>,
    pub data: Option<Vec<Cargo>>,
    pub items: Option<Vec<Cargo>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShip {
    #[serde(flatten)]
    pub ship: Ship,
    pub value: u64,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteShip {
    #[serde(flatten)]
    pub ship: Ship,
    pub value: u64,
    pub transfer_cost: Option<u64>,
    pub transfer_time: Option<f64>,
    pub star_system: Option<String>,
    #[serde(alias = "ShipMarketID")]
    pub market_id: Option<u64>,
    #[serde(default)]
    pub in_transit: bool,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipsEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub ships_here: Vec<StoredShip>,
    pub ships_remote: Vec<RemoteShip>,
    pub star_system: String,
    pub station_name: String,
}
