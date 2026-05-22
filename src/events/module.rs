use {crate::elite::module::Module, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    server_id: u64, // ServerId – nicee!
    ship: String,
    #[serde(alias = "ShipID")] // ShipID – whyyy...
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
    storage_slot: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
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
    ship: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
}
