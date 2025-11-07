from sqlalchemy import Column, String, DateTime, Integer, Text, JSON
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.sql import func

from .base import Base


class NotificationOutbox(Base):
    """알림 발송 아웃박스 테이블 매핑"""

    __tablename__ = "notifications_outbox"

    id = Column(
        UUID(as_uuid=True), primary_key=True, server_default=func.gen_random_uuid()
    )
    tenant_id = Column(UUID(as_uuid=True), nullable=True)
    channel = Column(String, nullable=False)
    category = Column(String, nullable=True)
    title = Column(String, nullable=True)
    message = Column(Text, nullable=True)
    payload = Column(JSON, nullable=True)
    status = Column(String, nullable=False, default="pending")
    retry_count = Column(Integer, nullable=False, default=0)
    max_retries = Column(Integer, nullable=False, default=3)
    last_error = Column(Text, nullable=True)
    scheduled_at = Column(DateTime(timezone=True), nullable=False)
    processing_started_at = Column(DateTime(timezone=True), nullable=True)
    processed_at = Column(DateTime(timezone=True), nullable=True)
    created_at = Column(
        DateTime(timezone=True), nullable=False, server_default=func.now()
    )
    updated_at = Column(
        DateTime(timezone=True), nullable=False, server_default=func.now()
    )
