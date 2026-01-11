use derive_new::new;

use crate::{
    audit::{AuditContext, EntityAudit},
    auth::permission::{AdminPermission, PassThroughPermission},
    book::values,
    shared::error::DomainError,
};

#[derive(Debug, new, PartialEq, Eq)]
pub struct Book {
    pub audit: EntityAudit<values::BookId>,
    pub title: values::BookTitle,
    pub author: values::BookAuthor,
    pub isbn: values::BookIsbn,
    pub description: values::BookDescription,
    pub owner: values::BookOwner,
}

impl Book {
    pub fn create_new(
        context: &AuditContext,
        title: values::BookTitle,
        author: values::BookAuthor,
        isbn: values::BookIsbn,
        description: values::BookDescription,
    ) -> Result<Self, DomainError> {
        let permission = AdminPermission::new(context.actor.clone());
        let audit = EntityAudit::create_new(context, &permission)?;

        Ok(Book {
            audit,
            title,
            author,
            isbn,
            description,
            owner: values::BookOwner::default(),
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
        let permission = AdminPermission::new(context.actor.clone());
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

    pub fn change_owner(
        self,
        context: &AuditContext,
        owner: values::BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = PassThroughPermission::new();
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(Book {
            audit,
            owner,
            ..self
        })
    }
}
