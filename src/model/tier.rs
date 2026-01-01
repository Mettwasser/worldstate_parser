#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum Tier {
    #[serde(rename = "VoidT1")]
    Lith,
    #[serde(rename = "VoidT2")]
    Meso,
    #[serde(rename = "VoidT3")]
    Neo,
    #[serde(rename = "VoidT4")]
    Axi,
    #[serde(rename = "VoidT5")]
    Requiem,
    #[serde(rename = "VoidT6")]
    Omnia,
}
