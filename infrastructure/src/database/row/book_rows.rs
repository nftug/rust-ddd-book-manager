use application::book::dto::BookListItemDTO;
use chrono::{DateTime, Utc};
use domain::auth::permission::Permission;
use sea_orm::FromQueryResult;
use uuid::Uuid;

use crate::{database::row::user_rows::UserReferenceRow, macros::hydrate_audit_summary_dto};

#[derive(Debug, FromQueryResult)]
pub struct BookListItemRow {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub description: String,
    pub owner_id: Uuid,
    pub owner_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl BookListItemRow {
    pub fn to_dto(&self, owner: UserReferenceRow, permission: &dyn Permission) -> BookListItemDTO {
        let audit = hydrate_audit_summary_dto!(self, permission);

        BookListItemDTO {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            owner: owner.into(),
            audit,
        }
    }
}
