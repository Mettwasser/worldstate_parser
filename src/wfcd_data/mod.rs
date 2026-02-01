pub mod bounty_rewards;
pub mod language_item;
pub mod sortie_data;

use std::collections::HashMap;

use crate::wfcd_data::{
    bounty_rewards::{BountyRewards, DropItem},
    language_item::LanguageItemMap,
    sortie_data::SortieData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WorldstateData {
    pub language_items: LanguageItemMap,
    pub sortie_data: SortieData,
    pub rewards: BountyRewards,
    pub archon_hunt_rewards: Vec<DropItem>,
    pub hubs: HashMap<String, String>,
    pub archon_shards_store_item: HashMap<String, String>,
}
