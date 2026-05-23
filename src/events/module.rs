use {
    crate::elite::{module::Module, ship::Ship},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    pub server_id: u64, // ServerId – nicee!
    #[serde(flatten)]
    pub ship: Ship,
    pub storage_slot: u64,
    pub stored_item: String,
    #[serde(alias = "StoredItem_Localised")]
    pub stored_item_display: Option<String>,
    pub transfer_cost: u64,
    pub transfer_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    pub items: Vec<Module>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(flatten)]
    pub ship: Ship,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleRetrieveEvent {
    pub ship: String,
    #[serde(alias = "ShipID")]
    pub ship_id: u64,
    pub engineer_modifications: Option<String>,
    pub level: Option<u8>,
    pub quality: Option<f64>,
    pub slot: String,
    // TODO:
    // maybe flatten module, but problem is that
    // otherwise we need to give an alias to module's name
    // `retrieved_item` and that's kinda dirty
    // need to think about it
    pub retrieved_item: String,
    #[serde(alias = "RetrievedItem_Localised")]
    pub retrieved_item_display: String,
    pub swap_out_item: Option<String>,
    #[serde(alias = "SwapOutItem_Localised")]
    pub swap_out_item_display: Option<String>,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub sell_item: String,
    #[serde(alias = "SellItem_Localised")]
    pub sell_item_display: String,
    pub sell_price: u64,
    #[serde(flatten)]
    pub ship: Ship,
    pub slot: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellRemoteEvent {
    pub server_id: u64,
    pub sell_item: String,
    #[serde(alias = "SellItem_Localised")]
    pub sell_item_display: String,
    pub sell_price: u64,
    #[serde(flatten)]
    pub ship: Ship,
    pub storage_slot: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(flatten)]
    pub ship: Ship,
    pub engineer_modifications: Option<String>,
    pub level: Option<u8>,
    pub quality: Option<f64>,
    pub slot: String,
    pub stored_item: String,
    #[serde(alias = "StoredItem_Localised")]
    pub stored_item_display: String,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSwapEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    #[serde(flatten)]
    pub ship: Ship,
    pub from_slot: String,
    pub from_item: String,
    #[serde(alias = "FromItem_Localised")]
    pub from_item_display: String,
    pub to_slot: String,
    pub to_item: Option<String>,
    #[serde(alias = "ToItem_Localised")]
    pub to_item_display: Option<String>,
    #[serde(default)]
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub buy_item: String,
    #[serde(alias = "BuyItem_Localised")]
    pub buy_item_display: String,
    pub buy_price: u64,
    pub sell_item: Option<String>,
    #[serde(alias = "SellItem_Localised")]
    pub sell_item_display: Option<String>,
    pub sell_price: Option<u64>,
    #[serde(flatten)]
    pub ship: Ship,
    pub slot: String,
    pub stored_item: Option<String>,
    #[serde(alias = "StoredItem_Localised")]
    pub stored_item_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyAndStoreEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub buy_item: String,
    #[serde(alias = "BuyItem_Localised")]
    pub buy_item_display: String,
    pub buy_price: u64,
    #[serde(flatten)]
    pub ship: Ship,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteModule {
    #[serde(flatten)]
    pub module: Module,
    pub storage_slot: u64,
    pub buy_price: u64,
    pub transfer_cost: Option<u64>,
    pub transfer_time: Option<f64>,
    pub star_system: Option<String>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(default)]
    pub in_transit: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub station_name: String,
    pub items: Vec<RemoteModule>,
}
