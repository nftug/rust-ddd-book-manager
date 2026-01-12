use std::sync::Arc;

use derive_new::new;
use uuid::Uuid;

use crate::{
    shared::error::ApplicationError,
    user::{dto::UserDetailsDTO, interface::UserQueryService},
};

#[derive(new)]
pub struct GetUserDetailsService {
    user_query_service: Arc<dyn UserQueryService>,
}

impl GetUserDetailsService {
    pub async fn execute(&self, user_id: Uuid) -> Result<UserDetailsDTO, ApplicationError> {
        self.user_query_service
            .get_user_details(user_id)
            .await
            .map_err(|e| e.into())
            .and_then(|opt| opt.ok_or(ApplicationError::NotFound))
    }
}
