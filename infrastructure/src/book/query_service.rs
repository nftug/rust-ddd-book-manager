use application::{
    book::{dto::*, interface::BookQueryService},
    shared::UserReferenceDTO,
};
use async_trait::async_trait;
use derive_new::new;
use domain::{audit::Actor, auth::permission::EntityPermission, shared::error::PersistenceError};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    Select,
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
        let id_db_query = books::Entity::find()
            .select_only()
            .column(books::Column::Id)
            .apply_if(query.owner_id, |q, owner_id| {
                q.filter(books::Column::OwnerId.eq(owner_id))
            })
            .apply_if(query.checked_out, |q, checked_out| match checked_out {
                true => q.filter(
                    books::Column::Id.in_subquery(active_checkout_ids_query().into_query()),
                ),
                false => q.filter(
                    books::Column::Id.not_in_subquery(active_checkout_ids_query().into_query()),
                ),
            })
            .apply_if(query.checked_out_to_id, |q, user_id| {
                q.filter(
                    books::Column::Id.in_subquery(
                        active_checkout_ids_query()
                            .filter(book_checkouts::Column::CheckedOutById.eq(user_id))
                            .into_query(),
                    ),
                )
            });

        let total_count = id_db_query
            .clone()
            .count(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        let book_ids: Vec<Uuid> = id_db_query
            .order_by_desc(books::Column::CreatedAt)
            .into_tuple()
            .paginate(self.db.inner_ref(), query.page_size)
            .fetch_page(query.page - 1)
            .await
            .map_err(log_db_error)?;

        let rows = books::Entity::find()
            .inner_join(authors::Entity)
            .inner_join(users::Entity)
            .left_join(book_checkouts::Entity)
            .filter(books::Column::Id.is_in(book_ids))
            .order_by_desc(books::Column::CreatedAt)
            .order_by_asc(book_authors::Column::OrderIndex)
            .into_partial_model::<BookListItemRow>()
            .all(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(BookListResponseDTO {
            page_size: query.page_size,
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
    ) -> Result<CheckoutHistoryListDTO, PersistenceError> {
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
            .paginate(self.db.inner_ref(), query.page_size)
            .fetch_page(query.page - 1)
            .await
            .map_err(log_db_error)?;

        Ok(CheckoutHistoryListDTO {
            page_size: query.page_size,
            page: query.page,
            total_count,
            items: rows
                .into_iter()
                .map(|row| BookCheckoutWithReturnDTO {
                    checkout_id: row.checkout_id,
                    checked_out_at: row.checked_out_at.into(),
                    checked_out_to: UserReferenceDTO {
                        id: row.checked_out_by_id,
                        name: row.checked_out_by_name,
                    },
                    returned_at: row.returned_at.map(|dt| dt.into()),
                })
                .collect(),
        })
    }
}

fn active_checkout_ids_query() -> Select<book_checkouts::Entity> {
    book_checkouts::Entity::find()
        .select_only()
        .column(book_checkouts::Column::BookId)
        .filter(book_checkouts::Column::ReturnedAt.is_null())
}
