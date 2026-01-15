use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    author::dto::AuthorSummaryDTO,
    shared::{AuditDTO, AuditSummaryDTO, PaginationDTO, UserReferenceDTO},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookDetailsDTO {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<AuthorSummaryDTO>,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub owner: UserReferenceDTO,
    pub checkout: Option<CheckoutDTO>,
    pub audit: AuditDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookListItemDTO {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<AuthorSummaryDTO>,
    pub owner: UserReferenceDTO,
    pub checked_out: bool,
    pub audit: AuditSummaryDTO,
}

pub type BookListResponseDTO = PaginationDTO<BookListItemDTO>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutDTO {
    pub checkout_id: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub checked_out_to: UserReferenceDTO,
}

pub type CheckoutHistoryDTO = PaginationDTO<CheckoutDTO>;
