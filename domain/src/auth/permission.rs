use derive_new::new;

use crate::{
    auth::Actor,
    user::{UserId, UserRole},
};

pub trait Permission {
    fn can_create(&self) -> bool;
    fn can_update(&self) -> bool;
    fn can_delete(&self) -> bool;
}

#[derive(Debug, new, PartialEq, Eq)]
pub struct AdminPermission(Actor);

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

#[derive(Debug, new, PartialEq, Eq)]
pub struct SystemPermission(Actor);

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

#[derive(Debug, new, PartialEq, Eq)]
pub struct EntityPermission {
    pub actor: Actor,
    pub created_by_id: UserId,
}

impl Permission for EntityPermission {
    fn can_create(&self) -> bool {
        self.actor.role != UserRole::Regular || self.actor.id() == self.created_by_id
    }
    fn can_update(&self) -> bool {
        self.actor.role != UserRole::Regular || self.actor.id() == self.created_by_id
    }
    fn can_delete(&self) -> bool {
        self.actor.role != UserRole::Regular || self.actor.id() == self.created_by_id
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
