#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MissionType {
    #[serde(rename = "MT_ARENA")]
    Rathuum,

    #[serde(rename = "MT_ARMAGEDDON")]
    VoidArmageddon,

    #[serde(rename = "MT_ARTIFACT")]
    Disruption,

    #[serde(rename = "MT_ASSAULT")]
    Assault,

    #[serde(rename = "MT_ASSASSINATION")]
    Assassination,

    #[serde(rename = "MT_CAPTURE")]
    Capture,

    #[serde(rename = "MT_CORRUPTION")]
    VoidFlood,

    #[serde(rename = "MT_DEFAULT")]
    Unknown,

    #[serde(rename = "MT_DEFENSE")]
    Defense,

    #[serde(rename = "MT_ENDLESS_CAPTURE")]
    LegacyteHarvest,

    #[serde(rename = "MT_ENDLESS_EXTERMINATION")]
    SanctuaryOnslaught,

    #[serde(rename = "MT_EVACUATION")]
    Defection,

    #[serde(rename = "MT_EXCAVATE")]
    Excavation,

    #[serde(rename = "MT_EXTERMINATION")]
    Exterminate,

    #[serde(rename = "MT_HIVE")]
    HiveSabotage,

    #[serde(rename = "MT_INTEL")]
    Spy,

    #[serde(rename = "MT_LANDSCAPE")]
    Landscape,

    #[serde(rename = "MT_MOBILE_DEFENSE")]
    MobileDefense,

    #[serde(rename = "MT_PURIFY")]
    InfestedSalvage,

    #[serde(rename = "MT_PVP")]
    Conclave,

    #[serde(rename = "MT_RACE")]
    Rush,

    #[serde(rename = "MT_RESCUE")]
    Rescue,

    #[serde(rename = "MT_RETRIEVAL")]
    Hijack,

    #[serde(rename = "MT_SABOTAGE")]
    Sabotage,

    #[serde(rename = "MT_SURVIVAL")]
    Survival,

    #[serde(rename = "MT_TERRITORY")]
    Interception,

    #[serde(rename = "MT_VOID_CASCADE")]
    VoidCascade,
}
