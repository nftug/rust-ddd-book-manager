use derive_new::new;

use crate::user::values::UserId;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct UserReference {
    pub user_id: UserId,
    pub username: String,
}
