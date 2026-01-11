use derive_new::new;

use crate::user::values::UserId;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct UserReference {
    id: UserId,
    name: String,
}

impl UserReference {
    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
