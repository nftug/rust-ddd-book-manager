use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BookListQueryDTO {
    #[garde(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[garde(range(min = 1))]
    #[serde(default)]
    pub page: usize,
    #[garde(skip)]
    pub owner_id: Option<String>,
}

const fn default_limit() -> usize {
    10
}

impl BookListQueryDTO {
    pub fn page_from_zero(&self) -> u64 {
        if self.page == 0 {
            0
        } else {
            (self.page - 1) as u64
        }
    }
}
