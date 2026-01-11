macro_rules! with_audit_columns {
    ($table:ident, $stmt:expr) => {
        $stmt
            .col(
                ColumnDef::new($table::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .extra("DEFAULT gen_random_uuid()"),
            )
            .col(
                ColumnDef::new($table::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new($table::CreatedById).uuid().not_null())
            .col(
                ColumnDef::new($table::CreatedByName)
                    .string_len(100)
                    .not_null(),
            )
            .col(
                ColumnDef::new($table::UpdatedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .col(ColumnDef::new($table::UpdatedById).uuid().null())
            .col(ColumnDef::new($table::UpdatedByName).string_len(100).null())
    };
}

pub(crate) use with_audit_columns;
