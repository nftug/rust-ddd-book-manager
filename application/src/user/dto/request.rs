use serde::Deserialize;
use uuid::Uuid;

use crate::user::dto::UserRoleDTO;

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOrCreateUserRequestDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRoleDTO,
}
