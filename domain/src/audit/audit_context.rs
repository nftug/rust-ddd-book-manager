use chrono::{DateTime, Utc};

use crate::{audit::Clock, auth::Actor};

#[derive(Debug, PartialEq, Eq)]
pub struct AuditContext {
    pub actor: Actor,
    pub timestamp: DateTime<Utc>,
}

impl AuditContext {
    pub fn new<C: Clock>(actor: Actor, clock: &C) -> Self {
        AuditContext {
            actor,
            timestamp: clock.now(),
        }
    }
}

impl From<AuditContext> for Actor {
    fn from(context: AuditContext) -> Self {
        context.actor
    }
}
