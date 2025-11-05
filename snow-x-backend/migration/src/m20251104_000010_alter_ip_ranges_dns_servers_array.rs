use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Convert existing dns_servers values from TEXT to JSONB array
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE ip_ranges
                ALTER COLUMN dns_servers TYPE JSONB
                USING
                    CASE
                        WHEN dns_servers IS NULL
                            OR dns_servers = ''
                            OR dns_servers = '{}' THEN NULL
                        ELSE to_jsonb(
                            string_to_array(
                                replace(replace(trim(both '{}' from dns_servers), '\"', ''), ' ', ''),
                                ','
                            )
                        )
                    END;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert dns_servers column back to TEXT
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE ip_ranges
                ALTER COLUMN dns_servers TYPE TEXT
                USING
                    CASE
                        WHEN dns_servers IS NULL THEN NULL
                        ELSE dns_servers::TEXT
                    END;
                "#,
            )
            .await?;

        Ok(())
    }
}
