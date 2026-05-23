use {crate::elite::material::Material, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEvent {
    materials: Vec<Material>,
    name: String,
}
