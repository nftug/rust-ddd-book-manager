use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    author::values::AuthorName,
    book::{interface::BookRepository, values::BookAuthorList},
};
use uuid::Uuid;

use crate::{
    author::service::AuthorsFactoryService, book::dto::UpdateBookRequestDTO,
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct UpdateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
    authors_factory_service: Arc<AuthorsFactoryService>,
}

impl UpdateBookService {
    pub async fn execute(
        &self,
        actor: &Actor,
        book_id: Uuid,
        request: UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let mut book = self
            .book_repository
            .find_by_id(book_id.into())
            .await?
            .ok_or(ApplicationError::NotFound)?;

        let author_names = request
            .author_names
            .into_iter()
            .map(|name| name.try_into())
            .collect::<Result<Vec<AuthorName>, _>>()?;

        let author_refs = self
            .authors_factory_service
            .ensure_authors_exist(&context, &author_names)
            .await?;

        book.update(
            &context,
            request.title.try_into()?,
            BookAuthorList::try_new(author_names, author_refs)?,
            request.isbn.try_into()?,
            request.description.try_into()?,
        )?;

        self.book_repository.save(&book).await?;

        Ok(())
    }
}
