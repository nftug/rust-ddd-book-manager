use domain::user::values::UserId;
use serde::Deserialize;

use crate::user::dto::UserRoleDTO;

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOrCreateUserRequestDTO {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: UserRoleDTO,
}
