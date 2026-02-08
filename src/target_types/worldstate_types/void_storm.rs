use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{node::Node, worldstate_types::fissure::FissureTier};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VoidStorm {
    pub id: String,

    pub node: Option<Node>,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub tier: FissureTier,
}
