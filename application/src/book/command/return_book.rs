use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    book::interface::BookRepository,
};

use crate::{book::dto::BookIdentity, shared::error::ApplicationError};

#[derive(new)]
pub struct ReturnBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl ReturnBookService {
    pub async fn execute(
        &self,
        actor: &Actor,
        identity: BookIdentity,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let mut book = self
            .book_repository
            .find_by_id(identity.book_id)
            .await?
            .ok_or(ApplicationError::NotFound)?;

        book.do_return(&context)?;

        self.book_repository.save(&book).await?;

        Ok(())
    }
}
