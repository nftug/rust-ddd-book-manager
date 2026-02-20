use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    book::interface::BookRepository,
};

use crate::{
    book::dto::{BookIdentity, UpdateBookRequestDTO},
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct UpdateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl UpdateBookService {
    pub async fn execute(
        &self,
        actor: &Actor,
        identity: BookIdentity,
        request: &UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let mut book = self
            .book_repository
            .find_by_id(identity.book_id)
            .await?
            .ok_or(ApplicationError::NotFound)?;

        book.update(
            &context,
            request.title.clone().try_into()?,
            request.author_names.clone().try_into()?,
            request.isbn.clone().try_into()?,
            request.description.clone().try_into()?,
        )?;

        self.book_repository.save(&book).await?;

        Ok(())
    }
}
