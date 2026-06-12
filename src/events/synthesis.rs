use {
    crate::elite::material::Material,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEvent {
    pub materials: Vec<Material>,
    pub name: String,
}
