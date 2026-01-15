use application::book::{dto::*, interface::BookQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{audit::Actor, auth::permission::EntityPermission, shared::error::PersistenceError};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::database::{
    ConnectionPool,
    entity::{authors, book_authors, book_checkouts, books, users},
    log_db_error,
    row::book::{aggregate::*, rows::*},
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
        let rows = books::Entity::find_by_id(book_id)
            .inner_join(authors::Entity)
            .inner_join(users::Entity)
            .left_join(book_checkouts::Entity)
            .order_by_asc(book_authors::Column::OrderIndex)
            .into_partial_model::<BookDetailsRow>()
            .all(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(AggregatedBookDetails::from_rows(rows).map(|agg| {
            let permission = EntityPermission::new(actor, agg.row.created_by_id.into());
            agg.to_dto(&permission)
        }))
    }

    async fn get_book_list(
        &self,
        actor: Option<&Actor>,
        query: &BookListQueryDTO,
    ) -> Result<BookListResponseDTO, PersistenceError> {
        let get_query = || {
            let mut select = books::Entity::find()
                .inner_join(authors::Entity)
                .inner_join(users::Entity)
                .left_join(book_checkouts::Entity);

            if let Some(owner_id) = query.owner_id {
                select = select.filter(users::Column::Id.eq(owner_id));
            }
            if let Some(checked_out) = query.checked_out {
                if checked_out {
                    select = select.filter(book_checkouts::Column::ReturnedAt.is_null());
                } else {
                    // Never checked out books or already returned books
                    select = select.filter(
                        book_checkouts::Column::CheckoutId
                            .is_null()
                            .or(book_checkouts::Column::ReturnedAt.is_not_null()),
                    );
                }
            }

            select
        };

        let total_count = get_query()
            .select_only()
            .count(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        let rows = get_query()
            .order_by_desc(books::Column::CreatedAt)
            .order_by_asc(book_authors::Column::OrderIndex)
            .into_partial_model::<BookListItemRow>()
            .paginate(self.db.inner_ref(), query.limit)
            .fetch_page(query.page - 1)
            .await
            .map_err(log_db_error)?;

        Ok(BookListResponseDTO {
            limit: query.limit,
            page: query.page,
            total_count,
            items: AggregatedBookListItem::from_rows(rows)
                .into_iter()
                .map(|book| {
                    let permission = EntityPermission::new(actor, book.row.user.id.into());
                    book.to_dto(&permission)
                })
                .collect(),
        })
    }
}
