use super::{
    CreatorType, Genre, ItemArgs, ItemRestriction, ItemStatus, ItemType, PremiumPricing,
    PriceStatus,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ItemDetailsResponse {
    pub data: Vec<ItemDetailsRaw>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemDetailsRaw {
    pub id: Option<u64>,
    pub item_type: Option<ItemType>,
    pub bundle_type: Option<u64>,
    pub asset_type: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub product_id: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    pub item_status: Option<Vec<ItemStatus>>,
    pub item_restrictions: Option<Vec<ItemRestriction>>,
    pub creator_has_verified_badge: Option<bool>,
    pub creator_type: Option<CreatorType>,
    pub creator_target_id: Option<u64>,
    pub creator_name: Option<String>,
    /// Exists instead of lowest_price if the item is non-limited.
    pub price: Option<u64>,
    /// Exists instead of price if the item is limited.
    pub lowest_price: Option<u64>,
    pub favorite_count: Option<u64>,
    pub premium_pricing: Option<PremiumPricing>,
    pub price_status: Option<PriceStatus>,
    /// It is unknown as to what type this value is.
    /// The farthest it can be tracked by reverse engineering is that the value
    /// is fed into a `new Date()` constructor in js.
    ///
    /// Because of this, it is not included in the public struct until
    /// we know what it is.
    pub off_sale_deadline: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ItemDetailsReqBody {
    pub(crate) items: Vec<ItemArgsReq>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemArgsReq {
    pub item_type: ItemType,
    pub id: u64,
}

impl From<ItemArgs> for ItemArgsReq {
    fn from(item: ItemArgs) -> Self {
        Self {
            item_type: item.item_type,
            id: item.id,
        }
    }
}
