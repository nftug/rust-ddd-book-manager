use application::shared::UserReferenceDTO;
use domain::user::values::{UserName, UserReference};
use sea_orm::FromQueryResult;
use uuid::Uuid;

#[derive(Debug, FromQueryResult)]
pub struct UserReferenceRow {
    pub id: Uuid,
    pub name: String,
}

impl From<UserReferenceRow> for UserReference {
    fn from(row: UserReferenceRow) -> Self {
        UserReference::new(row.id.into(), UserName::new(row.name))
    }
}

impl From<UserReferenceRow> for UserReferenceDTO {
    fn from(row: UserReferenceRow) -> Self {
        UserReferenceDTO {
            id: row.id,
            name: row.name,
        }
    }
}
