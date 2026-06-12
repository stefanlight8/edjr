use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialCategory {
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Encoded;")]
    Encoded,
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Manufactured;")]
    Manufactured,
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Elements;")]
    Raw,
    // I'm not sure that is a good thing to edit MaterialCategory
    // only because of MaterialsReward field in MissionCompleted,
    // but I'm quite good about to provide unified API.
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub category: Option<MaterialCategory>,
    #[serde(alias = "Material")]
    pub name: String,
    #[serde(alias = "Material_Localised", alias = "Name_Localised")]
    pub name_display: Option<String>,
    #[serde(alias = "Quantity")]
    pub count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraderType {
    Encoded,
    Manufactured,
    Raw,
}
