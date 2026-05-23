use {
    crate::elite::{module::Module, ship::Ship},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    server_id: u64, // ServerId – nicee!
    #[serde(flatten)]
    ship: Ship,
    storage_slot: u64,
    stored_item: String,
    #[serde(alias = "StoredItem_Localised")]
    stored_item_display: Option<String>,
    transfer_cost: u64,
    transfer_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    items: Vec<Module>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    #[serde(flatten)]
    ship: Ship,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleRetrieveEvent {
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
    engineer_modifications: Option<String>,
    level: Option<u8>,
    quality: Option<f64>,
    slot: String,
    // TODO:
    // maybe flatten module, but problem is that
    // otherwise we need to give an alias to module's name
    // `retrieved_item` and that's kinda dirty
    // need to think about it
    retrieved_item: String,
    #[serde(alias = "RetrievedItem_Localised")]
    retrieved_item_display: String,
    swap_out_item: Option<String>,
    #[serde(alias = "SwapOutItem_Localised")]
    swap_out_item_display: Option<String>,
    #[serde(default)]
    hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    sell_item: String,
    #[serde(alias = "SellItem_Localised")]
    sell_item_display: String,
    sell_price: u64,
    #[serde(flatten)]
    ship: Ship,
    slot: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellRemoteEvent {
    server_id: u64,
    sell_item: String,
    #[serde(alias = "SellItem_Localised")]
    sell_item_display: String,
    sell_price: u64,
    #[serde(flatten)]
    ship: Ship,
    storage_slot: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(flatten)]
    ship: Ship,
    engineer_modifications: Option<String>,
    level: Option<u8>,
    quality: Option<f64>,
    slot: String,
    stored_item: String,
    #[serde(alias = "StoredItem_Localised")]
    stored_item_display: String,
    #[serde(default)]
    hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSwapEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    #[serde(flatten)]
    ship: Ship,
    from_slot: String,
    from_item: String,
    #[serde(alias = "FromItem_Localised")]
    from_item_display: String,
    to_slot: String,
    to_item: Option<String>,
    #[serde(alias = "ToItem_Localised")]
    to_item_display: Option<String>,
    #[serde(default)]
    hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    buy_item: String,
    #[serde(alias = "BuyItem_Localised")]
    buy_item_display: String,
    buy_price: u64,
    sell_item: Option<String>,
    #[serde(alias = "SellItem_Localised")]
    sell_item_display: Option<String>,
    sell_price: Option<u64>,
    #[serde(flatten)]
    ship: Ship,
    slot: String,
    stored_item: Option<String>,
    #[serde(alias = "StoredItem_Localised")]
    stored_item_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyAndStoreEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    buy_item: String,
    #[serde(alias = "BuyItem_Localised")]
    buy_item_display: String,
    buy_price: u64,
    #[serde(flatten)]
    ship: Ship,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteModule {
    #[serde(flatten)]
    module: Module,
    storage_slot: u64,
    buy_price: u64,
    transfer_cost: Option<u64>,
    transfer_time: Option<f64>,
    star_system: Option<String>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    #[serde(default)]
    in_transit: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    star_system: String,
    station_name: String,
    items: Vec<RemoteModule>,
}
