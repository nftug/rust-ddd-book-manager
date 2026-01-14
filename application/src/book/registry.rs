use std::sync::Arc;

use domain::{
    audit::{Actor, Clock},
    book::interface::BookRepository,
};
use uuid::Uuid;

use crate::{
    author::service::AuthorsFactoryService,
    book::{command::*, dto::*, interface::BookQueryService, query::*},
    shared::{EntityCreationDTO, error::ApplicationError},
};

pub struct BookRegistry {
    create_book: Arc<CreateBookService>,
    update_book: Arc<UpdateBookService>,
    delete_book: Arc<DeleteBookService>,
    get_book_details: Arc<GetBookDetailsService>,
    get_book_list: Arc<GetBookListService>,
}

impl BookRegistry {
    pub fn new(
        repository: Arc<dyn BookRepository>,
        query_service: Arc<dyn BookQueryService>,
        authors_factory_service: Arc<AuthorsFactoryService>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let create_book = CreateBookService::new(
            clock.clone(),
            repository.clone(),
            authors_factory_service.clone(),
        );
        let update_book = UpdateBookService::new(
            clock.clone(),
            repository.clone(),
            authors_factory_service.clone(),
        );
        let delete_book = DeleteBookService::new(clock.clone(), repository.clone());
        let get_book_details = GetBookDetailsService::new(query_service.clone());
        let get_book_list = GetBookListService::new(query_service.clone());

        BookRegistry {
            create_book: Arc::new(create_book),
            update_book: Arc::new(update_book),
            delete_book: Arc::new(delete_book),
            get_book_details: Arc::new(get_book_details),
            get_book_list: Arc::new(get_book_list),
        }
    }

    pub async fn create_book(
        &self,
        actor: &Actor,
        request: CreateBookRequestDTO,
    ) -> Result<EntityCreationDTO, ApplicationError> {
        self.create_book.execute(actor, request).await
    }

    pub async fn update_book(
        &self,
        actor: &Actor,
        book_id: Uuid,
        request: UpdateBookRequestDTO,
    ) -> Result<(), ApplicationError> {
        self.update_book.execute(actor, book_id, request).await
    }

    pub async fn delete_book(&self, actor: &Actor, book_id: Uuid) -> Result<(), ApplicationError> {
        self.delete_book.execute(actor, book_id).await
    }

    pub async fn get_book_details(
        &self,
        actor: Option<&Actor>,
        book_id: Uuid,
    ) -> Result<BookDetailsDTO, ApplicationError> {
        self.get_book_details.execute(actor, book_id).await
    }

    pub async fn get_book_list(
        &self,
        actor: Option<&Actor>,
        query: BookListQueryDTO,
    ) -> Result<BookListResponseDTO, ApplicationError> {
        self.get_book_list.execute(actor, query).await
    }
}
