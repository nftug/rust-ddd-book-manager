use application::user::{dto::UserDetailsDTO, interface::UserQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{shared::error::PersistenceError, user::values::UserId};
use sea_orm::EntityTrait;

use crate::database::{ConnectionPool, entity::users, log_db_error, row::user::UserDetailsDTORow};

#[derive(new)]
pub struct UserQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserQueryService for UserQueryServiceImpl {
    async fn get_user_details(
        &self,
        user_id: UserId,
    ) -> Result<Option<UserDetailsDTO>, PersistenceError> {
        let result = users::Entity::find_by_id(user_id)
            .into_partial_model::<UserDetailsDTORow>()
            .one(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        match result {
            Some(user) => user.to_dto().map(Some),
            None => Ok(None),
        }
    }
}
