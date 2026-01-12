use chrono::{DateTime, Utc};
use domain::auth::permission::Permission;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditDTO {
    pub created_by: UserReferenceDTO,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<UserReferenceDTO>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permission: PermissionResponseDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditSummaryDTO {
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permission: PermissionResponseDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReferenceDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponseDTO {
    pub can_update: bool,
    pub can_delete: bool,
}

impl From<&dyn Permission> for PermissionResponseDTO {
    fn from(permission: &dyn Permission) -> Self {
        PermissionResponseDTO {
            can_update: permission.can_update(),
            can_delete: permission.can_delete(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResponseDTO<T> {
    pub limit: usize,
    pub page: usize,
    pub total_count: usize,
    pub items: Vec<T>,
}
