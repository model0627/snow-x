"""
IPAM (IP Address Management) related models
"""
from sqlalchemy import Column, String, Integer, Boolean, Date, Text, TIMESTAMP, UUID, JSON
from sqlalchemy.dialects.postgresql import JSONB
from datetime import datetime, timezone
from .base import Base
import uuid


class Contact(Base):
    """Contact model for managing contact information"""
    __tablename__ = "contacts"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String, nullable=False)
    title = Column(String)
    department = Column(String)
    phone = Column(String)
    mobile = Column(String)
    email = Column(String)
    office_location = Column(String)
    responsibilities = Column(Text)
    created_by = Column(UUID(as_uuid=True))
    created_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc))
    updated_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    is_active = Column(Boolean, default=True)
    source_type = Column(String, nullable=False, default='manual')  # 'manual' or 'api_sync'
    external_api_connection_id = Column(Integer, nullable=True)  # FK to external_api_connections


class Device(Base):
    """Device model for managing hardware devices"""
    __tablename__ = "devices"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    rack_id = Column(UUID(as_uuid=True))
    name = Column(String, nullable=False)
    description = Column(Text)
    device_type = Column(String)
    manufacturer = Column(String)
    model = Column(String)
    serial_number = Column(String)
    rack_position = Column(Integer)
    rack_size = Column(Integer)
    power_consumption = Column(Integer)
    status = Column(String)
    purchase_date = Column(Date)
    warranty_end = Column(Date)
    created_by = Column(UUID(as_uuid=True))
    created_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc))
    updated_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    is_active = Column(Boolean, default=True)
    source_type = Column(String, nullable=False, default='manual')  # 'manual' or 'api_sync'
    external_api_connection_id = Column(Integer, nullable=True)


class DeviceLibrary(Base):
    """Device Library model for managing device templates/catalogs"""
    __tablename__ = "device_library"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String, nullable=False)
    description = Column(Text)
    device_type = Column(String)
    manufacturer = Column(String)
    model = Column(String)
    default_rack_size = Column(Integer)
    default_power_consumption = Column(Integer)
    default_config = Column(JSON)
    created_by = Column(UUID(as_uuid=True))
    created_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc))
    updated_at = Column(TIMESTAMP, default=lambda: datetime.now(timezone.utc), onupdate=lambda: datetime.now(timezone.utc))
    is_active = Column(Boolean, default=True)
    device_id = Column(UUID(as_uuid=True))
    device_name = Column(String(255))
    source_type = Column(String, nullable=False, default='manual')  # 'manual' or 'api_sync'
    external_api_connection_id = Column(Integer, nullable=True)
