use serde::{Deserialize, Serialize};

use crate::core::{InternalPath, resolve_with};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CountedItemUnmapped {
    pub item_type: InternalPath<resolve_with::LanguageItems>,

    pub item_count: i64,
}
