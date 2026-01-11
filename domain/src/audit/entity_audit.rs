use chrono::{DateTime, Utc};
use derive_new::new;

use crate::{
    audit::AuditContext,
    auth::permission::Permission,
    shared::{Id, error::DomainError},
    user::UserReference,
};

#[derive(new, Debug, PartialEq, Eq)]
pub struct EntityAudit<EId: Id> {
    pub id: EId,
    pub created_at: DateTime<Utc>,
    pub created_by: UserReference,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<UserReference>,
}

impl<EId: Id> EntityAudit<EId> {
    pub fn create_new(
        context: &AuditContext,
        permission: &impl Permission,
    ) -> Result<Self, DomainError> {
        if !permission.can_create() {
            return Err(DomainError::Forbidden);
        }

        let created_by = context.actor.user.clone();

        Ok(EntityAudit {
            id: EId::new(),
            created_at: context.timestamp,
            created_by,
            updated_at: None,
            updated_by: None,
        })
    }

    pub fn mark_updated(
        self,
        context: &AuditContext,
        permission: &impl Permission,
    ) -> Result<Self, DomainError> {
        if !permission.can_update() {
            return Err(DomainError::Forbidden);
        }

        let updated_by = Some(context.actor.user.clone());

        Ok(EntityAudit {
            updated_at: Some(context.timestamp),
            updated_by,
            ..self
        })
    }
}
