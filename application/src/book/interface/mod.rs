use async_trait::async_trait;
use domain::{auth::Actor, shared::error::PersistenceError};
use uuid::Uuid;

use crate::book::dto::{BookListQueryDTO, BookListResponseDTO, BookResponseDTO};

#[async_trait]
pub trait BookQueryService: Send + Sync {
    async fn get_book_details(
        &self,
        actor: &Actor,
        book_id: Uuid,
    ) -> Result<Option<BookResponseDTO>, PersistenceError>;

    async fn get_book_list(
        &self,
        actor: &Actor,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError>;
}
