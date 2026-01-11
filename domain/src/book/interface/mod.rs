use async_trait::async_trait;

use crate::{
    book::{entity::Book, values::BookId},
    shared::error::PersistenceError,
};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find(&self, id: BookId) -> Result<Option<Book>, PersistenceError>;
    async fn save(&self, book: &Book) -> Result<(), PersistenceError>;
    async fn delete(&self, id: BookId) -> Result<(), PersistenceError>;
}
