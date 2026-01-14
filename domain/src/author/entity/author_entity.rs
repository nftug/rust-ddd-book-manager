use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::{EntityPermission, PassThroughPermission, Permission},
    author::values::{AuthorId, AuthorName, AuthorReference},
    shared::error::DomainError,
};

pub struct Author {
    audit: EntityAudit<AuthorId>,
    name: AuthorName,
}

impl Author {
    pub fn audit(&self) -> &EntityAudit<AuthorId> {
        &self.audit
    }
    pub fn name(&self) -> &str {
        self.name.raw()
    }
    pub fn reference(&self) -> AuthorReference {
        AuthorReference::from_values(self.audit().id(), self.name.clone())
    }

    pub fn hydrate(audit: EntityAudit<AuthorId>, name: String) -> Self {
        Self {
            audit,
            name: AuthorName::hydrate(name),
        }
    }

    pub fn create_new(context: &AuditContext, name: AuthorName) -> Result<Self, DomainError> {
        let permission = PassThroughPermission::new();

        Ok(Self {
            audit: EntityAudit::create_new(context, &permission)?,
            name,
        })
    }

    pub fn update(&mut self, context: &AuditContext, name: AuthorName) -> Result<(), DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), self.audit.created_by().id());

        self.audit.mark_updated(context, &permission)?;
        self.name = name;

        Ok(())
    }

    pub fn validate_deletion(&self, context: &AuditContext) -> Result<(), DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), self.audit.created_by().id());

        match permission.can_delete() {
            true => Ok(()),
            false => Err(DomainError::Forbidden),
        }
    }
}
