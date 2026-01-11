use sea_orm::FromQueryResult;
use uuid::Uuid;

#[derive(Debug, FromQueryResult)]
pub struct UserReferenceRow {
    pub id: Uuid,
    pub name: String,
}
