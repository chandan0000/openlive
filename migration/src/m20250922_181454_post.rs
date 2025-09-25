use crate::m20250912_024227_users::Users;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .col(ColumnDef::new(Post::Likes).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Post::Dislikes)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Post::Views).integer().not_null().default(0))
                     .col(ColumnDef::new(Post::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-user")
                            .from(Post::Table, Post::UserId) 
                            .to(Users::Table, Users::Id) 
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(ColumnDef::new(Post::DeletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Post::IsDeleted)
                            .boolean()
                             .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    Likes,
    Dislikes,
    Views,

    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    IsDeleted,
}
