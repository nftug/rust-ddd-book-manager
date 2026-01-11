use async_trait::async_trait;
use derive_new::new;
use domain::{
    book::{
        entity::Book,
        interface::BookRepository,
        values::{BookAuthor, BookDescription, BookId, BookIsbn, BookOwner, BookTitle},
    },
    shared::{Id, error::PersistenceError},
    user::UserReference,
};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{
    database::{
        ConnectionPool,
        entity::{books, users},
        row::user_row,
    },
    macros::{audit_defaults, audit_defaults_update, hydrate_audit},
};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn find(&self, id: BookId) -> Result<Option<Book>, PersistenceError> {
        let result: Option<(books::Model, Option<user_row::UserReferenceRow>)> =
            books::Entity::find_by_id(id.raw())
                .inner_join(users::Entity)
                .select_also(users::Entity)
                .into_model()
                .one(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some((book, Some(owner))) => {
                let audit = hydrate_audit!(book, BookId);
                Ok(Some(Book::new(
                    audit,
                    BookTitle::new(book.title),
                    BookAuthor::new(book.author),
                    BookIsbn::new(book.isbn),
                    BookDescription::new(book.description),
                    BookOwner::new(UserReference::new(owner.id.into(), owner.name)),
                )))
            }
            _ => Ok(None),
        }
    }

    async fn save(&self, book: &Book) -> Result<(), PersistenceError> {
        if book.audit().is_new() {
            let active_model = books::ActiveModel {
                title: Set(book.title().into()),
                author: Set(book.author().into()),
                isbn: Set(book.isbn().map(|v| v.into())),
                description: Set(book.description().map(|v| v.into())),
                owner_id: Set(book.owner().id().raw()),
                ..audit_defaults!(books::ActiveModel, book.audit())
            };

            books::Entity::insert(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;
        } else {
            let mut active_model: books::ActiveModel =
                books::Entity::find_by_id(book.audit().id().raw())
                    .one(self.db.inner_ref())
                    .await
                    .map_err(|_| PersistenceError::OperationError)?
                    .ok_or(PersistenceError::NotFound)?
                    .into();

            active_model.title = Set(book.title().into());
            active_model.author = Set(book.author().into());
            active_model.isbn = Set(book.isbn().map(|v| v.into()));
            active_model.description = Set(book.description().map(|v| v.into()));
            active_model.owner_id = Set(book.owner().id().raw());
            audit_defaults_update!(active_model, book.audit());

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
