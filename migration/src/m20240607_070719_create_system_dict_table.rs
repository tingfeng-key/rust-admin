use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemDict::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemDict::Id))
                    .col(string(SystemDict::Name))
                    .col(string(SystemDict::Sign))
                    .col(string(SystemDict::Remark).default(""))
                    .col(integer(SystemDict::Status).default(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemDict::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemDict {
    Table,
    Id,
    Name,
    Sign,
    Remark,
    Status,
}
