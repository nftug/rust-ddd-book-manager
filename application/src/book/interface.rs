use async_trait::async_trait;
use domain::{audit::Actor, shared::error::PersistenceError};

use crate::book::dto::*;

#[async_trait]
pub trait BookQueryService: Send + Sync {
    async fn get_book_details(
        &self,
        actor: Option<&Actor>,
        identity: BookIdentity,
    ) -> Result<Option<BookDetailsDTO>, PersistenceError>;

    async fn get_book_list(
        &self,
        actor: Option<&Actor>,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError>;

    async fn get_checkout_history(
        &self,
        identity: BookIdentity,
        query: &CheckoutHistoryQueryDTO,
    ) -> Result<CheckoutHistoryListDTO, PersistenceError>;
}
