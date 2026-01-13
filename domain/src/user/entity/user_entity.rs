use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::{EntityPermission, PassThroughPermission},
    shared::error::DomainError,
    user::{enums::*, values::*},
};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    audit: EntityAudit<UserId>,
    name: UserName,
    email: UserEmail,
    role: UserRole,
}

impl User {
    pub fn audit(&self) -> &EntityAudit<UserId> {
        &self.audit
    }
    pub fn name(&self) -> &str {
        self.name.raw()
    }
    pub fn email(&self) -> &str {
        self.email.raw()
    }
    pub fn role(&self) -> UserRole {
        self.role
    }

    pub fn hydrate(
        audit: EntityAudit<UserId>,
        name: String,
        email: String,
        role: UserRole,
    ) -> Self {
        Self {
            audit,
            name: UserName::hydrate(name),
            email: UserEmail::hydrate(email),
            role,
        }
    }

    pub fn create_new(
        context: &AuditContext,
        user_id: UserId,
        name: UserName,
        email: UserEmail,
        role: UserRole,
    ) -> Result<Self, DomainError> {
        let permission = PassThroughPermission::new();

        Ok(Self {
            audit: EntityAudit::create_new_with_id(context, &permission, user_id)?,
            name,
            email,
            role,
        })
    }

    pub fn update(
        &mut self,
        context: &AuditContext,
        name: UserName,
        email: UserEmail,
        role: UserRole,
    ) -> Result<(), DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), self.audit.id());

        self.audit.mark_updated(context, &permission)?;
        self.name = name;
        self.email = email;
        self.role = role;

        Ok(())
    }
}
