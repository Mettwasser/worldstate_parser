use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct DisplayInfo {
    pub title: String,
    pub description: Option<String>,
}
