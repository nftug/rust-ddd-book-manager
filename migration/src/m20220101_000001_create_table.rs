use sea_orm_migration::prelude::*;

use crate::macros::with_audit_columns;

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
                    Authors,
                    Table::create().table(Authors::Table).if_not_exists().col(
                        ColumnDef::new(Authors::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key()
                    )
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
                        .col(ColumnDef::new(Books::Isbn).string_len(13).null())
                        .col(ColumnDef::new(Books::Description).string_len(1000).null())
                        .col(ColumnDef::new(Books::OwnerId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_owner_user_id")
                                .from(Books::Table, Books::OwnerId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                )
                .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BookAuthors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BookAuthors::BookId).uuid().not_null())
                    .col(ColumnDef::new(BookAuthors::AuthorId).uuid().not_null())
                    .col(
                        ColumnDef::new(BookAuthors::OrderIndex)
                            .unsigned()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_book_authors")
                            .col(BookAuthors::BookId)
                            .col(BookAuthors::AuthorId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_authors_book_id")
                            .from(BookAuthors::Table, BookAuthors::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_authors_author_id")
                            .from(BookAuthors::Table, BookAuthors::AuthorId)
                            .to(Authors::Table, Authors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BookCheckouts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BookCheckouts::CheckoutId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookCheckouts::BookId).uuid().not_null())
                    .col(
                        ColumnDef::new(BookCheckouts::CheckedOutAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BookCheckouts::CheckedOutById)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BookCheckouts::CheckedOutByName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BookCheckouts::ReturnedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_checkouts_book_id")
                            .from(BookCheckouts::Table, BookCheckouts::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookAuthors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BookCheckouts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Authors::Table).to_owned())
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

#[derive(DeriveIden)]
enum Authors {
    Table,
    Id,
    Name,
    CreatedAt,
    CreatedById,
    CreatedByName,
    UpdatedAt,
    UpdatedById,
    UpdatedByName,
}

#[derive(DeriveIden)]
enum BookAuthors {
    Table,
    BookId,
    AuthorId,
    OrderIndex,
}

#[derive(DeriveIden)]
enum BookCheckouts {
    Table,
    CheckoutId,
    BookId,
    CheckedOutAt,
    CheckedOutById,
    CheckedOutByName,
    ReturnedAt,
}
