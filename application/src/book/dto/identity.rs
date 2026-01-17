use domain::book::values::BookId;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub struct BookIdentity {
    pub book_id: BookId,
}
