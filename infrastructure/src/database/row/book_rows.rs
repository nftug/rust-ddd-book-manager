use application::book::dto::{BookDetailsDTO, BookListItemDTO};
use domain::{
    auth::permission::Permission,
    book::{entity::Book, values::BookId},
};
use sea_orm::{DerivePartialModel, FromQueryResult, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

use crate::{
    database::row::user_rows::UserReferenceRow,
    macros::{hydrate_audit, hydrate_audit_dto, hydrate_audit_summary_dto},
};

#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookDetailsRow {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub updated_by_id: Option<Uuid>,
    pub updated_by_name: Option<String>,
    #[sea_orm(nested)]
    pub user: UserReferenceRow,
}

impl BookDetailsRow {
    pub fn to_dto(&self, permission: &dyn Permission) -> BookDetailsDTO {
        let audit = hydrate_audit_dto!(self, permission);

        BookDetailsDTO {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            isbn: self.isbn.clone(),
            description: self.description.clone(),
            owner: self.user.clone().into(),
            audit,
        }
    }

    pub fn to_entity(&self) -> Book {
        let audit = hydrate_audit!(self, BookId);

        Book::hydrate(
            audit,
            self.title.clone(),
            self.author.clone(),
            self.isbn.clone(),
            self.description.clone(),
            self.user.clone().into(),
        )
    }
}

#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookListItemRow {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(nested)]
    pub user: UserReferenceRow,
}

impl BookListItemRow {
    pub fn to_dto(&self, permission: &dyn Permission) -> BookListItemDTO {
        let audit = hydrate_audit_summary_dto!(self, permission);

        BookListItemDTO {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            owner: self.user.clone().into(),
            audit,
        }
    }
}
