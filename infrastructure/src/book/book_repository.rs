use async_trait::async_trait;
use derive_new::new;
use domain::{
    book::{entity::Book, interface::BookRepository, values::*},
    shared::{Id, error::PersistenceError},
};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{
    database::{
        ConnectionPool,
        entity::{books, users},
        row::book_rows::BookDetailsRow,
    },
    macros::audit_defaults,
};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn find(&self, id: BookId) -> Result<Option<Book>, PersistenceError> {
        let result = books::Entity::find_by_id(id.raw())
            .inner_join(users::Entity)
            .into_partial_model::<BookDetailsRow>()
            .one(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some(book) => Ok(Some(book.to_entity())),
            _ => Ok(None),
        }
    }

    async fn save(&self, book: &Book) -> Result<(), PersistenceError> {
        let active_model = books::ActiveModel {
            title: Set(book.title().into()),
            author: Set(book.author().into()),
            isbn: Set(book.isbn().map(|v| v.into())),
            description: Set(book.description().map(|v| v.into())),
            owner_id: Set(book.owner().id().raw()),
            ..audit_defaults!(books::ActiveModel, book.audit())
        };

        if book.audit().is_new() {
            books::Entity::insert(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;
        } else {
            books::Entity::update(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;
        }

        Ok(())
    }

    async fn delete(&self, id: BookId) -> Result<(), PersistenceError> {
        let result = books::Entity::delete_by_id(id.raw())
            .exec(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        if result.rows_affected == 0 {
            Err(PersistenceError::NotFound)
        } else {
            Ok(())
        }
    }
}
