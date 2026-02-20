use crate::{
    audit::{Actor, AuditContext, EntityAudit},
    auth::permission::{AdminPermission, EntityPermission, Permission},
    book::values::*,
    shared::error::DomainError,
    user::values::UserReference,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    audit: EntityAudit<BookId>,
    title: BookTitle,
    authors: BookAuthorList,
    isbn: BookIsbn,
    description: BookDescription,
    owner: BookOwner,
    checkouts: BookCheckoutList,
}

impl Book {
    pub fn audit(&self) -> &EntityAudit<BookId> {
        &self.audit
    }
    pub fn title(&self) -> &str {
        self.title.raw()
    }
    pub fn authors(&self) -> &[OrderedAuthorReference] {
        self.authors.raw()
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
    pub fn checkouts(&self) -> &[BookCheckout] {
        self.checkouts.raw()
    }

    pub fn hydrate(
        audit: EntityAudit<BookId>,
        title: String,
        authors: Vec<(BookAuthorName, usize)>,
        isbn: Option<String>,
        description: Option<String>,
        owner: UserReference,
        checkouts: Vec<BookCheckout>,
    ) -> Self {
        Book {
            audit,
            title: BookTitle::hydrate(title),
            authors: BookAuthorList::hydrate(authors),
            isbn: BookIsbn::hydrate(isbn),
            description: BookDescription::hydrate(description),
            owner: BookOwner::hydrate(owner),
            checkouts: BookCheckoutList::hydrate(checkouts),
        }
    }

    pub fn create_new(
        context: &AuditContext,
        title: BookTitle,
        authors: BookAuthorList,
        isbn: BookIsbn,
        description: BookDescription,
        owner: BookOwner,
    ) -> Result<Self, DomainError> {
        let permission = EntityPermission::new(Some(context.actor()), owner.id());

        Ok(Self {
            audit: EntityAudit::create_new(context, &permission)?,
            title,
            authors,
            isbn,
            description,
            owner,
            checkouts: BookCheckoutList::hydrate(vec![]),
        })
    }

    pub fn update(
        &mut self,
        context: &AuditContext,
        title: BookTitle,
        authors: BookAuthorList,
        isbn: BookIsbn,
        description: BookDescription,
    ) -> Result<(), DomainError> {
        let permission = self.permission_to_update(context.actor());

        self.audit.mark_updated(context, &permission)?;
        self.title = title;
        self.authors = authors;
        self.isbn = isbn;
        self.description = description;

        Ok(())
    }

    pub fn validate_deletion(&self, context: &AuditContext) -> Result<(), DomainError> {
        let permission = self.permission_to_update(context.actor());

        match permission.can_delete() {
            true => Ok(()),
            false => Err(DomainError::Forbidden),
        }
    }

    pub fn do_checkout(&mut self, context: &AuditContext) -> Result<(), DomainError> {
        self.checkouts.do_checkout(context)
    }

    pub fn do_return(&mut self, context: &AuditContext) -> Result<(), DomainError> {
        self.checkouts.do_return(context)
    }

    pub fn change_owner(
        &mut self,
        context: &AuditContext,
        owner: BookOwner,
    ) -> Result<(), DomainError> {
        let permission = AdminPermission::new(context.actor());

        self.audit.mark_updated(context, &permission)?;
        self.owner = owner;

        Ok(())
    }

    fn permission_to_update(&self, actor: &Actor) -> EntityPermission {
        EntityPermission::new(Some(actor), self.owner.id())
    }
}
