use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, calculate_cycle};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum CambionDriftState {
    Fass,
    Vome,
}

pub type CambionDriftCycle = Cycle<CambionDriftState>;

impl CambionDriftCycle {
    pub const FASS_DURATION: Duration = Duration::minutes(100);
    pub const VOME_DURATION: Duration = Duration::minutes(50);

    const KNOWN_FASS_START: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 2, 4)
        .unwrap()
        .and_hms_opt(16, 59, 0)
        .unwrap()
        .and_utc();

    pub fn now() -> Self {
        Self::at(Utc::now().with_nanosecond(0).unwrap())
    }

    pub fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::KNOWN_FASS_START,
            Phase(Self::FASS_DURATION, CambionDriftState::Fass),
            Phase(Self::VOME_DURATION, CambionDriftState::Vome),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CambionDriftCycle;

    #[test]
    fn test() {
        dbg!(CambionDriftCycle::now().time_left());
        dbg!(CambionDriftCycle::now());
    }
}
