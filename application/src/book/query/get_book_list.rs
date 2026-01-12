use std::sync::Arc;

use derive_new::new;
use domain::auth::Actor;

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
        actor: Actor,
        query: BookListQueryDTO,
    ) -> Result<BookListResponseDTO, ApplicationError> {
        self.book_query_service
            .get_book_list(&actor, &query)
            .await
            .map_err(|e| e.into())
    }
}
