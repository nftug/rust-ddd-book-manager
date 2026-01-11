use derive_new::new;

use crate::{auth::Actor, user::UserId};

pub trait Permission {
    fn can_create(&self) -> bool;
    fn can_update(&self) -> bool;
    fn can_delete(&self) -> bool;
}

#[derive(Debug, new, PartialEq, Eq)]
pub struct AdminPermission(Actor);

impl Permission for AdminPermission {
    fn can_create(&self) -> bool {
        self.0.is_admin()
    }

    fn can_update(&self) -> bool {
        self.0.is_admin()
    }

    fn can_delete(&self) -> bool {
        self.0.is_admin()
    }
}

#[derive(Debug, new, PartialEq, Eq)]
pub struct SystemPermission(Actor);

impl Permission for SystemPermission {
    fn can_create(&self) -> bool {
        self.0.is_system()
    }

    fn can_update(&self) -> bool {
        self.0.is_system()
    }

    fn can_delete(&self) -> bool {
        self.0.is_system()
    }
}

#[derive(Debug, new, PartialEq, Eq)]
pub struct EntityPermission {
    pub actor: Actor,
    pub created_by_id: UserId,
}

impl Permission for EntityPermission {
    fn can_create(&self) -> bool {
        self.actor.is_admin() || self.actor.id() == &self.created_by_id
    }

    fn can_update(&self) -> bool {
        self.actor.is_admin() || self.actor.id() == &self.created_by_id
    }

    fn can_delete(&self) -> bool {
        self.actor.is_admin() || self.actor.id() == &self.created_by_id
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
