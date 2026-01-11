use derive_new::new;

use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::SystemPermission,
    shared::error::DomainError,
    user::{enums, values},
};

#[derive(Debug, new, PartialEq, Eq)]
pub struct User {
    pub audit: EntityAudit<values::UserId>,
    pub name: values::UserName,
    pub email: values::UserEmail,
    pub role: enums::UserRole,
}

impl User {
    pub fn create_new(
        context: &AuditContext,
        name: values::UserName,
        email: values::UserEmail,
        role: enums::UserRole,
    ) -> Result<Self, DomainError> {
        let permission = SystemPermission::new(context.actor.clone());
        let audit = EntityAudit::create_new(context, &permission)?;

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
        let permission = SystemPermission::new(context.actor.clone());
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(User {
            audit,
            name,
            email,
            role,
        })
    }

    pub fn id(&self) -> &values::UserId {
        &self.audit.id
    }
}
