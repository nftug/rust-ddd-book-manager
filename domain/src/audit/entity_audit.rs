use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    audit::AuditContext,
    auth::permission::Permission,
    shared::{Id, error::DomainError},
    user::values::UserReference,
};

#[derive(Debug, PartialEq, Eq)]
pub struct EntityAudit<EId: Id> {
    id: EId,
    created_at: DateTime<Utc>,
    created_by: UserReference,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<UserReference>,
    is_new: bool,
}

impl<EId: Id> EntityAudit<EId> {
    pub fn id(&self) -> EId {
        self.id
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn created_by(&self) -> &UserReference {
        &self.created_by
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
    pub fn updated_by(&self) -> Option<&UserReference> {
        self.updated_by.as_ref()
    }
    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn hydrate(
        id: Uuid,
        created_at: DateTime<Utc>,
        created_by_id: Uuid,
        created_by_name: String,
        updated_at: Option<DateTime<Utc>>,
        updated_by_id: Option<Uuid>,
        updated_by_name: Option<String>,
    ) -> Self {
        EntityAudit {
            id: id.into(),
            created_at,
            created_by: UserReference::hydrate(created_by_id, created_by_name),
            updated_at,
            updated_by: match (updated_by_id, updated_by_name) {
                (Some(id), Some(name)) => Some(UserReference::hydrate(id, name)),
                _ => None,
            },
            is_new: false,
        }
    }

    pub fn create_new(
        context: &AuditContext,
        permission: &dyn Permission,
    ) -> Result<Self, DomainError> {
        Self::create_new_with_id(context, permission, EId::new())
    }

    pub fn create_new_with_id(
        context: &AuditContext,
        permission: &dyn Permission,
        id: EId,
    ) -> Result<Self, DomainError> {
        if !permission.can_create() {
            return Err(DomainError::Forbidden);
        }

        Ok(EntityAudit {
            id,
            created_at: context.timestamp(),
            created_by: context.actor_user().clone(),
            updated_at: None,
            updated_by: None,
            is_new: true,
        })
    }

    pub fn mark_updated(
        self,
        context: &AuditContext,
        permission: &dyn Permission,
    ) -> Result<Self, DomainError> {
        if !permission.can_update() {
            return Err(DomainError::Forbidden);
        }

        Ok(EntityAudit {
            updated_at: Some(context.timestamp()),
            updated_by: Some(context.actor_user().clone()),
            ..self
        })
    }
}
