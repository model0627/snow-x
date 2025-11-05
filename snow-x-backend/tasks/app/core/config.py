from pydantic_settings import BaseSettings, SettingsConfigDict
from pydantic import ( EmailStr, computed_field )


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_ignore_empty=True,
        extra="ignore",
    )
    API_STR: str = "/tasks"

    # Celery 설정 (DB 0 사용)
    CELERY_BROKER_URL: str = "redis://localhost:6379/0"
    CELERY_RESULT_BACKEND: str = "redis://localhost:6379/0"

    # AWS 설정 (Cloud Custodian용)
    AWS_DEFAULT_REGION: str = "us-east-1"
    AWS_ACCESS_KEY_ID: str = ""
    AWS_SECRET_ACCESS_KEY: str = ""

    # Cloudflare R2 설정
    R2_ACCOUNT_ID: str = ""
    R2_ACCESS_KEY_ID: str = ""
    R2_SECRET_ACCESS_KEY: str = ""
    R2_BUCKET_NAME: str = ""
    R2_PUBLIC_DOMAIN: str = ""

    # PostgreSQL 설정
    POSTGRES_USER: str = "app"
    POSTGRES_PASSWORD: str = "secret"
    POSTGRES_HOST: str = "localhost"
    POSTGRES_PORT: str = "5432"
    POSTGRES_NAME: str = "snow_x"

    @computed_field
    @property
    def DATABASE_URL(self) -> str:
        return f"postgresql://{self.POSTGRES_USER}:{self.POSTGRES_PASSWORD}@{self.POSTGRES_HOST}:{self.POSTGRES_PORT}/{self.POSTGRES_NAME}"

    # SMTP
    SMTP_TLS: bool = True
    SMTP_SSL: bool = False
    SMTP_PORT: int = 587
    SMTP_HOST: str | None = None
    SMTP_USER: str | None = None
    SMTP_PASSWORD: str | None = None
    EMAILS_FROM_EMAIL: EmailStr | None = None
    EMAILS_FROM_NAME: str | None = None
    AUTH_EMAIL_VERIFICATION_TOKEN_EXPIRE_TIME: int = 1
    AUTH_PASSWORD_RESET_TOKEN_EXPIRE_TIME: int = 1
    PROJECT_NAME: str = "Snow-X"
    FRONTEND_HOST: str = "http://localhost:5173"

    @computed_field  # type: ignore[prop-decorator]
    @property
    def emails_enabled(self) -> bool:
        return bool(self.SMTP_HOST and self.EMAILS_FROM_EMAIL)

    # Meilisearch 설정
    MEILISEARCH_HOST: str = "http://localhost:7700"
    MEILISEARCH_API_KEY: str = ""
    
    # Markdown 서비스 설정
    MARKDOWN_SERVICE_HOST: str = "localhost"
    MARKDOWN_SERVICE_PORT: int = 6700

    # System user (for automated sync writes)
    SYSTEM_USER_ID: str | None = None
    SYSTEM_USER_NAME: str = "SnowX System"
    SYSTEM_USER_HANDLE: str = "snowx-system"
    SYSTEM_USER_EMAIL: EmailStr = "system@snow-x.dev"


settings = Settings()
