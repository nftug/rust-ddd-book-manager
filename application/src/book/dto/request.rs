use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequestDTO {
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequestDTO {
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBookOwnerRequestDTO {
    pub new_owner_id: Uuid,
}
