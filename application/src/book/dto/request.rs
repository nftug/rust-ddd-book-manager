use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequestDTO {
    pub title: String,
    pub author_names: Vec<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequestDTO {
    pub title: String,
    pub author_names: Vec<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBookOwnerRequestDTO {
    pub new_owner_id: Uuid,
}
