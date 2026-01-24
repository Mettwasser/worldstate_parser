use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{faction::Faction, node::Node, worldstate::counted_item::CountedItem};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Invasion {
    pub id: String,

    pub faction: Faction,

    pub defender_faction: Faction,

    pub node: Option<Node>,

    pub count: i64,

    pub goal: u64,

    pub loc_tag: String,

    pub completed: bool,

    pub chain_id: String,

    pub attacker_reward: Vec<CountedItem>,

    pub attacker_mission_info: InvasionMissionInfo,

    pub defender_reward: Vec<CountedItem>,

    pub defender_mission_info: InvasionMissionInfo,

    pub activation: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct InvasionMissionInfo {
    pub seed: i64,

    pub faction: Faction,
}
