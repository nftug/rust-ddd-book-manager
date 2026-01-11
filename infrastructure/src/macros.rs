macro_rules! hydrate_audit {
    ($model:expr, $id_type:ty) => {
        domain::audit::EntityAudit::<$id_type>::new(
            $model.id.into(),
            $model.created_at.into(),
            domain::user::values::UserReference::new(
                $model.created_by_id.into(),
                domain::user::values::UserName::new($model.created_by_name.clone()),
            ),
            $model.updated_at.map(|dt| dt.into()),
            $model.updated_by_id.map(|id| {
                domain::user::values::UserReference::new(
                    id.into(),
                    domain::user::values::UserName::new(
                        $model.updated_by_name.clone().unwrap_or_default(),
                    ),
                )
            }),
            false,
        )
    };
}

macro_rules! hydrate_audit_dto {
    ($model:expr, $permission:expr) => {
        application::shared::AuditResponseDTO {
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

macro_rules! audit_defaults {
    ($active_model:path, $audit:expr) => {
        ({
            $active_model {
                id: Set($audit.id().raw()),
                created_at: Set($audit.created_at().into()),
                created_by_id: Set($audit.created_by().id().raw()),
                created_by_name: Set($audit.created_by().name().into()),
                updated_at: Set($audit.updated_at().map(|v| v.into())),
                updated_by_id: Set($audit.updated_by().map(|u| u.id().raw())),
                updated_by_name: Set($audit.updated_by().map(|u| u.name().into())),
                ..Default::default()
            }
        })
    };
}

macro_rules! audit_defaults_update {
    ($active_model:expr, $audit:expr) => {
        $active_model.updated_at = Set($audit.updated_at().map(|v| v.into()));
        $active_model.updated_by_id = Set($audit.updated_by().map(|u| u.id().raw()));
        $active_model.updated_by_name = Set($audit.updated_by().map(|u| u.name().into()));
    };
}

pub(crate) use audit_defaults;
pub(crate) use audit_defaults_update;
pub(crate) use hydrate_audit;
pub(crate) use hydrate_audit_dto;
