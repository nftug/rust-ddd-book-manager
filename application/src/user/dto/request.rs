use derive_new::new;
use domain::user::enums::UserRole;
use serde::Deserialize;
use uuid::Uuid;

#[derive(new, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrCreateUserRequestDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}
