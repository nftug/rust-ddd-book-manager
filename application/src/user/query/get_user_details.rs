use std::sync::Arc;

use crate::{
    shared::error::ApplicationError,
    user::{dto::UserDetailsDTO, interface::UserQueryService},
};
use derive_new::new;
use domain::user::values::UserId;

#[derive(new)]
pub struct GetUserDetailsService {
    user_query_service: Arc<dyn UserQueryService>,
}

impl GetUserDetailsService {
    pub async fn execute(&self, user_id: UserId) -> Result<UserDetailsDTO, ApplicationError> {
        self.user_query_service
            .get_user_details(user_id)
            .await
            .map_err(|e| e.into())
            .and_then(|opt| opt.ok_or(ApplicationError::NotFound))
    }
}
