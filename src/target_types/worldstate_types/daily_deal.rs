use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DailyDeal {
    pub item: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub discount_percentage: u32,

    pub original_price: u32,

    pub sale_price: u32,

    pub stock: u32,

    pub amount_sold: u32,
}
