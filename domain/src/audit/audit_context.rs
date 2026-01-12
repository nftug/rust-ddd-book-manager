use chrono::{DateTime, Utc};

use crate::{audit::Clock, auth::Actor, user::values::UserReference};

#[derive(Debug, PartialEq, Eq)]
pub struct AuditContext {
    actor: Actor,
    timestamp: DateTime<Utc>,
}

impl AuditContext {
    pub fn new(actor: &Actor, clock: &dyn Clock) -> Self {
        AuditContext {
            actor: actor.clone(),
            timestamp: clock.now(),
        }
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }
    pub fn actor_user(&self) -> &UserReference {
        &self.actor.user
    }
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
