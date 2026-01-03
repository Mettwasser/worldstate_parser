use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    custom_maps::solnode_to_region::Region,
    manifest_entries::{faction::Faction, region::MissionType},
    worldstate_model::{Id, deserialize_mongo_date},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct Alert {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub mission_info: MissionInfo,

    pub tag: String,

    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MissionInfo {
    pub mission_type: MissionType,

    pub faction: Faction,

    pub location: Region,

    pub level_override: String,

    pub enemy_spec: String,

    pub extra_enemy_spec: Option<String>,

    pub min_enemy_level: i64,

    pub max_enemy_level: i64,

    pub difficulty: i64,

    pub seed: i64,

    pub mission_reward: MissionReward,

    pub desc_text: String,

    pub quest_req: Option<String>,

    pub leaders_always_allowed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MissionReward {
    pub credits: Option<i64>,

    pub counted_items: Vec<CountedItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct CountedItem {
    pub item_type: String,

    pub item_count: i64,
}

pub mod unmapped {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::{
        core::{InternalPath, Mappable, UseLanguages},
        custom_maps::CustomMaps,
        manifests::Exports,
        worldstate_data::WorldstateData,
        worldstate_model::{
            Id,
            WorldstateFaction,
            WorldstateMissionType,
            alert::{Alert, CountedItem, MissionInfo, MissionReward},
            deserialize_mongo_date,
        },
    };

    impl Mappable for AlertUnmapped {
        type MapTo = Option<Alert>;

        fn map(
            self,
            export: &Exports,
            custom_maps: &CustomMaps,
            worldstate_data: &WorldstateData,
        ) -> Self::MapTo {
            Some(Alert {
                id: self.id,
                activation: self.activation,
                expiry: self.expiry,
                mission_info: self
                    .mission_info
                    .map(export, custom_maps, worldstate_data)?,
                tag: self.tag,
                icon: self.icon,
            })
        }
    }

    impl Mappable for MissionInfoUnmapped {
        type MapTo = Option<MissionInfo>;

        fn map(
            self,
            export: &Exports,
            custom_maps: &CustomMaps,
            worldstate_data: &WorldstateData,
        ) -> Self::MapTo {
            Some(MissionInfo {
                mission_type: self.mission_type.try_into().ok()?,
                faction: self.faction.into(),
                location: custom_maps
                    .solnode_to_region
                    .get(self.location.as_str())
                    .cloned()?,
                level_override: self.level_override.to_title_case()?,
                enemy_spec: self.level_override.to_title_case()?,
                extra_enemy_spec: self.extra_enemy_spec.and_then(|spec| spec.to_title_case()),
                min_enemy_level: self.min_enemy_level,
                max_enemy_level: self.max_enemy_level,
                difficulty: self.difficulty,
                seed: self.seed,
                mission_reward: self
                    .mission_reward
                    .map(export, custom_maps, worldstate_data),
                desc_text: self.desc_text.map(export, custom_maps, worldstate_data),
                quest_req: self
                    .quest_req
                    .map(|quest_req| quest_req.map(export, custom_maps, worldstate_data)),
                leaders_always_allowed: self.leaders_always_allowed,
            })
        }
    }

    impl Mappable for MissionRewardUnmapped {
        type MapTo = MissionReward;

        fn map(
            self,
            export: &Exports,
            custom_maps: &CustomMaps,
            worldstate_data: &WorldstateData,
        ) -> Self::MapTo {
            MissionReward {
                credits: self.credits,
                counted_items: self
                    .counted_items
                    .into_iter()
                    .map(|item| item.map(export, custom_maps, worldstate_data))
                    .collect(),
            }
        }
    }

    impl Mappable for CountedItemUnmapped {
        type MapTo = CountedItem;

        fn map(
            self,
            export: &Exports,
            custom_maps: &CustomMaps,
            worldstate_data: &WorldstateData,
        ) -> Self::MapTo {
            CountedItem {
                item_count: self.item_count,
                item_type: self.item_type.map(export, custom_maps, worldstate_data),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct AlertUnmapped {
        #[serde(rename = "_id")]
        id: Id,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub activation: DateTime<Utc>,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub expiry: DateTime<Utc>,

        mission_info: MissionInfoUnmapped,

        tag: String,

        icon: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MissionInfoUnmapped {
        mission_type: WorldstateMissionType,

        faction: WorldstateFaction,

        /// Sol Node
        location: String,

        level_override: InternalPath,

        enemy_spec: InternalPath,

        extra_enemy_spec: Option<InternalPath>,

        min_enemy_level: i64,

        max_enemy_level: i64,

        difficulty: i64,

        seed: i64,

        mission_reward: MissionRewardUnmapped,

        desc_text: InternalPath<UseLanguages>,

        quest_req: Option<InternalPath<UseLanguages>>,

        leaders_always_allowed: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MissionRewardUnmapped {
        credits: Option<i64>,

        #[serde(default)]
        counted_items: Vec<CountedItemUnmapped>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CountedItemUnmapped {
        item_type: InternalPath<UseLanguages>,

        item_count: i64,
    }
}
