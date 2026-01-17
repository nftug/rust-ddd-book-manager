use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthorSummaryDTO {
    pub id: Uuid,
    pub name: String,
}
