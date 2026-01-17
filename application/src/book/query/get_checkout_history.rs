use std::sync::Arc;

use derive_new::new;
use domain::audit::Actor;
use garde::Validate;

use crate::{
    book::{
        dto::{BookIdentity, CheckoutHistoryListDTO, CheckoutHistoryQueryDTO},
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
        actor: &Actor,
        identity: BookIdentity,
        query: &CheckoutHistoryQueryDTO,
    ) -> Result<CheckoutHistoryListDTO, ApplicationError> {
        if !actor.is_admin() {
            return Err(ApplicationError::Forbidden);
        }

        query.validate()?;

        self.book_query_service
            .get_checkout_history(identity, query)
            .await
            .map_err(|e| e.into())
    }
}
