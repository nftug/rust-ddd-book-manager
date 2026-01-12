use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{AuditContext, Clock},
    auth::Actor,
    book::{interface::BookRepository, values::*},
};
use uuid::Uuid;

use crate::{book::dto::UpdateBookRequestDTO, shared::error::ApplicationError};

#[derive(new)]
pub struct UpdateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl UpdateBookService {
    pub async fn execute(
        &self,
        actor: Actor,
        book_id: Uuid,
        request: UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor.clone(), self.clock.as_ref());

        let book = self
            .book_repository
            .find(book_id.into())
            .await?
            .ok_or(ApplicationError::NotFound)?;

        let updated_book = book.update(
            &context,
            BookTitle::try_new(request.title)?,
            BookAuthor::try_new(request.author)?,
            BookIsbn::try_new(request.isbn)?,
            BookDescription::try_new(request.description)?,
        )?;

        self.book_repository.save(&updated_book).await?;

        Ok(())
    }
}
