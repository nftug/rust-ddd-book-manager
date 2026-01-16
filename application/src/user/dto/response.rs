use serde::Serialize;
use uuid::Uuid;

use crate::user::dto::UserRoleDTO;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct UserDetailsDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRoleDTO,
}
