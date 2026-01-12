use crate::{
    audit::{AuditContext, EntityAudit},
    auth::{
        Actor,
        permission::{AdminPermission, EntityPermission, Permission},
    },
    book::values::*,
    shared::error::DomainError,
    user::values::UserReference,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    audit: EntityAudit<BookId>,
    title: BookTitle,
    author: BookAuthor,
    isbn: BookIsbn,
    description: BookDescription,
    owner: BookOwner,
}

impl Book {
    pub fn audit(&self) -> &EntityAudit<BookId> {
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

    pub fn hydrate(
        audit: EntityAudit<BookId>,
        title: String,
        author: String,
        isbn: Option<String>,
        description: Option<String>,
        owner: UserReference,
    ) -> Self {
        Book {
            audit,
            title: BookTitle::hydrate(title),
            author: BookAuthor::hydrate(author),
            isbn: BookIsbn::hydrate(isbn),
            description: BookDescription::hydrate(description),
            owner: BookOwner::hydrate(owner),
        }
    }

    pub fn create_new(
        context: &AuditContext,
        title: BookTitle,
        author: BookAuthor,
        isbn: BookIsbn,
        description: BookDescription,
        owner: BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), owner.id());
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
        title: BookTitle,
        author: BookAuthor,
        isbn: BookIsbn,
        description: BookDescription,
    ) -> Result<Self, DomainError> {
        let permission = self.permission_to_update(context.actor());
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
        let permission = self.permission_to_update(context.actor());
        if !permission.can_delete() {
            return Err(DomainError::Forbidden);
        }

        Ok(())
    }

    pub fn change_owner(
        self,
        context: &AuditContext,
        owner: BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = AdminPermission::new(context.actor());
        let audit = self.audit.mark_updated(context, &permission)?;

        Ok(Book {
            audit,
            owner,
            ..self
        })
    }

    fn permission_to_update(&self, actor: &Actor) -> EntityPermission {
        EntityPermission::new(Some(actor), self.audit.created_by().id())
    }
}
