use uuid::Uuid;

use crate::user::values::{UserId, UserName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserReference {
    id: UserId,
    name: UserName,
}

impl UserReference {
    pub fn hydrate(id: Uuid, name: String) -> Self {
        Self {
            id: id.into(),
            name: UserName::hydrate(name),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.raw()
    }
}
