use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    author::values::AuthorName,
    book::{interface::BookRepository, values::BookAuthorList},
};

use crate::{
    author::service::AuthorsFactoryService,
    book::dto::{BookIdentity, UpdateBookRequestDTO},
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
        identity: BookIdentity,
        request: &UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let mut book = self
            .book_repository
            .find_by_id(identity.book_id)
            .await?
            .ok_or(ApplicationError::NotFound)?;

        let author_names = request
            .author_names
            .iter()
            .map(|name| name.clone().try_into())
            .collect::<Result<Vec<AuthorName>, _>>()?;

        let author_refs = self
            .authors_factory_service
            .ensure_authors_exist(&context, &author_names)
            .await?;

        book.update(
            &context,
            request.title.clone().try_into()?,
            BookAuthorList::try_new(author_names, author_refs)?,
            request.isbn.clone().try_into()?,
            request.description.clone().try_into()?,
        )?;

        self.book_repository.save(&book).await?;

        Ok(())
    }
}
