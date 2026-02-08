use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, calculate_cycle};

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

    const KNOWN_WARM_START: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 2, 4)
        .unwrap()
        .and_hms_opt(19, 46, 48)
        .unwrap()
        .and_utc();

    pub fn now() -> Self {
        Self::at(Utc::now().with_nanosecond(0).unwrap())
    }

    pub fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::KNOWN_WARM_START,
            Phase(Self::WARM_DURATION, OrbVallisState::Warm),
            Phase(Self::COLD_DURATION, OrbVallisState::Cold),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::OrbVallisCycle;

    #[test]
    fn test() {
        dbg!(OrbVallisCycle::now().time_left());
        dbg!(OrbVallisCycle::now());
    }
}
