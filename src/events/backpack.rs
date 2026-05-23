use {crate::elite::backpack::BackpackItem, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    pub components: Option<Vec<BackpackItem>>,
    pub consumables: Option<Vec<BackpackItem>>,
    pub data: Option<Vec<BackpackItem>>,
    pub items: Option<Vec<BackpackItem>>,
    // I guess, TODO: need correction
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEvent {
    pub added: Option<Vec<BackpackItem>>,
    pub removed: Option<Vec<BackpackItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CollectItemsEvent {
    #[serde(flatten)]
    pub item: BackpackItem,
    #[serde(default)]
    pub stolen: bool,
}
