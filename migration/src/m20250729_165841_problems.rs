use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the Problems table
        manager
            .create_table(
                Table::create()
                    .table(Problems::Table)
                    .if_not_exists()
                    .col(pk_auto(Problems::Id).big_integer())
                    .col(string(Problems::Title).string_len(128))
                    .col(string(Problems::Slug).string_len(128).unique_key())
                    .col(text_null(Problems::Statement))
                    .col(text_null(Problems::InputSpec))
                    .col(text_null(Problems::OutputSpec))
                    .col(json_null(Problems::SampleInputs))
                    .col(small_integer(Problems::TimeLimit).default(1000))
                    .col(small_integer(Problems::MemoryLimit).default(256))
                    .col(string_null(Problems::Difficulty).string_len(8))
                    .col(boolean(Problems::IsPublic).default(false))
                    .col(timestamp(Problems::CreatedAt).default(Expr::current_timestamp()))
                    .col(big_integer_null(Problems::AuthorId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Problems::Table, Problems::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Problems::Table)
                    .col(Problems::AuthorId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Problems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
    Title,
    Slug,
    Statement,
    InputSpec,
    OutputSpec,
    SampleInputs,
    TimeLimit,
    MemoryLimit,
    Difficulty,
    IsPublic,
    CreatedAt,
    AuthorId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
