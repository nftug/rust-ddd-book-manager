macro_rules! hydrate_audit {
    ($model:expr, $id_type:ty) => {
        domain::audit::EntityAudit::<$id_type>::new(
            $model.id.into(),
            $model.created_at.into(),
            domain::user::UserReference::new(
                $model.created_by_id.into(),
                $model.created_by_name.clone(),
            ),
            $model.updated_at.map(|dt| dt.into()),
            $model.updated_by_id.map(|id| {
                domain::user::UserReference::new(
                    id.into(),
                    $model.updated_by_name.clone().unwrap_or_default(),
                )
            }),
            false,
        )
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
