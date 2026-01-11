use derive_new::new;

use crate::user::{
    entity::User,
    enums::UserRole,
    values::{UserId, UserName, UserReference},
};

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
            user: UserReference::new(UserId::default(), UserName::new("System".to_string())),
            role: UserRole::System,
        }
    }
}

impl From<Actor> for UserReference {
    fn from(actor: Actor) -> Self {
        actor.user
    }
}

impl From<User> for Actor {
    fn from(user: User) -> Self {
        Actor {
            user: UserReference::new(user.audit().id(), UserName::new(user.name().to_string())),
            role: user.role().clone(),
        }
    }
}
