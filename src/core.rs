use std::marker::PhantomData;

use heck::ToTitleCase;
use serde::{Deserialize, Serialize};

use crate::{custom_maps::CustomMaps, manifests::Exports, worldstate_data::WorldstateData};

pub trait Mappable {
    type MapTo;

    fn map(
        self,
        export: &Exports,
        custom_maps: &CustomMaps,
        worldstate_data: &WorldstateData,
    ) -> Self::MapTo;
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, derive_more::FromStr)]
pub enum InternalPathTag {
    Levels,
    Types,
    Language,
    Upgrades,
    Interface,

    Unknown,
}

#[derive(Debug)]
pub struct UseLanguages;

/// Deserializes an internal path like `/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense`.
///
/// Yields additional info about the tag via the [`InternalPath::tag`] field.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, derive_more::Display)]
#[serde(from = "String")]
#[display("{path}")]
pub struct InternalPath<T = ()> {
    pub path: String,
    pub tag: InternalPathTag,

    #[serde(skip)]
    _p: PhantomData<T>,
}

impl<T> From<String> for InternalPath<T> {
    fn from(path: String) -> Self {
        let tag = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(InternalPathTag::Unknown);

        Self {
            path,
            tag,
            _p: PhantomData,
        }
    }
}

impl<T> InternalPath<T> {
    pub fn to_title_case(&self) -> Option<String> {
        self.path.split('/').next_back().map(|s| s.to_title_case())
    }

    pub fn into_title_case_or_path(self) -> String {
        self.to_title_case().unwrap_or(self.path)
    }
}

impl Mappable for InternalPath<UseLanguages> {
    type MapTo = String;

    fn map(
        self,
        _export: &Exports,
        _custom_maps: &CustomMaps,
        worldstate_data: &WorldstateData,
    ) -> Self::MapTo {
        worldstate_data
            .language_items
            .get(&self.path)
            .map(|item| &item.value)
            .cloned()
            .unwrap_or_else(|| self.to_title_case().unwrap_or(self.path))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BoxDynError,
        core::{InternalPath, InternalPathTag},
    };

    #[test]
    fn test_from_internal_path() -> Result<(), BoxDynError> {
        let internal_path: InternalPath =
            serde_json::from_str("\"/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense\"")?;

        assert_eq!(internal_path.tag, InternalPathTag::Levels);

        assert_eq!(
            internal_path.to_title_case().unwrap(),
            "Orokin Tower Mobile Defense"
        );

        Ok(())
    }
}
