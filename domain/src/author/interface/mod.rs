use async_trait::async_trait;

use crate::{
    author::{
        entity::Author,
        values::{AuthorId, AuthorName, AuthorReference},
    },
    shared::error::PersistenceError,
};

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn find_by_id(&self, id: AuthorId) -> Result<Option<Author>, PersistenceError>;
    async fn save(&self, author: &Author) -> Result<(), PersistenceError>;
    async fn delete(&self, id: AuthorId) -> Result<(), PersistenceError>;
}

#[async_trait]
pub trait AuthorDomainQueryService: Send + Sync {
    async fn find_author_refs_by_name(
        &self,
        names: &[AuthorName],
    ) -> Result<Vec<AuthorReference>, PersistenceError>;
}
