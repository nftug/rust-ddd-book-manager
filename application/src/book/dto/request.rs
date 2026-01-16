use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct CreateBookRequestDTO {
    pub title: String,
    pub author_names: Vec<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct UpdateBookRequestDTO {
    pub title: String,
    pub author_names: Vec<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct ChangeBookOwnerRequestDTO {
    pub new_owner_id: Uuid,
}
