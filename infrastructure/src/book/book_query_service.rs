use application::book::{dto::*, interface::BookQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    auth::{Actor, permission::EntityPermission},
    shared::error::PersistenceError,
};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::{
    database::{
        ConnectionPool,
        entity::{books, users},
        row::{book_rows::BookListItemRow, user_rows::UserReferenceRow},
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
        actor: Option<&Actor>,
        book_id: Uuid,
    ) -> Result<Option<BookDetailsDTO>, PersistenceError> {
        let result: Option<(books::Model, Option<UserReferenceRow>)> =
            books::Entity::find_by_id(book_id)
                .inner_join(users::Entity)
                .select_also(users::Entity)
                .columns([users::Column::Id, users::Column::Name])
                .into_model()
                .one(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some((book, Some(owner))) => {
                let permission = EntityPermission::new(actor, book.created_by_id.into());
                let audit = hydrate_audit_dto!(book, &permission);

                Ok(Some(BookDetailsDTO {
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
        actor: Option<&Actor>,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError> {
        let total_count = books::Entity::find()
            .count(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        let rows = books::Entity::find()
            .columns([
                books::Column::Id,
                books::Column::Title,
                books::Column::Author,
                books::Column::Description,
                books::Column::OwnerId,
                books::Column::CreatedAt,
                books::Column::UpdatedAt,
            ])
            .inner_join(users::Entity)
            .select_also(users::Entity)
            .columns([users::Column::Id, users::Column::Name])
            .order_by_asc(books::Column::CreatedAt)
            .into_model::<BookListItemRow, UserReferenceRow>()
            .paginate(self.db.inner_ref(), query.limit as u64)
            .fetch_page(query.page_from_zero())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        let rows: Vec<_> = rows
            .into_iter()
            .filter_map(|(book, owner)| owner.map(|o| (book, o)))
            .collect();

        Ok(BookListResponseDTO {
            limit: query.limit,
            page: query.page,
            total_count: total_count as usize,
            items: rows
                .into_iter()
                .map(|(book, owner)| {
                    let permission = EntityPermission::new(actor, book.owner_id.into());
                    book.to_dto(owner, &permission)
                })
                .collect(),
        })
    }
}
