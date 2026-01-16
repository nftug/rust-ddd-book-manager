use domain::user::enums::UserRole;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, EnumString, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub enum UserRoleDTO {
    Admin,
    #[default]
    Regular,
    System,
}

impl From<UserRoleDTO> for UserRole {
    fn from(dto: UserRoleDTO) -> Self {
        match dto {
            UserRoleDTO::Admin => UserRole::Admin,
            UserRoleDTO::Regular => UserRole::Regular,
            UserRoleDTO::System => UserRole::System,
        }
    }
}
