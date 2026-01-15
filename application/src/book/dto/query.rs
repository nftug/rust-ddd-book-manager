use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct BookListQueryDTO {
    #[garde(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[garde(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u64,
    #[garde(skip)]
    pub owner_id: Option<Uuid>,
    #[garde(skip)]
    pub checked_out: Option<bool>,
}

const fn default_limit() -> u64 {
    10
}

const fn default_page() -> u64 {
    1
}
