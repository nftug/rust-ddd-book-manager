use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct AuthorSummaryDTO {
    pub id: Uuid,
    pub name: String,
}
