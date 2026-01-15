use async_trait::async_trait;
use derive_new::new;
use domain::{
    book::{entity::Book, interface::BookRepository, values::*},
    shared::{Id, error::PersistenceError},
};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    TransactionTrait,
};

use crate::{
    database::{
        ConnectionPool,
        entity::{authors, book_authors, book_checkouts, books, users},
        log_db_error,
        row::book::{aggregate::AggregatedBookDetails, rows::BookDetailsRow},
    },
    macros::audit_defaults,
};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn find_by_id(&self, id: BookId) -> Result<Option<Book>, PersistenceError> {
        let rows = books::Entity::find_by_id(id.raw())
            .inner_join(authors::Entity)
            .inner_join(users::Entity)
            .left_join(book_checkouts::Entity)
            .order_by_asc(book_authors::Column::OrderIndex)
            .into_partial_model::<BookDetailsRow>()
            .all(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(AggregatedBookDetails::from_rows(rows).map(|agg| agg.to_entity()))
    }

    async fn save(&self, book: &Book) -> Result<(), PersistenceError> {
        // Begin transaction
        let txn = self.db.inner_ref().begin().await.map_err(log_db_error)?;

        // Upsert book
        let book_active_model = books::ActiveModel {
            title: Set(book.title().into()),
            isbn: Set(book.isbn().map(|v| v.into())),
            description: Set(book.description().map(|v| v.into())),
            owner_id: Set(book.owner().raw_id()),
            ..audit_defaults!(books::ActiveModel, book.audit())
        };

        let exists = books::Entity::find()
            .filter(books::Column::Id.eq(book.audit().raw_id()))
            .count(&txn)
            .await
            .map_err(log_db_error)?
            > 0;

        if exists {
            books::Entity::update(book_active_model)
                .exec(&txn)
                .await
                .map_err(log_db_error)?;
        } else {
            books::Entity::insert(book_active_model)
                .exec(&txn)
                .await
                .map_err(log_db_error)?;
        }

        // Upsert book authors
        let book_authors = book
            .authors()
            .iter()
            .map(|author_ref| book_authors::ActiveModel {
                book_id: Set(book.audit().raw_id()),
                author_id: Set(author_ref.raw_id()),
                order_index: Set(author_ref.order_index() as i32),
            })
            .collect::<Vec<_>>();

        book_authors::Entity::delete_many()
            .filter(book_authors::Column::BookId.eq(book.audit().raw_id()))
            .exec(&txn)
            .await
            .map_err(log_db_error)?;
        book_authors::Entity::insert_many(book_authors)
            .exec(&txn)
            .await
            .map_err(log_db_error)?;

        // Upsert book checkouts
        let book_checkouts = book
            .checkouts()
            .iter()
            .map(|checkout| book_checkouts::ActiveModel {
                checkout_id: Set(checkout.id()),
                book_id: Set(book.audit().raw_id()),
                checked_out_at: Set(checkout.checked_out_at().into()),
                checked_out_by_id: Set(checkout.checked_out_by().raw_id()),
                checked_out_by_name: Set(checkout.checked_out_by().name().to_string()),
                returned_at: Set(checkout.returned_at().map(|dt| dt.into())),
            })
            .collect::<Vec<_>>();

        book_checkouts::Entity::delete_many()
            .filter(book_checkouts::Column::BookId.eq(book.audit().raw_id()))
            .exec(&txn)
            .await
            .map_err(log_db_error)?;
        book_checkouts::Entity::insert_many(book_checkouts)
            .exec(&txn)
            .await
            .map_err(log_db_error)?;

        // Commit transaction
        txn.commit().await.map_err(log_db_error)?;

        Ok(())
    }

    async fn delete(&self, id: BookId) -> Result<(), PersistenceError> {
        let result = books::Entity::delete_by_id(id.raw())
            .exec(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        if result.rows_affected == 0 {
            Err(PersistenceError::NotFound)
        } else {
            Ok(())
        }
    }
}
