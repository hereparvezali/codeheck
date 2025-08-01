use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Submissions::Id).big_integer())
                    .col(big_integer(Submissions::UserId))
                    .col(big_integer(Submissions::ProblemId))
                    .col(string(Submissions::Language).string_len(16))
                    .col(text(Submissions::Code).not_null())
                    .col(
                        string(Submissions::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(text_null(Submissions::Verdict))
                    .col(small_integer_null(Submissions::Time))
                    .col(small_integer_null(Submissions::Memory))
                    .col(timestamp(Submissions::SubmittedAt).default(Expr::current_timestamp()))
                    .col(big_integer_null(Submissions::ContestId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submissions::Table, Submissions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submissions::Table, Submissions::ProblemId)
                            .to(Problems::Table, Problems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submissions::Table, Submissions::ContestId)
                            .to(Contests::Table, Contests::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Submissions::Table)
                    .col(Submissions::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Submissions::Table)
                    .col(Submissions::ProblemId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Submissions::Table)
                    .col(Submissions::ContestId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
    UserId,
    ProblemId,
    Language,
    Code,
    Status,
    Verdict,
    Time,
    Memory,
    SubmittedAt,
    ContestId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Contests {
    Table,
    Id,
}
