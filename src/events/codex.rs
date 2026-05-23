use {crate::elite::codex::CodexEntryCategory, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    #[serde(alias = "BodyID")]
    body_id: u64,
    category: CodexEntryCategory,
    #[serde(default)]
    is_new_entry: bool,
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: String,
    nearest_destination: Option<String>,
    region: String,
    #[serde(alias = "Region_Localised")]
    region_display: String,
    sub_category: String,
    #[serde(alias = "SubCategory_Localised")]
    sub_category_display: String,
    system_address: u64,
    voucher_amount: Option<u64>,
}
