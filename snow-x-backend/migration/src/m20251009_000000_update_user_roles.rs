use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. 기존 user_role enum 값이 존재할 때만 이름을 변경 (신규 설치 호환성)
        db.execute_unprepared(
            r#"
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_enum e
        JOIN pg_type t ON e.enumtypid = t.oid
        WHERE t.typname = 'user_role' AND e.enumlabel = 'member'
    ) THEN
        EXECUTE 'ALTER TYPE user_role RENAME VALUE ''member'' TO ''Admin''';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM pg_enum e
        JOIN pg_type t ON e.enumtypid = t.oid
        WHERE t.typname = 'user_role' AND e.enumlabel = 'moderator'
    ) THEN
        EXECUTE 'ALTER TYPE user_role RENAME VALUE ''moderator'' TO ''Manager''';
    END IF;
END
$$;
"#
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

        // 롤백: enum 값을 원래 이름으로 되돌릴 때만 실행
        db.execute_unprepared(
            r#"
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_enum e
        JOIN pg_type t ON e.enumtypid = t.oid
        WHERE t.typname = 'user_role' AND e.enumlabel = 'Admin'
    ) THEN
        EXECUTE 'ALTER TYPE user_role RENAME VALUE ''Admin'' TO ''member''';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM pg_enum e
        JOIN pg_type t ON e.enumtypid = t.oid
        WHERE t.typname = 'user_role' AND e.enumlabel = 'Manager'
    ) THEN
        EXECUTE 'ALTER TYPE user_role RENAME VALUE ''Manager'' TO ''moderator''';
    END IF;
END
$$;
"#
        ).await?;

        // 롤백: 기본값을 member로 변경 (위에서 Admin -> member로 변경된 이후라면 정상 동작)
        db.execute_unprepared(
            "ALTER TABLE users ALTER COLUMN role SET DEFAULT 'member';"
        ).await?;

        Ok(())
    }
}
