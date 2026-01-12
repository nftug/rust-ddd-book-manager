use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{AuditContext, Clock},
    auth::Actor,
    book::{entity::Book, interface::BookRepository, values::*},
    shared::Id,
};
use uuid::Uuid;

use crate::{book::dto::CreateBookRequestDTO, shared::error::ApplicationError};

#[derive(new)]
pub struct CreateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl CreateBookService {
    pub async fn execute(
        &self,
        actor: Actor,
        request: CreateBookRequestDTO,
    ) -> Result<Uuid, ApplicationError> {
        let context = AuditContext::new(actor.clone(), self.clock.as_ref());

        let book = Book::create_new(
            &context,
            BookTitle::try_new(request.title)?,
            BookAuthor::try_new(request.author)?,
            BookIsbn::try_new(request.isbn)?,
            BookDescription::try_new(request.description)?,
            BookOwner::new(actor.clone().into()),
        )?;

        self.book_repository.save(&book).await?;

        Ok(book.audit().id().raw())
    }
}
