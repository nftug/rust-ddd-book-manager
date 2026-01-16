use application::book::{dto::*, interface::BookQueryService};
use async_trait::async_trait;
use derive_new::new;
use domain::{audit::Actor, auth::permission::EntityPermission, shared::error::PersistenceError};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};
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
        let mut id_db_query = books::Entity::find()
            .select_only()
            .column(books::Column::Id);

        // Apply filters
        if let Some(owner_id) = query.owner_id {
            id_db_query = id_db_query.filter(books::Column::OwnerId.eq(owner_id));
        }
        if let Some(checked_out) = query.checked_out {
            let active_checkouts = book_checkouts::Entity::find()
                .select_only()
                .column(book_checkouts::Column::BookId)
                .filter(book_checkouts::Column::ReturnedAt.is_null());

            if checked_out {
                id_db_query = id_db_query
                    .filter(books::Column::Id.in_subquery(active_checkouts.into_query()));
            } else {
                id_db_query = id_db_query
                    .filter(books::Column::Id.not_in_subquery(active_checkouts.into_query()));
            }
        }
        if let Some(checked_out_to_id) = query.checked_out_to_id {
            let checkouts_for_user = book_checkouts::Entity::find()
                .select_only()
                .column(book_checkouts::Column::BookId)
                .filter(book_checkouts::Column::CheckedOutById.eq(checked_out_to_id))
                .filter(book_checkouts::Column::ReturnedAt.is_null());

            id_db_query =
                id_db_query.filter(books::Column::Id.in_subquery(checkouts_for_user.into_query()));
        }

        // Get total count before pagination
        let total_count = id_db_query
            .clone()
            .count(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        // Apply pagination
        let book_ids: Vec<Uuid> = id_db_query
            .order_by_desc(books::Column::CreatedAt)
            .order_by_desc(books::Column::Id) // Ensure consistent order when paginating
            .into_tuple()
            .paginate(self.db.inner_ref(), query.limit)
            .fetch_page(query.page - 1)
            .await
            .map_err(log_db_error)?;

        let rows = books::Entity::find()
            .inner_join(authors::Entity)
            .inner_join(users::Entity)
            .left_join(book_checkouts::Entity)
            .filter(books::Column::Id.is_in(book_ids))
            .order_by_desc(books::Column::CreatedAt)
            .order_by_desc(books::Column::Id) // Ensure consistent order when paginating
            .order_by_asc(book_authors::Column::OrderIndex)
            .into_partial_model::<BookListItemRow>()
            .all(self.db.inner_ref())
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

    async fn get_checkout_history(
        &self,
        book_id: Uuid,
        query: &CheckoutHistoryQueryDTO,
    ) -> Result<CheckoutHistoryDTO, PersistenceError> {
        let db_query =
            book_checkouts::Entity::find().filter(book_checkouts::Column::BookId.eq(book_id));

        let total_count = db_query
            .clone()
            .select_only()
            .count(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        let rows = db_query
            .order_by_desc(book_checkouts::Column::CheckedOutAt)
            .into_partial_model::<BookCheckoutRow>()
            .paginate(self.db.inner_ref(), query.limit)
            .fetch_page(query.page - 1)
            .await
            .map_err(log_db_error)?;

        Ok(CheckoutHistoryDTO {
            limit: query.limit,
            page: query.page,
            total_count,
            items: rows.into_iter().map(|row| row.to_dto()).collect(),
        })
    }
}
