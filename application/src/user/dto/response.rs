use serde::Serialize;
use uuid::Uuid;

use crate::user::dto::UserRoleDTO;

#[derive(Serialize, Debug, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailsDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRoleDTO,
}
