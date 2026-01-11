use derive_new::new;

use crate::user::values::{UserId, UserName};

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct UserReference {
    id: UserId,
    name: UserName,
}

impl UserReference {
    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.raw()
    }
}
