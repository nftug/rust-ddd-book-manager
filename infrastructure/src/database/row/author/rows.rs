use application::author::dto::AuthorSummaryDTO;
use domain::author::values::AuthorReference;
use sea_orm::DerivePartialModel;
use uuid::Uuid;

#[derive(DerivePartialModel)]
#[sea_orm(entity = "crate::database::entity::authors::Entity")]
pub struct AuthorSummaryRow {
    pub id: Uuid,
    pub name: String,
}

impl AuthorSummaryRow {
    pub fn to_domain(self) -> AuthorReference {
        AuthorReference::hydrate(self.id, self.name)
    }
}

#[derive(DerivePartialModel, Clone)]
#[sea_orm(entity = "crate::database::entity::authors::Entity")]
pub struct BookAuthorRow {
    pub id: Uuid,
    pub name: String,
}

impl BookAuthorRow {
    pub fn to_domain(self) -> AuthorReference {
        AuthorReference::hydrate(self.id, self.name)
    }

    pub fn to_dto(self) -> AuthorSummaryDTO {
        AuthorSummaryDTO {
            id: self.id,
            name: self.name,
        }
    }
}
