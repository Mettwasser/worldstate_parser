use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Circuit {
    pub normal_choices: Vec<String>,

    pub steel_path_choices: Vec<String>,
}
