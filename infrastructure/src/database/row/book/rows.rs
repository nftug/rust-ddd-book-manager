use application::{book::dto::BookCheckoutDTO, shared::UserReferenceDTO};
use domain::{book::values::BookCheckout, user::values::UserReference};
use sea_orm::{DerivePartialModel, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

use crate::database::row::{AuthorReferenceRow, user::UserReferenceRow};

#[derive(DerivePartialModel, Clone)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookDetailsRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(nested, alias = "authors")]
    pub author: AuthorReferenceRow,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub updated_by_id: Option<Uuid>,
    pub updated_by_name: Option<String>,
    #[sea_orm(nested, alias = "users")]
    pub user: UserReferenceRow,
    #[sea_orm(nested, alias = "book_checkouts")]
    pub checkout: Option<BookCheckoutRow>,
}

#[derive(DerivePartialModel, Clone)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookListItemRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(nested, alias = "authors")]
    pub author: AuthorReferenceRow,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(nested, alias = "users")]
    pub user: UserReferenceRow,
    #[sea_orm(nested, alias = "book_checkouts")]
    pub checkout: Option<BookCheckoutRow>,
}

#[derive(DerivePartialModel, Clone)]
#[sea_orm(entity = "crate::database::entity::book_checkouts::Entity")]
pub struct BookCheckoutRow {
    pub checkout_id: Uuid,
    pub book_id: Uuid,
    pub checked_out_at: DateTimeWithTimeZone,
    pub checked_out_by_id: Uuid,
    pub checked_out_by_name: String,
    pub returned_at: Option<DateTimeWithTimeZone>,
}

impl BookCheckoutRow {
    pub fn to_domain(self) -> BookCheckout {
        BookCheckout::hydrate(
            self.checkout_id,
            UserReference::hydrate(self.checked_out_by_id, self.checked_out_by_name),
            self.checked_out_at.into(),
            self.returned_at.map(|dt| dt.into()),
        )
    }

    pub fn to_dto(self) -> BookCheckoutDTO {
        BookCheckoutDTO {
            checkout_id: self.checkout_id,
            checked_out_at: self.checked_out_at.into(),
            checked_out_to: UserReferenceDTO {
                id: self.checked_out_by_id,
                name: self.checked_out_by_name,
            },
        }
    }
}
