use chrono::{DateTime, Duration, NaiveDate, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, WorldCycle, calculate_cycle};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum OrbVallisState {
    Warm,
    Cold,
}

pub type OrbVallisCycle = Cycle<OrbVallisState>;

impl OrbVallisCycle {
    pub const COLD_DURATION: Duration = Duration::minutes(20);
    pub const WARM_DURATION: Duration = Duration::seconds(6 * 60 + 40);
}

impl WorldCycle for OrbVallisCycle {
    type State = OrbVallisState;

    const ANCHOR: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 6, 28)
        .unwrap()
        .and_hms_opt(17, 6, 48)
        .unwrap()
        .and_utc();

    const ANCHOR_STATE: Self::State = OrbVallisState::Warm;

    fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::ANCHOR,
            Phase(Self::WARM_DURATION, Self::ANCHOR_STATE),
            Phase(Self::COLD_DURATION, OrbVallisState::Cold),
        )
    }
}

