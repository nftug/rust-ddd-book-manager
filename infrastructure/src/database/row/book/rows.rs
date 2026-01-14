use sea_orm::{DerivePartialModel, FromQueryResult, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

use crate::database::row::{author::BookAuthorRow, user::UserReferenceRow};

#[derive(DerivePartialModel, FromQueryResult, Clone)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookDetailsRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(nested)]
    pub author: BookAuthorRow,
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

#[derive(DerivePartialModel, FromQueryResult, Clone)]
#[sea_orm(entity = "crate::database::entity::books::Entity")]
pub struct BookListItemRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(nested)]
    pub author: BookAuthorRow,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(nested)]
    pub user: UserReferenceRow,
}
