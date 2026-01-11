use crate::with_audit_columns;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                with_audit_columns!(
                    Users,
                    Table::create()
                        .table(Users::Table)
                        .if_not_exists()
                        .col(ColumnDef::new(Users::Name).string_len(100).not_null())
                        .col(
                            ColumnDef::new(Users::Email)
                                .string_len(100)
                                .not_null()
                                .unique_key(),
                        )
                        .col(ColumnDef::new(Users::Role).string_len(100).not_null())
                )
                .to_owned(),
            )
            .await?;

        manager
            .create_table(
                with_audit_columns!(
                    Books,
                    Table::create()
                        .table(Books::Table)
                        .if_not_exists()
                        .col(ColumnDef::new(Books::Title).string_len(255).not_null())
                        .col(ColumnDef::new(Books::Author).string_len(255).not_null())
                        .col(ColumnDef::new(Books::Isbn).string_len(13).not_null())
                        .col(ColumnDef::new(Books::Description).string_len(1000).null())
                        .col(ColumnDef::new(Books::OwnerId).uuid().null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_owner_user_id")
                                .from(Books::Table, Books::OwnerId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::SetNull),
                        )
                )
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    Title,
    Author,
    Isbn,
    Description,
    OwnerId,
    CreatedAt,
    CreatedById,
    CreatedByName,
    UpdatedAt,
    UpdatedById,
    UpdatedByName,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    Role,
    CreatedAt,
    CreatedById,
    CreatedByName,
    UpdatedAt,
    UpdatedById,
    UpdatedByName,
}
