use async_trait::async_trait;

use crate::{
    audit::Actor,
    shared::error::PersistenceError,
    user::{entity::User, values::UserId},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find(&self, id: UserId) -> Result<Option<User>, PersistenceError>;
    async fn save(&self, user: &User) -> Result<(), PersistenceError>;
    async fn delete(&self, id: UserId) -> Result<(), PersistenceError>;
}

#[async_trait]
pub trait UserDomainQueryService: Send + Sync {
    async fn find_actor_by_id(&self, id: UserId) -> Result<Option<Actor>, PersistenceError>;
}
