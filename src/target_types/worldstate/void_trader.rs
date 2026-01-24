use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VoidTrader {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub character: String,

    pub node: String,

    pub shop: Vec<ShopItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ShopItem {
    pub item_type: String,

    pub prime_price: u64,

    pub regular_price: u64,

    /// Limited per-user. Such as Baro's loot box
    pub limit: Option<u64>,
}
