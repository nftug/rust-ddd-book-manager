use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{AuditContext, Clock},
    auth::Actor,
    book::{entity::Book, interface::BookRepository, values::*},
    shared::Id,
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
        request: CreateBookRequestDTO,
    ) -> Result<EntityCreationDTO, ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let book = Book::create_new(
            &context,
            BookTitle::try_new(request.title)?,
            BookAuthor::try_new(request.author)?,
            BookIsbn::try_new(request.isbn)?,
            BookDescription::try_new(request.description)?,
            BookOwner::new(actor.clone().into()),
        )?;

        self.book_repository.save(&book).await?;

        Ok(EntityCreationDTO::new(book.audit().id().raw()))
    }
}
