use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::shared::{AuditDTO, AuditSummaryDTO, PaginationDTO, UserReferenceDTO};

#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookDetailsDTO {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub owner: UserReferenceDTO,
    pub checkout: Option<BookCheckoutDTO>,
    pub audit: AuditDTO,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutDTO {
    pub checkout_id: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub checked_out_to: UserReferenceDTO,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookListItemDTO {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub owner: UserReferenceDTO,
    pub checked_out: bool,
    pub audit: AuditSummaryDTO,
}

pub type BookListResponseDTO = PaginationDTO<BookListItemDTO>;

#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutWithReturnDTO {
    pub checkout_id: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub checked_out_to: UserReferenceDTO,
    pub returned_at: Option<DateTime<Utc>>,
}

pub type CheckoutHistoryListDTO = PaginationDTO<BookCheckoutWithReturnDTO>;
