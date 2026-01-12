use uuid::Uuid;

use crate::user::{
    enums::UserRole,
    values::{UserId, UserReference},
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn hydrate(id: Uuid, name: String, role: UserRole) -> Self {
        Actor {
            user: UserReference::hydrate(id, name),
            role,
        }
    }

    pub fn new_system() -> Self {
        Actor {
            user: UserReference::hydrate(Uuid::default(), "System".to_string()),
            role: UserRole::System,
        }
    }
}

impl From<Actor> for UserReference {
    fn from(actor: Actor) -> Self {
        actor.user
    }
}
