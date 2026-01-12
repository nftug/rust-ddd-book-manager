use domain::user::enums::UserRole;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailsDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}
