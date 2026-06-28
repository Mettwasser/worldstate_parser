use chrono::{DateTime, Duration, NaiveDate, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::{Cycle, Phase, WorldCycle, calculate_cycle};

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
}

impl WorldCycle for CambionDriftCycle {
    type State = CambionDriftState;

    const ANCHOR: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 6, 28)
        .unwrap()
        .and_hms_opt(15, 33, 0)
        .unwrap()
        .and_utc();

    const ANCHOR_STATE: Self::State = CambionDriftState::Fass;

    fn at(time: DateTime<Utc>) -> Self {
        calculate_cycle(
            time,
            Self::ANCHOR,
            Phase(Self::FASS_DURATION, Self::ANCHOR_STATE),
            Phase(Self::VOME_DURATION, CambionDriftState::Vome),
        )
    }
}
