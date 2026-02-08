use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VaultTrader {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub initial_start_date: DateTime<Utc>,

    pub node: String,

    pub shop: Vec<VaultTraderManifest>,

    /// Former Twitch Prime drops available for sale
    pub twitch_prime_shop: Vec<VaultTraderManifest>,

    pub schedule_info: Vec<VaultTraderScheduleInfo>,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum VaultTraderCurrency {
    Aya,
    #[serde(rename = "Regal Aya")]
    RegalAya,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VaultTraderPrice {
    pub currency: VaultTraderCurrency,
    pub amount: u64,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VaultTraderManifest {
    pub item_type: String,

    pub price: VaultTraderPrice,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VaultTraderScheduleInfo {
    pub expiry: DateTime<Utc>,

    pub preview_hidden_until: Option<DateTime<Utc>>,

    pub featured_item: String,
}
