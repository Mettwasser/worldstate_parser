use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::node::Node;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub grace_period: Option<DateTime<Utc>>,

    pub count: Option<u64>,

    pub goal: Option<u64>,

    pub success: Option<u64>,

    pub personal: bool,

    pub desc: String,

    pub tool_tip: Option<String>,

    pub icon: Option<String>,

    pub tag: String,

    pub node: Option<Node>,
}
