use derive_new::new;

use crate::user::{UserId, UserReference, UserRole};

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct Actor {
    pub user: UserReference,
    pub role: UserRole,
}

impl Actor {
    pub fn new_system() -> Self {
        Actor {
            user: UserReference::new(UserId::default(), "System".into()),
            role: UserRole::System,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.user.user_id
    }

    pub fn username(&self) -> &str {
        &self.user.username
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn is_system(&self) -> bool {
        self.role == UserRole::System
    }
}
