use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the table
        manager
            .create_table(
                Table::create()
                    .table(ContestProblems::Table)
                    .if_not_exists()
                    .col(integer(ContestProblems::ContestId))
                    .col(integer(ContestProblems::ProblemId))
                    .col(string_null(ContestProblems::Label).string_len(1))
                    .primary_key(
                        Index::create()
                            .col(ContestProblems::ContestId)
                            .col(ContestProblems::ProblemId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContestProblems::Table, ContestProblems::ContestId)
                            .to(Contests::Table, Contests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContestProblems::Table, ContestProblems::ProblemId)
                            .to(Problems::Table, Problems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(ContestProblems::Table)
                    .col(ContestProblems::ContestId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(ContestProblems::Table)
                    .col(ContestProblems::ProblemId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContestProblems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContestProblems {
    Table,
    ContestId,
    ProblemId,
    Label,
}

#[derive(DeriveIden)]
enum Contests {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
}
