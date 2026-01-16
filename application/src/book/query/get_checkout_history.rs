use std::sync::Arc;

use derive_new::new;
use garde::Validate;
use uuid::Uuid;

use crate::{
    book::{
        dto::{CheckoutHistoryDTO, CheckoutHistoryQueryDTO},
        interface::BookQueryService,
    },
    shared::error::ApplicationError,
};

#[derive(new)]
pub struct GetCheckoutHistoryService {
    book_query_service: Arc<dyn BookQueryService>,
}

impl GetCheckoutHistoryService {
    pub async fn execute(
        &self,
        book_id: Uuid,
        query: CheckoutHistoryQueryDTO,
    ) -> Result<CheckoutHistoryDTO, ApplicationError> {
        query.validate()?;

        self.book_query_service
            .get_checkout_history(book_id, &query)
            .await
            .map_err(|e| e.into())
    }
}
