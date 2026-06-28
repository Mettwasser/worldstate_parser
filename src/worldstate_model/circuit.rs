use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{Resolve, resolvable_string::ResolvableString, resolve_with},
    target_types::worldstate_types::circuit::Circuit,
    worldstate_model::deserialize_mongo_date,
};

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub enum CategoryUnmapped {
//     #[serde(rename = "EXC_NORMAL")]
//     Normal,
//     #[serde(rename = "EXC_HARD")]
//     Hard,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CircuitUnmapped {
    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    category_choices: [CircuitCategoryChoiceUnmapped; 2],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CircuitCategoryChoiceUnmapped {
    // category: CategoryUnmapped,
    choices: Vec<ResolvableString<resolve_with::TitleCase>>,
}

impl Resolve<()> for CircuitUnmapped {
    type Output = Circuit;

    fn resolve(self, _ctx: ()) -> Self::Output {
        let [normal, sp] = self.category_choices;

        Circuit {
            activation: self.activation,
            expiry: self.expiry,
            // Unwrapping here is safe because you "know" they are both there
            normal_choices: normal.choices.resolve(()),
            steel_path_choices: sp.choices.resolve(()),
        }
    }
}
