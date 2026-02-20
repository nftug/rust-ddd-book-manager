use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    book::{entity::Book, interface::BookRepository},
};

use crate::{
    book::dto::CreateBookRequestDTO,
    shared::{EntityCreationDTO, error::ApplicationError},
};

#[derive(new)]
pub struct CreateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
}

impl CreateBookService {
    pub async fn execute(
        &self,
        actor: &Actor,
        request: &CreateBookRequestDTO,
    ) -> Result<EntityCreationDTO, ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let book = Book::create_new(
            &context,
            request.title.clone().try_into()?,
            request.author_names.clone().try_into()?,
            request.isbn.clone().try_into()?,
            request.description.clone().try_into()?,
            actor.into(),
        )?;

        self.book_repository.save(&book).await?;

        Ok(book.audit().into())
    }
}
