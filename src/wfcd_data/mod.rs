pub mod bounty_rewards;
pub mod language_item;
pub mod sortie_data;

use std::{collections::HashMap, fs, io, path::Path, sync::LazyLock};

use regex::Regex;
use serde::de::DeserializeOwned;

use crate::{
    PathContext,
    wfcd_data::{
        bounty_rewards::{BountyRewards, DropItem},
        language_item::LanguageItemMap,
        sortie_data::SortieData,
    },
};

static SOLNODES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*) \(.*\)").unwrap());

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorldstateDataError {
    Io(#[from] io::Error),
    Deserialize(#[from] serde_json::Error),
}

fn init<T: DeserializeOwned>(
    data_dir: impl AsRef<Path>,
    file: impl AsRef<Path>,
) -> Result<T, WorldstateDataError> {
    let path = data_dir.as_ref().join(file.as_ref().with_extension("json"));

    Ok(serde_json::from_str(fs::read_to_string(path)?.as_str())?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldstateData {
    pub language_items: LanguageItemMap,
    pub sortie_data: SortieData,
    pub rewards: BountyRewards,
    pub archon_hunt_rewards: Vec<DropItem>,
    pub hubs: HashMap<String, String>,
    pub archon_shards_store_item: HashMap<String, String>,
}

impl WorldstateData {
    pub fn new(
        PathContext {
            data_dir,
            assets_dir,
            drops_dir,
        }: PathContext<'_>,
    ) -> Result<Self, WorldstateDataError> {
        let mut language_items: LanguageItemMap = init(data_dir, "languages")?;
        let archimedea_ext: LanguageItemMap = init(assets_dir, "archimedeaExt")?;
        language_items.extend(archimedea_ext);

        #[derive(serde::Deserialize)]
        pub struct SolNodeItem {
            value: String,
        }

        let sol_nodes: HashMap<String, SolNodeItem> = init(data_dir, "solNodes")?;

        let hubs = sol_nodes
            .into_iter()
            .filter_map(|(key, value)| {
                if !key.contains("HUB") {
                    return None;
                }

                let relay_name = SOLNODES_REGEX
                    .captures(&value.value)
                    .and_then(|cap| cap.get(1))
                    .map(|r#match| r#match.as_str().to_owned())
                    .unwrap_or_else(|| value.value);

                Some((key, relay_name))
            })
            .collect();

        Ok(Self {
            language_items,
            sortie_data: init(data_dir, "sortieData")?,
            rewards: init(drops_dir, "data")?,
            hubs,
            archon_hunt_rewards: init(assets_dir, "archonHuntRewards")?,
            archon_shards_store_item: init(assets_dir, "archonShardsStoreItem")?,
        })
    }
}
