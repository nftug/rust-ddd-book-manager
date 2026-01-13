use std::sync::Arc;

use derive_new::new;
use domain::audit::Actor;
use uuid::Uuid;

use crate::{
    book::{dto::BookDetailsDTO, interface::BookQueryService},
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct GetBookDetailsService {
    book_query_service: Arc<dyn BookQueryService>,
}

impl GetBookDetailsService {
    pub async fn execute(
        &self,
        actor: Option<&Actor>,
        book_id: Uuid,
    ) -> Result<BookDetailsDTO, ApplicationError> {
        self.book_query_service
            .get_book_details(actor, book_id)
            .await
            .map_err(|e| e.into())
            .and_then(|opt| opt.ok_or(ApplicationError::NotFound))
    }
}
