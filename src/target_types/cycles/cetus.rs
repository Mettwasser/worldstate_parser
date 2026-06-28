use chrono::{DateTime, Duration, NaiveDate, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, WorldCycle, calculate_cycle};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum CetusState {
    Day,
    Night,
}

pub type CetusCycle = Cycle<CetusState>;

impl CetusCycle {
    pub const DAY_DURATION: Duration = Duration::minutes(100);
    pub const NIGHT_DURATION: Duration = Duration::minutes(50);
}

impl WorldCycle for CetusCycle {
    type State = CetusState;

    const ANCHOR: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 6, 28)
        .unwrap()
        .and_hms_opt(15, 33, 0)
        .unwrap()
        .and_utc();

    const ANCHOR_STATE: Self::State = CetusState::Day;

    fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::ANCHOR,
            Phase(Self::DAY_DURATION, Self::ANCHOR_STATE),
            Phase(Self::NIGHT_DURATION, CetusState::Night),
        )
    }
}
