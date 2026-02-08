pub mod cambion_drift;
pub mod cetus;
pub mod duviri;
pub mod orb_vallis;

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Cycle<S> {
    pub activation: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    pub state: S,
}

impl<S> Cycle<S> {
    /// Returns the time remaining as a formatted string (e.g., "1h 30mins 1sec")
    pub fn time_left(&self) -> String {
        let now = Utc::now().with_nanosecond(0).unwrap();
        let duration = self.expiry - now;

        if duration.num_seconds() <= 0 {
            return "0s".to_string();
        }

        Self::format_duration(duration)
    }

    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.num_seconds();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        let mut parts = Vec::with_capacity(3);

        if hours > 0 {
            parts.push(format!("{hours}h"));
        }

        if minutes > 0 {
            parts.push(format!("{minutes}m"));
        }

        if seconds > 0 {
            parts.push(format!("{seconds}s"));
        }

        parts.join(" ")
    }
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Phase<S>(pub Duration, pub S);

fn calculate_cycle<S: Copy>(
    now: DateTime<Utc>,
    anchor: DateTime<Utc>,
    first_phase: Phase<S>,
    second_phase: Phase<S>,
) -> Cycle<S> {
    let now = now.with_nanosecond(0).unwrap();

    let Phase(first_duration, first_state) = first_phase;
    let Phase(second_duration, second_state) = second_phase;

    let total_cycle = first_duration + second_duration;

    let offset = (now.timestamp() - anchor.timestamp()).rem_euclid(total_cycle.num_seconds());

    let cycle_start = now - Duration::seconds(offset);

    // Logic: Are we in Phase 1 or Phase 2?
    if offset < first_duration.num_seconds() {
        Cycle {
            state: first_state,
            activation: cycle_start,
            expiry: cycle_start + first_duration,
        }
    } else {
        Cycle {
            state: second_state,
            activation: cycle_start + first_duration,
            expiry: cycle_start + first_duration + second_duration,
        }
    }
}
