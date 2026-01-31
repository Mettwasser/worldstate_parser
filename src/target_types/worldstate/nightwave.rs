use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::display_info::DisplayInfo;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum ChallengeType {
    Daily,
    Weekly,
    Elite,
}

impl ChallengeType {
    pub fn standing_awarded(self) -> u32 {
        match self {
            ChallengeType::Daily => 1000,
            ChallengeType::Weekly => 4500,
            ChallengeType::Elite => 7000,
        }
    }

    pub(crate) fn from_path(path: &str) -> Option<Self> {
        Some(match path {
            p if p.contains("WeeklyHard/") => Self::Elite,
            p if p.contains("Weekly/") => Self::Weekly,
            p if p.contains("Daily/") => Self::Daily,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Nightwave {
    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub affiliation_tag: String,

    pub season: u32,

    pub phase: u32,

    pub params: String,

    pub active_challenges: Vec<ActiveChallenge>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeInfo {
    pub challenge_type: ChallengeType,

    pub standing_awarded: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ActiveChallenge {
    pub id: String,

    #[serde(flatten)]
    pub challenge_info: Option<ChallengeInfo>,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    #[serde(flatten)]
    pub challenge: DisplayInfo,
}
