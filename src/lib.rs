pub mod core;
pub mod custom_maps;
#[cfg(feature = "default_provider")]
pub mod default_context_provider;
pub mod manifest_entries;
pub mod manifests;
pub mod target_types;
pub(crate) mod wfcd_data;
pub mod worldstate;
pub(crate) mod worldstate_model;

use std::path::Path;

use crate::core::Context;

pub trait ContextProvider<Data> {
    type Err;
    fn get_ctx(&self, data: Data) -> impl Future<Output = Result<Context, Self::Err>> + Send;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathContext<'a> {
    pub data_dir: &'a Path,
    pub drops_dir: &'a Path,
    pub assets_dir: &'a Path,
}
