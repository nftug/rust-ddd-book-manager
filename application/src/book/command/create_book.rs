use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    author::values::AuthorName,
    book::{entity::Book, interface::BookRepository, values::BookAuthorList},
};

use crate::{
    author::service::AuthorsFactoryService,
    book::dto::CreateBookRequestDTO,
    shared::{EntityCreationDTO, error::ApplicationError},
};

#[derive(new)]
pub struct CreateBookService {
    clock: Arc<dyn Clock>,
    book_repository: Arc<dyn BookRepository>,
    authors_factory_service: Arc<AuthorsFactoryService>,
}

impl CreateBookService {
    pub async fn execute(
        &self,
        actor: &Actor,
        request: &CreateBookRequestDTO,
    ) -> Result<EntityCreationDTO, ApplicationError> {
        let context = AuditContext::new(actor, self.clock.as_ref());

        let author_names = request
            .author_names
            .iter()
            .map(|name| name.clone().try_into())
            .collect::<Result<Vec<AuthorName>, _>>()?;

        let authors_refs = self
            .authors_factory_service
            .ensure_authors_exist(&context, &author_names)
            .await?;

        let book = Book::create_new(
            &context,
            request.title.clone().try_into()?,
            BookAuthorList::try_new(author_names, authors_refs)?,
            request.isbn.clone().try_into()?,
            request.description.clone().try_into()?,
            actor.into(),
        )?;

        self.book_repository.save(&book).await?;

        Ok(book.audit().into())
    }
}
