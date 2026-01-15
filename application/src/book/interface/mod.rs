use async_trait::async_trait;
use domain::{audit::Actor, shared::error::PersistenceError};
use uuid::Uuid;

use crate::{book::dto::*, shared::PaginationDTO};

#[async_trait]
pub trait BookQueryService: Send + Sync {
    async fn get_book_details(
        &self,
        actor: Option<&Actor>,
        book_id: Uuid,
    ) -> Result<Option<BookDetailsDTO>, PersistenceError>;

    async fn get_book_list(
        &self,
        actor: Option<&Actor>,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError>;

    async fn get_checkout_history(
        &self,
        book_id: Uuid,
        query: &CheckoutHistoryQueryDTO,
    ) -> Result<PaginationDTO<CheckoutDTO>, PersistenceError>;
}
