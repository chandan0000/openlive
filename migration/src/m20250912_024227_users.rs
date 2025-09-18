use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
   
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto;")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Users::FullName).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PhoneNumber).string().not_null())
                    .col(ColumnDef::new(Users::ProfileUrl).string())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null())
                    .col(ColumnDef::new(Users::IsSuperuser).boolean().not_null())
                    .col(ColumnDef::new(Users::IsVerified).boolean().not_null())
                    .col(ColumnDef::new(Users::IsDeleted).boolean().not_null())
                    .col(ColumnDef::new(Users::IsOnline).boolean().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    FullName,
    Email,
    PhoneNumber,
    ProfileUrl,
    Password,
    IsActive,
    IsSuperuser,
    IsVerified,
    IsDeleted,
    IsOnline,
    CreatedAt,
    UpdatedAt,
}
