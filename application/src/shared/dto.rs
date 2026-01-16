use chrono::{DateTime, Utc};
use domain::{audit::EntityAudit, auth::permission::Permission, shared::Id};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct AuditDTO {
    pub created_by: UserReferenceDTO,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<UserReferenceDTO>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permission: PermissionDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct AuditSummaryDTO {
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permission: PermissionDTO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct UserReferenceDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct PermissionDTO {
    pub can_update: bool,
    pub can_delete: bool,
}

impl From<&dyn Permission> for PermissionDTO {
    fn from(permission: &dyn Permission) -> Self {
        PermissionDTO {
            can_update: permission.can_update(),
            can_delete: permission.can_delete(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct PaginationDTO<T> {
    pub page: u64,
    pub page_size: u64,
    pub total_count: u64,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(utoipa::ToSchema))]
pub struct EntityCreationDTO {
    pub id: Uuid,
}

impl<EId: Id> From<&EntityAudit<EId>> for EntityCreationDTO {
    fn from(audit: &EntityAudit<EId>) -> Self {
        let id = audit.id().into();
        EntityCreationDTO { id }
    }
}
