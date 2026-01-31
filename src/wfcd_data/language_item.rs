use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{core::Resolve, target_types::display_info::DisplayInfo};

pub type LanguageItemMap = HashMap<String, LanguageItem>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct LanguageItem {
    pub value: String,
    pub desc: Option<String>,
}

impl Resolve<()> for LanguageItem {
    type Output = DisplayInfo;

    fn resolve(self, _ctx: ()) -> Self::Output {
        DisplayInfo {
            title: self.value,
            description: self.desc,
        }
    }
}
