use application::book::{dto::*, interface::BookQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    auth::{Actor, permission::EntityPermission},
    shared::error::PersistenceError,
};
use sea_orm::{
    EntityTrait, FromQueryResult, PaginatorTrait, QueryOrder, prelude::DateTimeWithTimeZone,
};
use uuid::Uuid;

use crate::{
    database::{
        ConnectionPool,
        entity::{books, users},
        row::user_row::UserReferenceRow,
    },
    macros::hydrate_audit_dto,
};

#[derive(new)]
pub struct BookQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookQueryService for BookQueryServiceImpl {
    async fn get_book_details(
        &self,
        actor: &Actor,
        book_id: uuid::Uuid,
    ) -> Result<Option<BookResponseDTO>, PersistenceError> {
        let result: Option<(books::Model, Option<UserReferenceRow>)> =
            books::Entity::find_by_id(book_id)
                .inner_join(users::Entity)
                .select_also(users::Entity)
                .into_model()
                .one(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some((book, Some(owner))) => {
                let permission = EntityPermission::new(actor.clone(), book.created_by_id.into());
                let audit = hydrate_audit_dto!(book, &permission);
                Ok(Some(BookResponseDTO {
                    id: book.id,
                    title: book.title,
                    author: book.author,
                    isbn: book.isbn,
                    description: book.description,
                    owner: owner.into(),
                    audit,
                }))
            }
            _ => Ok(None),
        }
    }

    async fn get_book_list(
        &self,
        actor: &Actor,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError> {
        let total_count = books::Entity::find()
            .count(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        let paginator = books::Entity::find()
            .inner_join(users::Entity)
            .select_also(users::Entity)
            .order_by_asc(books::Column::CreatedAt)
            .into_model::<BookListItemRow, UserReferenceRow>()
            .paginate(self.db.inner_ref(), query.limit as u64);

        let rows = paginator
            .fetch_page(query.page_from_zero())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        Ok(BookListResponseDTO {
            limit: query.limit,
            page: query.page,
            total_count: total_count as usize,
            items: rows
                .into_iter()
                .map(|(book, owner)| {
                    let permission =
                        EntityPermission::new(actor.clone(), book.created_by_id.into());
                    let audit = hydrate_audit_dto!(book, &permission);

                    BookListItemResponseDTO {
                        id: book.id,
                        title: book.title,
                        author: book.author,
                        owner: owner.unwrap().into(),
                        audit,
                    }
                })
                .collect(),
        })
    }
}

#[derive(Debug, FromQueryResult)]
struct BookListItemRow {
    id: Uuid,
    title: String,
    author: String,
    created_at: DateTimeWithTimeZone,
    created_by_id: Uuid,
    created_by_name: String,
    updated_at: Option<DateTimeWithTimeZone>,
    updated_by_id: Option<Uuid>,
    updated_by_name: Option<String>,
}
