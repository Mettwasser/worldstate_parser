use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::display_info::DisplayInfo;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub days: Vec<CalendarDay>,

    pub season: CalendarSeason,

    pub year_iteration: u32,

    pub version: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct CalendarDay {
    pub day: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub event: Option<CalendarEvent>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum CalendarEvent {
    Challenge(DisplayInfo),

    Rewards([String; 2]),

    Upgrades([DisplayInfo; 3]),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CalendarSeason {
    Summer,

    Winter,

    Spring,

    Fall,
}
