use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. 기존 user_role enum의 값을 새로운 값으로 변경
        db.execute_unprepared(
            "ALTER TYPE user_role RENAME VALUE 'member' TO 'Admin';
             ALTER TYPE user_role RENAME VALUE 'moderator' TO 'Manager';"
        ).await?;

        // 2. Admin enum 값이 이미 있을 수 있으므로 Staff만 추가
        db.execute_unprepared(
            "ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'Staff';"
        ).await?;

        // 3. users 테이블의 기본값을 Admin으로 변경
        db.execute_unprepared(
            "ALTER TABLE users ALTER COLUMN role SET DEFAULT 'Admin';"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 롤백: 기본값을 member로 변경
        db.execute_unprepared(
            "ALTER TABLE users ALTER COLUMN role SET DEFAULT 'member';"
        ).await?;

        // 롤백: enum 값을 원래대로 (완전한 롤백은 어려우므로 기본 동작만)
        db.execute_unprepared(
            "ALTER TYPE user_role RENAME VALUE 'Admin' TO 'member';
             ALTER TYPE user_role RENAME VALUE 'Manager' TO 'moderator';"
        ).await?;

        Ok(())
    }
}
