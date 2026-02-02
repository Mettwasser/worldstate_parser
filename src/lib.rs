pub mod core;
pub mod custom_maps;
#[cfg(feature = "default_provider")]
pub mod default_context_provider;
#[cfg(feature = "default_fetcher")]
pub mod default_data_fetcher;
pub mod manifest_entries;
pub mod manifests;
pub mod target_types;
pub(crate) mod wfcd_data;
pub mod worldstate;
pub(crate) mod worldstate_model;

use crate::core::Context;

pub trait ContextProvider {
    type Err;
    fn get_ctx(&self) -> impl Future<Output = Result<Context, Self::Err>> + Send;
}
