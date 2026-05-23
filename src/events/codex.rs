use {crate::elite::codex::CodexEntryCategory, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub category: CodexEntryCategory,
    #[serde(default)]
    pub is_new_entry: bool,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
    pub nearest_destination: Option<String>,
    pub region: String,
    #[serde(alias = "Region_Localised")]
    pub region_display: String,
    pub sub_category: String,
    #[serde(alias = "SubCategory_Localised")]
    pub sub_category_display: String,
    pub system_address: u64,
    pub voucher_amount: Option<u64>,
}
