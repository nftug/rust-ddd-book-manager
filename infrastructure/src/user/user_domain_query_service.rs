use async_trait::async_trait;
use derive_new::new;
use domain::{
    audit::Actor,
    shared::{Id, error::PersistenceError},
    user::{interface::UserDomainQueryService, values::UserId},
};
use sea_orm::EntityTrait;

use crate::database::{ConnectionPool, entity::users, row::user_rows::ActorRow};

#[derive(new)]
pub struct UserDomainQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserDomainQueryService for UserDomainQueryServiceImpl {
    async fn find_actor_by_id(&self, id: UserId) -> Result<Option<Actor>, PersistenceError> {
        let result = users::Entity::find_by_id(id.raw())
            .into_partial_model::<ActorRow>()
            .one(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some(actor_row) => Ok(Some(actor_row.to_actor()?)),
            None => Ok(None),
        }
    }
}
