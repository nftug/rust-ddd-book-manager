use async_trait::async_trait;
use domain::{shared::error::PersistenceError, user::values::UserId};

use crate::user::dto::UserDetailsDTO;

#[async_trait]
pub trait UserQueryService: Send + Sync {
    async fn get_user_details(
        &self,
        user_id: UserId,
    ) -> Result<Option<UserDetailsDTO>, PersistenceError>;
}
