use async_trait::async_trait;
use derive_new::new;
use domain::{
    audit::Actor,
    shared::error::PersistenceError,
    user::{interface::UserDomainQueryService, values::UserId},
};
use sea_orm::EntityTrait;

use crate::database::{ConnectionPool, entity::users, log_db_error, row::user::ActorRow};

#[derive(new)]
pub struct UserDomainQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserDomainQueryService for UserDomainQueryServiceImpl {
    async fn find_actor_by_id(&self, id: UserId) -> Result<Option<Actor>, PersistenceError> {
        let result = users::Entity::find_by_id(id)
            .into_partial_model::<ActorRow>()
            .one(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        match result {
            Some(actor_row) => Ok(Some(actor_row.to_actor()?)),
            None => Ok(None),
        }
    }
}
