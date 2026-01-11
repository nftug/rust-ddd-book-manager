use std::sync::Arc;

use derive_new::new;
use domain::audit::AuditContext;
use uuid::Uuid;

use crate::{
    book::{dto::BookResponseDTO, interface::BookQueryService},
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct GetBookDetailsService {
    book_query_service: Arc<dyn BookQueryService>,
}

impl GetBookDetailsService {
    pub async fn execute(
        &self,
        context: &AuditContext,
        book_id: Uuid,
    ) -> Result<Option<BookResponseDTO>, ApplicationError> {
        self.book_query_service
            .get_book_details(&context.actor, book_id)
            .await
            .map_err(|e| e.into())
    }
}
