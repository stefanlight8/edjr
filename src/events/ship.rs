use {crate::elite::ship::Ship, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetUserShipNameEvent {
    #[serde(flatten)]
    ship: Ship,
    user_ship_id: String,
    user_ship_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(alias = "Name", alias = "Type")]
    name: String,
    #[serde(alias = "Name_Localised", alias = "Type_Localised")]
    name_display: Option<String>,
    count: Option<u64>,
    #[serde(alias = "OwnerID")]
    owner_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEvent {
    components: Option<Vec<Cargo>>,
    consumables: Option<Vec<Cargo>>,
    data: Option<Vec<Cargo>>,
    items: Option<Vec<Cargo>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShip {
    #[serde(flatten)]
    ship: Ship,
    value: u64,
    #[serde(default)]
    hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteShip {
    #[serde(flatten)]
    ship: Ship,
    value: u64,
    transfer_cost: Option<u64>,
    transfer_time: Option<f64>,
    star_system: Option<String>,
    #[serde(alias = "ShipMarketID")]
    market_id: Option<u64>,
    #[serde(default)]
    in_transit: bool,
    #[serde(default)]
    hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipsEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    ships_here: Vec<StoredShip>,
    ships_remote: Vec<RemoteShip>,
    star_system: String,
    station_name: String,
}
