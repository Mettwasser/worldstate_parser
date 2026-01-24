use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, InternalPath, Resolve, resolve_with, sol_node::SolNode},
    target_types::worldstate::{
        counted_item::CountedItem,
        invasion::{Invasion, InvasionMissionInfo},
    },
    worldstate_model::{
        Id,
        WorldstateFaction,
        counted_item::CountedItemUnmapped,
        deserialize_mongo_date,
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvasionUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    faction: WorldstateFaction,

    defender_faction: WorldstateFaction,

    node: SolNode,

    count: i64,

    goal: u64,

    loc_tag: InternalPath<resolve_with::LanguageItems>,

    completed: bool,

    #[serde(rename = "ChainID")]
    chain_id: Id,

    attacker_reward: AttackerRewardUnmapped,

    attacker_mission_info: InvasionMissionInfoUnmapped,

    defender_reward: InvasionRewardUnmapped,

    defender_mission_info: InvasionMissionInfoUnmapped,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,
}

impl Resolve<Context<'_>> for InvasionUnmapped {
    type Output = Invasion;

    fn resolve(self, ctx: Context<'_>) -> Self::Output {
        Invasion {
            id: self.id.oid,
            faction: self.faction.resolve(()),
            defender_faction: self.defender_faction.resolve(()),
            node: self.node.resolve(ctx).cloned(),
            count: self.count,
            goal: self.goal,
            loc_tag: self.loc_tag.resolve(ctx),
            completed: self.completed,
            chain_id: self.chain_id.oid,
            attacker_reward: self.attacker_reward.resolve(ctx),
            attacker_mission_info: self.attacker_mission_info.resolve(()),
            defender_reward: self.defender_reward.counted_items.resolve(ctx),
            defender_mission_info: self.defender_mission_info.resolve(()),
            activation: self.activation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvasionMissionInfoUnmapped {
    seed: i64,

    faction: WorldstateFaction,
}

impl Resolve<()> for InvasionMissionInfoUnmapped {
    type Output = InvasionMissionInfo;

    fn resolve(self, _ctx: ()) -> Self::Output {
        InvasionMissionInfo {
            seed: self.seed,
            faction: self.faction.resolve(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttackerRewardUnmapped {
    Array(Vec<Option<serde_json::Value>>),

    Reward(InvasionRewardUnmapped),
}

impl Resolve<Context<'_>> for AttackerRewardUnmapped {
    type Output = Vec<CountedItem>;

    fn resolve(self, ctx: Context<'_>) -> Self::Output {
        match self {
            AttackerRewardUnmapped::Array(_) => vec![],
            AttackerRewardUnmapped::Reward(reward_unmapped) => {
                reward_unmapped.counted_items.resolve(ctx)
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvasionRewardUnmapped {
    counted_items: Vec<CountedItemUnmapped>,
}
