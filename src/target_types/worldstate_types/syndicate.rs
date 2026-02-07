use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn serialize_nightwave<S>(season: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Nightwave Season {season}"))
}

fn deserialize_nightwave<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if let Some(season_str) = s.strip_prefix("Nightwave Season ") {
        season_str.parse::<u8>().map_err(serde::de::Error::custom)
    } else {
        Err(serde::de::Error::custom(
            "expected 'Nightwave Season {season}'",
        ))
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SyndicateType {
    #[serde(rename(serialize = "Arbiters"))]
    Arbiters,

    #[serde(rename(serialize = "Necraloid"))]
    Necraloid,

    #[serde(rename(serialize = "Event"))]
    Event,

    #[serde(rename(serialize = "Cephalon Suda"))]
    CephalonSuda,

    #[serde(rename(serialize = "Kahl"))]
    Kahl,

    #[serde(rename(serialize = "NewLoka"))]
    NewLoka,

    #[serde(rename(serialize = "Nightcap Journal"))]
    NightcapJournal,

    #[serde(rename(serialize = "Quills"))]
    Quills,

    #[serde(rename(serialize = "Radio Legion"))]
    RadioLegion,

    #[serde(rename(serialize = "Radio Legion 2"))]
    RadioLegion2,

    #[serde(rename(serialize = "Radio Legion 3"))]
    RadioLegion3,

    #[serde(rename(serialize = "Perrin"))]
    Perrin,

    #[serde(rename(serialize = "Vox"))]
    Vox,

    #[serde(rename(serialize = "Red Veil"))]
    RedVeil,

    #[serde(rename(serialize = "Vent Kids"))]
    VentKids,

    #[serde(rename(serialize = "Steel Meridian"))]
    SteelMeridian,

    #[serde(rename(serialize = "Cavia"))]
    Cavia,

    #[serde(rename(serialize = "Hex"))]
    Hex,

    #[serde(rename(serialize = "Entrati"))]
    Entrati,

    #[serde(rename(serialize = "Ostrons"))]
    Ostrons,

    #[serde(rename(serialize = "Solaris United"))]
    SolarisUnited,

    #[serde(rename(serialize = "Zariman"))]
    Zariman,

    #[serde(
        serialize_with = "serialize_nightwave",
        deserialize_with = "deserialize_nightwave"
    )]
    Nightwave(u8),
}
