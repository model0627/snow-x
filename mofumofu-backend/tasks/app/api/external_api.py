"""
External API management endpoints
"""
from datetime import datetime, timezone
from typing import List, Dict, Any, Optional
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from sqlalchemy import desc

from ..core.database import get_db_session
from ..models.external_api import ExternalApiConnection, ExternalApiSyncLog, ExternalApiData
from ..services.external_api_client import ExternalApiClient
from ..tasks.external_api_sync import (
    sync_external_api_connection, 
    test_api_connection, 
    sync_all_active_connections
)
from ..utils import get_logger

logger = get_logger(__name__)
router = APIRouter(tags=["External API"])


# Pydantic models for request/response
from pydantic import BaseModel, Field, ConfigDict
from enum import Enum

class TargetType(str, Enum):
    device = "device"
    device_library = "device_library"
    contact = "contact"

class CreateConnectionRequest(BaseModel):
    name: str = Field(..., min_length=1, max_length=255)
    base_url: str = Field(..., min_length=1, max_length=500)
    description: Optional[str] = None
    target_type: TargetType = Field(default=TargetType.device)
    headers: Optional[Dict[str, str]] = None
    auth_config: Optional[Dict[str, Any]] = None
    field_mapping: Optional[Dict[str, Any]] = None
    sync_interval: int = Field(default=3600, ge=60, le=86400)  # 1 minute to 24 hours
    is_active: bool = True
    auto_sync: bool = True

class UpdateConnectionRequest(BaseModel):
    model_config = ConfigDict(use_enum_values=True)

    name: Optional[str] = Field(None, min_length=1, max_length=255)
    base_url: Optional[str] = Field(None, min_length=1, max_length=500)
    description: Optional[str] = None
    target_type: Optional[TargetType] = None
    headers: Optional[Dict[str, str]] = None
    auth_config: Optional[Dict[str, Any]] = None
    field_mapping: Optional[Dict[str, Any]] = None
    sync_interval: Optional[int] = Field(None, ge=60, le=86400)
    is_active: Optional[bool] = None
    auto_sync: Optional[bool] = None

class ConnectionResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    name: str
    base_url: str
    description: Optional[str]
    target_type: str
    headers: Optional[Dict[str, str]]
    auth_config: Optional[Dict[str, Any]]
    field_mapping: Optional[Dict[str, Any]]
    sync_interval: int
    is_active: bool
    auto_sync: bool
    last_sync_at: Optional[datetime]
    next_sync_at: Optional[int]
    sync_count: int
    last_error_message: Optional[str]
    created_at: datetime
    updated_at: datetime

class TestConnectionRequest(BaseModel):
    base_url: str
    headers: Optional[Dict[str, str]] = None
    auth_config: Optional[Dict[str, Any]] = None


@router.get("/connections", response_model=List[ConnectionResponse])
async def list_connections(
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db_session)
):
    """List all external API connections"""
    connections = db.query(ExternalApiConnection).offset(skip).limit(limit).all()
    return connections


@router.get("/connections/{connection_id}", response_model=ConnectionResponse)
async def get_connection(
    connection_id: int,
    db: Session = Depends(get_db_session)
):
    """Get a specific external API connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )

    logger.info(f"GET connection {connection_id}: target_type = {connection.target_type}")
    logger.info(f"GET connection {connection_id}: hasattr target_type = {hasattr(connection, 'target_type')}")
    return connection


@router.post("/connections", response_model=ConnectionResponse)
async def create_connection(
    request: CreateConnectionRequest,
    db: Session = Depends(get_db_session)
):
    """Create a new external API connection"""
    # Calculate next sync time
    next_sync_at = None
    if request.auto_sync:
        next_sync_at = int(datetime.now(timezone.utc).timestamp()) + request.sync_interval
    
    connection = ExternalApiConnection(
        name=request.name,
        base_url=request.base_url,
        description=request.description,
        target_type=request.target_type.value,
        headers=request.headers,
        auth_config=request.auth_config,
        field_mapping=request.field_mapping,
        sync_interval=request.sync_interval,
        is_active=request.is_active,
        auto_sync=request.auto_sync,
        next_sync_at=next_sync_at
    )
    
    db.add(connection)
    db.commit()
    db.refresh(connection)
    
    logger.info(f"Created new API connection: {connection.name} (ID: {connection.id})")
    return connection


@router.put("/connections/{connection_id}", response_model=ConnectionResponse)
async def update_connection(
    connection_id: int,
    request: UpdateConnectionRequest,
    db: Session = Depends(get_db_session)
):
    """Update an existing external API connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )
    
    # Update fields
    update_data = request.model_dump(exclude_unset=True)
    logger.info(f"Update data received: {update_data}")

    for field, value in update_data.items():
        # Convert Enum to string for target_type
        if field == 'target_type':
            logger.info(f"Updating target_type from {connection.target_type} to {value}")
            if hasattr(value, 'value'):
                value = value.value
        setattr(connection, field, value)
    
    # Recalculate next sync time if relevant fields changed
    if 'auto_sync' in update_data or 'sync_interval' in update_data:
        if connection.auto_sync and connection.is_active:
            connection.next_sync_at = int(datetime.now(timezone.utc).timestamp()) + connection.sync_interval
        else:
            connection.next_sync_at = None
    
    connection.updated_at = datetime.now(timezone.utc)
    db.commit()
    db.refresh(connection)

    logger.info(f"Updated API connection: {connection.name} (ID: {connection.id})")
    logger.info(f"Connection target_type after update: {connection.target_type}")
    logger.info(f"Connection attributes: {dir(connection)}")
    return connection


@router.delete("/connections/{connection_id}")
async def delete_connection(
    connection_id: int,
    db: Session = Depends(get_db_session)
):
    """Delete an external API connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )
    
    db.delete(connection)
    db.commit()
    
    logger.info(f"Deleted API connection: {connection.name} (ID: {connection_id})")
    return {"message": "Connection deleted successfully"}


@router.post("/test-connection")
async def test_connection_endpoint(request: TestConnectionRequest):
    """Test an API connection without saving it"""
    connection_data = {
        "base_url": request.base_url,
        "headers": request.headers or {},
        "auth_config": request.auth_config or {}
    }
    
    # Run test connection task
    task = test_api_connection.delay(connection_data)
    result = task.get(timeout=30)  # Wait up to 30 seconds
    
    return result


@router.post("/connections/{connection_id}/sync")
async def sync_connection(
    connection_id: int,
    db: Session = Depends(get_db_session)
):
    """Trigger manual synchronization for a specific connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )
    
    # Queue sync task
    task = sync_external_api_connection.delay(connection_id)
    
    return {
        "message": "Sync task queued",
        "task_id": task.id,
        "connection_id": connection_id
    }


@router.post("/sync-all")
async def sync_all_connections():
    """Trigger synchronization for all active connections"""
    task = sync_all_active_connections.delay()
    
    return {
        "message": "Sync all task queued",
        "task_id": task.id
    }


@router.get("/connections/{connection_id}/logs")
async def get_sync_logs(
    connection_id: int,
    skip: int = 0,
    limit: int = 50,
    db: Session = Depends(get_db_session)
):
    """Get sync logs for a specific connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )
    
    logs = db.query(ExternalApiSyncLog).filter(
        ExternalApiSyncLog.connection_id == connection_id
    ).order_by(desc(ExternalApiSyncLog.started_at)).offset(skip).limit(limit).all()
    
    return logs


@router.get("/connections/{connection_id}/data")
async def get_api_data(
    connection_id: int,
    skip: int = 0,
    limit: int = 100,
    data_type: Optional[str] = None,
    db: Session = Depends(get_db_session)
):
    """Get data from a specific connection"""
    connection = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.id == connection_id
    ).first()
    
    if not connection:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Connection not found"
        )
    
    query = db.query(ExternalApiData).filter(
        ExternalApiData.connection_id == connection_id,
        ExternalApiData.status == "active"
    )
    
    if data_type:
        query = query.filter(ExternalApiData.data_type == data_type)
    
    data = query.order_by(desc(ExternalApiData.last_sync_at)).offset(skip).limit(limit).all()
    
    return data


@router.get("/stats")
async def get_api_stats(db: Session = Depends(get_db_session)):
    """Get overall API synchronization statistics"""
    total_connections = db.query(ExternalApiConnection).count()
    active_connections = db.query(ExternalApiConnection).filter(
        ExternalApiConnection.is_active == True
    ).count()
    
    recent_syncs = db.query(ExternalApiSyncLog).filter(
        ExternalApiSyncLog.started_at >= datetime.now(timezone.utc).replace(hour=0, minute=0, second=0, microsecond=0)
    ).count()
    
    failed_syncs = db.query(ExternalApiSyncLog).filter(
        ExternalApiSyncLog.status == "error",
        ExternalApiSyncLog.started_at >= datetime.now(timezone.utc).replace(hour=0, minute=0, second=0, microsecond=0)
    ).count()
    
    total_records = db.query(ExternalApiData).filter(
        ExternalApiData.status == "active"
    ).count()
    
    return {
        "total_connections": total_connections,
        "active_connections": active_connections,
        "today_syncs": recent_syncs,
        "today_failed_syncs": failed_syncs,
        "total_active_records": total_records
    }