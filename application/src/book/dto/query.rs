use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BookListQueryDTO {
    #[garde(range(min = 1))]
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    #[garde(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u64,
    #[garde(skip)]
    pub owner_id: Option<Uuid>,
    #[garde(skip)]
    pub checked_out: Option<bool>,
    #[garde(skip)]
    pub checked_out_to_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CheckoutHistoryQueryDTO {
    #[garde(range(min = 1))]
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    #[garde(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u64,
}

const fn default_page_size() -> u64 {
    10
}

const fn default_page() -> u64 {
    1
}
