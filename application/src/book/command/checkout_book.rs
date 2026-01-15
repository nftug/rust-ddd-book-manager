use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    book::interface::BookRepository,
};
use uuid::Uuid;

use crate::shared::error::ApplicationError;

#[derive(new)]
pub struct CheckoutBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl CheckoutBookService {
    pub async fn execute(&self, actor: &Actor, book_id: Uuid) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let mut book = self
            .book_repository
            .find_by_id(book_id.into())
            .await?
            .ok_or(ApplicationError::NotFound)?;

        book.do_checkout(&context)?;

        self.book_repository.save(&book).await?;

        Ok(())
    }
}
