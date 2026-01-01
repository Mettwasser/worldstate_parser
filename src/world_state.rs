use crate::{
    fissure::{Fissure, FissureUnmapped},
    manifests::Exports,
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldStateUnmapped {
    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map(self, exports: Exports) -> Option<WorldState> {
        let fissures = self
            .fissures
            .into_iter()
            .map(|unmapped_fissure| unmapped_fissure.map(&exports.export_regions))
            .collect::<Option<Vec<_>>>()?;

        Some(WorldState { fissures })
    }
}

#[derive(Debug)]
pub struct WorldState {
    pub fissures: Vec<Fissure>,
}
