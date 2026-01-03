use serde::{Deserialize, Serialize};

use crate::{
    core::Mappable,
    custom_maps::CustomMaps,
    manifests::Exports,
    worldstate_data::WorldstateData,
    worldstate_model::{
        alert::unmapped::AlertUnmapped,
        fissure::{Fissure, FissureUnmapped},
    },
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldStateUnmapped {
    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,

    pub alerts: Vec<AlertUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(
        self,
        exports: &Exports,
        custom_maps: &CustomMaps,
        worldstate_data: &WorldstateData,
    ) -> Option<WorldState> {
        let fissures = self
            .fissures
            .into_iter()
            .map(|unmapped_fissure| unmapped_fissure.map(exports, custom_maps, worldstate_data))
            .collect::<Option<Vec<_>>>()?;

        Some(WorldState { fissures })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldState {
    pub fissures: Vec<Fissure>,
}
