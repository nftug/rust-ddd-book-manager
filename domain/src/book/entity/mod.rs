use derive_new::new;

use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::{AdminPermission, EntityPermission, Permission},
    book::values,
    shared::error::DomainError,
    user::values::UserReference,
};

#[derive(Debug, PartialEq, Eq, new)]
pub struct Book {
    pub(crate) audit: EntityAudit<values::BookId>,
    pub(crate) title: values::BookTitle,
    pub(crate) author: values::BookAuthor,
    pub(crate) isbn: values::BookIsbn,
    pub(crate) description: values::BookDescription,
    pub(crate) owner: values::BookOwner,
}

impl Book {
    pub fn audit(&self) -> &EntityAudit<values::BookId> {
        &self.audit
    }
    pub fn title(&self) -> &str {
        self.title.raw()
    }
    pub fn author(&self) -> &str {
        self.author.raw()
    }
    pub fn isbn(&self) -> Option<&str> {
        self.isbn.raw()
    }
    pub fn description(&self) -> Option<&str> {
        self.description.raw()
    }
    pub fn owner(&self) -> &UserReference {
        self.owner.raw()
    }

    pub fn create_new(
        context: &AuditContext,
        title: values::BookTitle,
        author: values::BookAuthor,
        isbn: values::BookIsbn,
        description: values::BookDescription,
        owner: values::BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = EntityPermission::new(context.actor.clone(), owner.id());
        let audit = EntityAudit::create_new(context, &permission)?;

        Ok(Book {
            audit,
            title,
            author,
            isbn,
            description,
            owner,
        })
    }

    pub fn update(
        self,
        context: &AuditContext,
        title: values::BookTitle,
        author: values::BookAuthor,
        isbn: values::BookIsbn,
        description: values::BookDescription,
    ) -> Result<Self, DomainError> {
        let permission = EntityPermission::new(context.actor.clone(), self.audit.created_by().id());
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(Book {
            audit,
            title,
            author,
            isbn,
            description,
            ..self
        })
    }

    pub fn validate_deletion(&self, context: &AuditContext) -> Result<(), DomainError> {
        let permission = EntityPermission::new(context.actor.clone(), self.audit.created_by().id());
        if !permission.can_delete() {
            return Err(DomainError::Forbidden);
        }

        Ok(())
    }

    pub fn change_owner(
        self,
        context: &AuditContext,
        owner: values::BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = AdminPermission::new(context.actor.clone());
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(Book {
            audit,
            owner,
            ..self
        })
    }
}
