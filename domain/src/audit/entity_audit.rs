use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    audit::AuditContext,
    auth::permission::Permission,
    shared::{EntityIdTrait, error::DomainError},
    user::values::UserReference,
};

#[derive(Debug, PartialEq, Eq)]
pub struct EntityAudit<EId: EntityIdTrait> {
    id: EId,
    created_at: DateTime<Utc>,
    created_by: UserReference,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<UserReference>,
}

impl<EId: EntityIdTrait> EntityAudit<EId> {
    pub fn id(&self) -> EId {
        self.id
    }
    pub fn raw_id(&self) -> Uuid {
        self.id.raw()
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
        })
    }

    pub fn mark_updated(
        &mut self,
        context: &AuditContext,
        permission: &dyn Permission,
    ) -> Result<(), DomainError> {
        if !permission.can_update() {
            return Err(DomainError::Forbidden);
        }

        self.updated_at = Some(context.timestamp());
        self.updated_by = Some(context.actor_user().clone());

        Ok(())
    }
}
