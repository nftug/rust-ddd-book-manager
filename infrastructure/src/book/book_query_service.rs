use application::book::{dto::*, interface::BookQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{audit::Actor, auth::permission::EntityPermission, shared::error::PersistenceError};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::database::{
    ConnectionPool,
    entity::{books, users},
    row::book_rows::{BookDetailsRow, BookListItemRow},
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
        let result = books::Entity::find_by_id(book_id)
            .inner_join(users::Entity)
            .into_partial_model::<BookDetailsRow>()
            .one(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some(book) => {
                let permission = EntityPermission::new(actor, book.created_by_id.into());
                Ok(Some(book.to_dto(&permission)))
            }
            _ => Ok(None),
        }
    }

    async fn get_book_list(
        &self,
        actor: Option<&Actor>,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError> {
        let get_query = || {
            let mut select = books::Entity::find().inner_join(users::Entity);

            if let Some(owner_id) = query.owner_id {
                select = select.filter(users::Column::Id.eq(owner_id));
            }

            select
        };

        let total_count = get_query()
            .select_only()
            .count(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        let rows = get_query()
            .order_by_desc(books::Column::CreatedAt)
            .into_partial_model::<BookListItemRow>()
            .paginate(self.db.inner_ref(), query.limit)
            .fetch_page(query.page - 1)
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        Ok(BookListResponseDTO {
            limit: query.limit,
            page: query.page,
            total_count,
            items: rows
                .into_iter()
                .map(|book| {
                    let permission = EntityPermission::new(actor, book.user.id.into());
                    book.to_dto(&permission)
                })
                .collect(),
        })
    }
}
