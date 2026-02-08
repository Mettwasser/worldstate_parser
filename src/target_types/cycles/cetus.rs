use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, calculate_cycle};

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

    const KNOWN_NIGHT_START: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 2, 4)
        .unwrap()
        .and_hms_opt(16, 9, 0)
        .unwrap()
        .and_utc();

    pub fn now() -> Self {
        Self::at(Utc::now().with_nanosecond(0).unwrap())
    }

    pub fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::KNOWN_NIGHT_START,
            Phase(Self::NIGHT_DURATION, CetusState::Night),
            Phase(Self::DAY_DURATION, CetusState::Day),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CetusCycle;

    #[test]
    fn test() {
        dbg!(CetusCycle::now().time_left());
        dbg!(CetusCycle::now());
    }
}
