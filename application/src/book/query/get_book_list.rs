use std::sync::Arc;

use derive_new::new;
use domain::audit::AuditContext;

use crate::{
    book::{
        dto::{BookListQueryDTO, BookListResponseDTO},
        interface::BookQueryService,
    },
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct GetBookListService {
    book_query_service: Arc<dyn BookQueryService>,
}

impl GetBookListService {
    pub async fn execute(
        &self,
        context: &AuditContext,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, ApplicationError> {
        self.book_query_service
            .get_book_list(&context.actor, query)
            .await
            .map_err(|e| e.into())
    }
}
