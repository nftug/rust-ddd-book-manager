use serde::Serialize;
use uuid::Uuid;

use crate::shared::{AuditDTO, AuditSummaryDTO, PaginationDTO, UserReferenceDTO};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookDetailsDTO {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub owner: UserReferenceDTO,
    pub audit: AuditDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookListItemDTO {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub owner: UserReferenceDTO,
    pub audit: AuditSummaryDTO,
}

pub type BookListResponseDTO = PaginationDTO<BookListItemDTO>;
