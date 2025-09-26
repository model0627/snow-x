"""
External API related database models
"""
from datetime import datetime, timezone
from typing import Dict, Any, Optional
from sqlalchemy import Column, Integer, String, Text, DateTime, JSON, Boolean, ForeignKey
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship

Base = declarative_base()


class ExternalApiConnection(Base):
    """External API connection configuration"""
    __tablename__ = "external_api_connections"
    
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String(255), nullable=False)
    base_url = Column(String(500), nullable=False)
    description = Column(Text, nullable=True)
    
    # Authentication and headers
    headers = Column(JSON, nullable=True)
    auth_config = Column(JSON, nullable=True)  # {'type': 'bearer', 'token': '...'}
    
    # Sync configuration
    sync_interval = Column(Integer, default=3600)  # seconds
    is_active = Column(Boolean, default=True)
    auto_sync = Column(Boolean, default=True)
    
    # Status tracking
    last_sync_at = Column(DateTime(timezone=True), nullable=True)
    next_sync_at = Column(Integer, nullable=True)  # timestamp
    sync_count = Column(Integer, default=0)
    last_error_message = Column(Text, nullable=True)
    field_mapping = Column(JSON, nullable=True)  # Field mapping configuration
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc))
    updated_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    
    # Relationships
    sync_logs = relationship("ExternalApiSyncLog", back_populates="connection", cascade="all, delete-orphan")
    api_data = relationship("ExternalApiData", back_populates="connection", cascade="all, delete-orphan")


class ExternalApiSyncLog(Base):
    """Log entries for API synchronization attempts"""
    __tablename__ = "external_api_sync_logs"
    
    id = Column(Integer, primary_key=True, index=True)
    connection_id = Column(Integer, ForeignKey("external_api_connections.id"), nullable=False)
    
    # Status and timing
    status = Column(String(50), nullable=False)  # 'success', 'error', 'in_progress'
    started_at = Column(DateTime(timezone=True), nullable=False)
    completed_at = Column(DateTime(timezone=True), nullable=True)
    duration = Column(Integer, nullable=True)  # milliseconds
    
    # Request details
    request_url = Column(String(1000), nullable=False)
    request_method = Column(String(10), nullable=False)
    request_headers = Column(JSON, nullable=True)
    request_body = Column(Text, nullable=True)
    
    # Response details
    response_status = Column(Integer, nullable=True)
    response_headers = Column(JSON, nullable=True)
    response_body = Column(Text, nullable=True)
    
    # Processing results
    records_processed = Column(Integer, default=0)
    error_message = Column(Text, nullable=True)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc))
    updated_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    
    # Relationships
    connection = relationship("ExternalApiConnection", back_populates="sync_logs")


class ExternalApiData(Base):
    """Stored data from external APIs"""
    __tablename__ = "external_api_data"
    
    id = Column(Integer, primary_key=True, index=True)
    connection_id = Column(Integer, ForeignKey("external_api_connections.id"), nullable=False)
    
    # Data identification
    external_id = Column(String(255), nullable=True)  # ID from external system
    data_type = Column(String(100), nullable=False)  # 'device', 'user', 'log', etc.
    hash = Column(String(64), nullable=False)  # For change detection
    
    # Data content
    raw_data = Column(JSON, nullable=False)  # Original API response
    processed_data = Column(JSON, nullable=True)  # Processed/transformed data
    
    # Status and sync info
    status = Column(String(50), default="active")  # 'active', 'archived', 'deleted'
    last_sync_at = Column(DateTime(timezone=True), nullable=False)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc))
    updated_at = Column(DateTime(timezone=True), default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    
    # Relationships
    connection = relationship("ExternalApiConnection", back_populates="api_data")