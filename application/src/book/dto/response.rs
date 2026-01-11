use serde::Serialize;
use uuid::Uuid;

use crate::shared::{AuditResponseDTO, PaginationResponseDTO, UserReferenceDTO};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponseDTO {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub owner: UserReferenceDTO,
    pub audit: AuditResponseDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookListItemResponseDTO {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub owner: UserReferenceDTO,
    pub audit: AuditResponseDTO,
}

pub type BookListResponseDTO = PaginationResponseDTO<BookListItemResponseDTO>;
