use derive_new::new;

use crate::user::{UserId, UserReference, UserRole};

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct Actor {
    pub(crate) user: UserReference,
    pub(crate) role: UserRole,
}

impl Actor {
    pub fn id(&self) -> UserId {
        self.user.id()
    }
    pub fn username(&self) -> &str {
        self.user.name()
    }

    pub fn new_system() -> Self {
        Actor {
            user: UserReference::new(UserId::default(), "System".into()),
            role: UserRole::System,
        }
    }
}
