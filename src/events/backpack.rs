use {crate::elite::backpack::BackpackItem, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    components: Option<Vec<BackpackItem>>,
    consumables: Option<Vec<BackpackItem>>,
    data: Option<Vec<BackpackItem>>,
    items: Option<Vec<BackpackItem>>,
    // I guess, TODO: need correction
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEvent {
    added: Option<Vec<BackpackItem>>,
    removed: Option<Vec<BackpackItem>>,
}
