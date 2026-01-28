macro_rules! hydrate_audit {
    ($model:expr, $id_type:ty) => {
        domain::audit::EntityAudit::<$id_type>::hydrate(
            $model.id,
            $model.created_at.into(),
            $model.created_by_id,
            $model.created_by_name.clone(),
            $model.updated_at.map(|dt| dt.into()),
            $model.updated_by_id,
            $model.updated_by_name.clone(),
        )
    };
}

macro_rules! hydrate_audit_dto {
    ($model:expr, $permission:expr) => {
        application::shared::AuditDTO {
            created_at: $model.created_at.into(),
            created_by: application::shared::UserReferenceDTO {
                id: $model.created_by_id.into(),
                name: $model.created_by_name.clone(),
            },
            updated_at: $model.updated_at.map(|dt| dt.into()),
            updated_by: $model
                .updated_by_id
                .map(|id| application::shared::UserReferenceDTO {
                    id,
                    name: $model.updated_by_name.clone().unwrap_or_default(),
                }),
            permission: ($permission as &dyn domain::auth::permission::Permission).into(),
        }
    };
}

macro_rules! hydrate_audit_summary_dto {
    ($model:expr, $permission:expr) => {
        application::shared::AuditSummaryDTO {
            created_at: $model.created_at.into(),
            updated_at: $model.updated_at.map(|dt| dt.into()),
            permission: ($permission as &dyn domain::auth::permission::Permission).into(),
        }
    };
}

macro_rules! audit_defaults {
    ($active_model:path, $audit:expr) => {
        ({
            $active_model {
                id: Set($audit.raw_id()),
                created_at: Set($audit.created_at().into()),
                created_by_id: Set($audit.created_by().raw_id()),
                created_by_name: Set($audit.created_by().name().into()),
                updated_at: Set($audit.updated_at().map(|v| v.into())),
                updated_by_id: Set($audit.updated_by().map(|u| u.raw_id())),
                updated_by_name: Set($audit.updated_by().map(|u| u.name().into())),
                ..Default::default()
            }
        })
    };
}

macro_rules! update_on_conflict {
    ($column:ty, [$($update_col:path),+ $(,)?]) => {
        sea_orm::sea_query::OnConflict::column(<$column>::Id)
            .update_columns([
                $($update_col),+,
                <$column>::UpdatedAt,
                <$column>::UpdatedById,
                <$column>::UpdatedByName,
            ])
            .to_owned()
    };
}

pub(crate) use audit_defaults;
pub(crate) use hydrate_audit;
pub(crate) use hydrate_audit_dto;
pub(crate) use hydrate_audit_summary_dto;
pub(crate) use update_on_conflict;
