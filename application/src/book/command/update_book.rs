use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{AuditContext, Clock},
    auth::Actor,
    book::interface::BookRepository,
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
        actor: &Actor,
        book_id: Uuid,
        request: UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());
        let book = self
            .book_repository
            .find(book_id.into())
            .await?
            .ok_or(ApplicationError::NotFound)?;

        let updated_book = book.update(
            &context,
            request.title.try_into()?,
            request.author.try_into()?,
            request.isbn.try_into()?,
            request.description.try_into()?,
        )?;

        self.book_repository.save(&updated_book).await?;

        Ok(())
    }
}
