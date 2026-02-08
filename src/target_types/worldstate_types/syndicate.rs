use std::str::FromStr;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub enum SyndicateType {
    #[display("Arbiters")]
    Arbiters,

    #[display("Necraloid")]
    Necraloid,

    #[display("Event")]
    Event,

    #[display("Cephalon Suda")]
    CephalonSuda,

    #[display("Kahl")]
    Kahl,

    #[display("NewLoka")]
    NewLoka,

    #[display("Nightcap Journal")]
    NightcapJournal,

    #[display("Quills")]
    Quills,

    #[display("Radio Legion")]
    RadioLegion,

    #[display("Radio Legion 2")]
    RadioLegion2,

    #[display("Radio Legion 3")]
    RadioLegion3,

    #[display("Perrin")]
    Perrin,

    #[display("Vox")]
    Vox,

    #[display("Red Veil")]
    RedVeil,

    #[display("Vent Kids")]
    VentKids,

    #[display("Steel Meridian")]
    SteelMeridian,

    #[display("Cavia")]
    Cavia,

    #[display("Hex")]
    Hex,

    #[display("Entrati")]
    Entrati,

    #[display("Ostrons")]
    Ostrons,

    #[display("Solaris United")]
    SolarisUnited,

    #[display("Zariman")]
    Zariman,

    #[display("Nightwave Season {_0}")]
    Nightwave(u8),
}

impl From<SyndicateType> for String {
    fn from(syndicate: SyndicateType) -> Self {
        syndicate.to_string()
    }
}

impl TryFrom<String> for SyndicateType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for SyndicateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(season_str) = s.strip_prefix("Nightwave Season ") {
            return season_str
                .parse::<u8>()
                .map(SyndicateType::Nightwave)
                .map_err(|_| "Invalid Nightwave season number".to_string());
        }

        match s {
            "Arbiters" => Ok(SyndicateType::Arbiters),
            "Necraloid" => Ok(SyndicateType::Necraloid),
            "Event" => Ok(SyndicateType::Event),
            "Cephalon Suda" => Ok(SyndicateType::CephalonSuda),
            "Kahl" => Ok(SyndicateType::Kahl),
            "NewLoka" => Ok(SyndicateType::NewLoka),
            "Nightcap Journal" => Ok(SyndicateType::NightcapJournal),
            "Quills" => Ok(SyndicateType::Quills),
            "Radio Legion" => Ok(SyndicateType::RadioLegion),
            "Radio Legion 2" => Ok(SyndicateType::RadioLegion2),
            "Radio Legion 3" => Ok(SyndicateType::RadioLegion3),
            "Perrin" => Ok(SyndicateType::Perrin),
            "Vox" => Ok(SyndicateType::Vox),
            "Red Veil" => Ok(SyndicateType::RedVeil),
            "Vent Kids" => Ok(SyndicateType::VentKids),
            "Steel Meridian" => Ok(SyndicateType::SteelMeridian),
            "Cavia" => Ok(SyndicateType::Cavia),
            "Hex" => Ok(SyndicateType::Hex),
            "Entrati" => Ok(SyndicateType::Entrati),
            "Ostrons" => Ok(SyndicateType::Ostrons),
            "Solaris United" => Ok(SyndicateType::SolarisUnited),
            "Zariman" => Ok(SyndicateType::Zariman),
            _ => Err(format!("Unknown syndicate type: {}", s)),
        }
    }
}
