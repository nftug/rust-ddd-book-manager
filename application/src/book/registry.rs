use std::sync::Arc;

use domain::{audit::Clock, auth::Actor, book::interface::BookRepository};
use uuid::Uuid;

use crate::{
    book::{command::*, dto::*, interface::BookQueryService, query::*},
    shared::error::ApplicationError,
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
        book_repository: Arc<dyn BookRepository>,
        book_queryservice: Arc<dyn BookQueryService>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let create_book = CreateBookService::new(clock.clone(), book_repository.clone());
        let update_book = UpdateBookService::new(clock.clone(), book_repository.clone());
        let delete_book = DeleteBookService::new(clock.clone(), book_repository.clone());
        let get_book_details = GetBookDetailsService::new(book_queryservice.clone());
        let get_book_list = GetBookListService::new(book_queryservice.clone());

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
    ) -> Result<Uuid, ApplicationError> {
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
    ) -> Result<Option<BookResponseDTO>, ApplicationError> {
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
