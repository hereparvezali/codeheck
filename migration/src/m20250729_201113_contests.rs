use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contests::Table)
                    .if_not_exists()
                    .col(pk_auto(Contests::Id))
                    .col(string(Contests::Title).string_len(128))
                    .col(string(Contests::Slug).string_len(128).unique_key())
                    .col(text_null(Contests::Description))
                    .col(date_time(Contests::StartTime).not_null())
                    .col(date_time(Contests::EndTime).not_null())
                    .col(boolean(Contests::IsPublic).default(true))
                    .col(integer_null(Contests::AuthorId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contests::Table, Contests::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Contests::Table)
                    .col(Contests::AuthorId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contests {
    Table,
    Id,
    Title,
    Slug,
    Description,
    StartTime,
    EndTime,
    IsPublic,
    AuthorId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
