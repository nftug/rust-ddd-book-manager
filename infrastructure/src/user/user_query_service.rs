use application::user::{dto::UserDetailsDTO, interface::UserQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::shared::error::PersistenceError;
use sea_orm::{EntityTrait, QuerySelect};
use uuid::Uuid;

use crate::database::{ConnectionPool, entity::users, row::user_rows::UserDetailsRow};

#[derive(new)]
pub struct UserQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserQueryService for UserQueryServiceImpl {
    async fn get_user_details(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserDetailsDTO>, PersistenceError> {
        let result = users::Entity::find_by_id(user_id)
            .select_only()
            .columns([
                users::Column::Id,
                users::Column::Name,
                users::Column::Email,
                users::Column::Role,
            ])
            .into_model::<UserDetailsRow>()
            .one(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some(user) => user.to_dto().map(Some),
            None => Ok(None),
        }
    }
}
