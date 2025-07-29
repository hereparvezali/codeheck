use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create table
        manager
            .create_table(
                Table::create()
                    .table(ContestRegistrations::Table)
                    .if_not_exists()
                    .col(pk_auto(ContestRegistrations::Id))
                    .col(integer(ContestRegistrations::UserId))
                    .col(integer(ContestRegistrations::ContestId))
                    .col(
                        timestamp(ContestRegistrations::RegisteredAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContestRegistrations::Table, ContestRegistrations::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContestRegistrations::Table, ContestRegistrations::ContestId)
                            .to(Contests::Table, Contests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(ContestRegistrations::Table)
                    .col(ContestRegistrations::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(ContestRegistrations::Table)
                    .col(ContestRegistrations::ContestId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContestRegistrations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContestRegistrations {
    Table,
    Id,
    UserId,
    ContestId,
    RegisteredAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Contests {
    Table,
    Id,
}
