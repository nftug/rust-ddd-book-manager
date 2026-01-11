use chrono::{DateTime, Utc};

use crate::{audit::Clock, auth::Actor};

#[derive(Debug, PartialEq, Eq)]
pub struct AuditContext {
    pub actor: Actor,
    pub timestamp: DateTime<Utc>,
}

impl AuditContext {
    pub fn new(actor: Actor, clock: &dyn Clock) -> Self {
        AuditContext {
            actor,
            timestamp: clock.now(),
        }
    }
}
