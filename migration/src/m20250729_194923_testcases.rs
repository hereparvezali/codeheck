use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Testcases::Table)
                    .if_not_exists()
                    .col(pk_auto(Testcases::Id))
                    .col(integer(Testcases::ProblemId))
                    .col(text_null(Testcases::Input))
                    .col(text_null(Testcases::Output))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Testcases::Table, Testcases::ProblemId)
                            .to(Problems::Table, Problems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Testcases::Table)
                    .col(Testcases::ProblemId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Testcases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Testcases {
    Table,
    Id,
    ProblemId,
    Input,
    Output,
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
}
