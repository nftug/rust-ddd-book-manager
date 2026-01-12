use derive_new::new;

use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::{EntityPermission, PassThroughPermission},
    shared::error::DomainError,
    user::{enums, values},
};

#[derive(Debug, new, PartialEq, Eq)]
pub struct User {
    pub(crate) audit: EntityAudit<values::UserId>,
    pub(crate) name: values::UserName,
    pub(crate) email: values::UserEmail,
    pub(crate) role: enums::UserRole,
}

impl User {
    pub fn audit(&self) -> &EntityAudit<values::UserId> {
        &self.audit
    }
    pub fn name(&self) -> &str {
        self.name.raw()
    }
    pub fn email(&self) -> &str {
        self.email.raw()
    }
    pub fn role(&self) -> &enums::UserRole {
        &self.role
    }

    pub fn create_new(
        context: &AuditContext,
        user_id: values::UserId,
        name: values::UserName,
        email: values::UserEmail,
        role: enums::UserRole,
    ) -> Result<Self, DomainError> {
        let permission = PassThroughPermission::new();
        let audit = EntityAudit::create_new_with_id(context, &permission, user_id)?;

        Ok(User {
            audit,
            name,
            email,
            role,
        })
    }

    pub fn update(
        self,
        context: &AuditContext,
        name: values::UserName,
        email: values::UserEmail,
        role: enums::UserRole,
    ) -> Result<Self, DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), self.audit.id());
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(User {
            audit,
            name,
            email,
            role,
        })
    }
}
