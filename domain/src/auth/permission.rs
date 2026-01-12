use derive_new::new;

use crate::{auth::Actor, user::enums::UserRole, user::values::UserId};

pub trait Permission {
    fn can_create(&self) -> bool;
    fn can_update(&self) -> bool;
    fn can_delete(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub struct AdminPermission(Actor);

impl AdminPermission {
    pub fn new(actor: &Actor) -> Self {
        AdminPermission(actor.clone())
    }
}

impl Permission for AdminPermission {
    fn can_create(&self) -> bool {
        self.0.role == UserRole::Admin
    }
    fn can_update(&self) -> bool {
        self.0.role == UserRole::Admin
    }
    fn can_delete(&self) -> bool {
        self.0.role == UserRole::Admin
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SystemPermission(Actor);

impl SystemPermission {
    pub fn new(actor: &Actor) -> Self {
        SystemPermission(actor.clone())
    }
}

impl Permission for SystemPermission {
    fn can_create(&self) -> bool {
        self.0.role == UserRole::System
    }
    fn can_update(&self) -> bool {
        self.0.role == UserRole::System
    }
    fn can_delete(&self) -> bool {
        self.0.role == UserRole::System
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPermission {
    actor: Option<Actor>,
    owner_user_id: UserId,
}

impl EntityPermission {
    pub fn new(actor: Option<&Actor>, owner_user_id: UserId) -> Self {
        EntityPermission {
            actor: actor.cloned(),
            owner_user_id,
        }
    }

    fn can_edit(&self) -> bool {
        match self.actor {
            Some(ref actor) => actor.role != UserRole::Regular || actor.id() == self.owner_user_id,
            None => false,
        }
    }
}

impl Permission for EntityPermission {
    fn can_create(&self) -> bool {
        self.can_edit()
    }
    fn can_update(&self) -> bool {
        self.can_edit()
    }
    fn can_delete(&self) -> bool {
        self.can_edit()
    }
}

#[derive(Debug, new, PartialEq, Eq)]
pub struct PassThroughPermission;

impl Permission for PassThroughPermission {
    fn can_create(&self) -> bool {
        true
    }
    fn can_update(&self) -> bool {
        true
    }
    fn can_delete(&self) -> bool {
        true
    }
}
