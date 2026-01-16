use serde::Deserialize;
use uuid::Uuid;

use crate::user::dto::UserRoleDTO;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct GetOrCreateUserRequestDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRoleDTO,
}
