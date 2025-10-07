"""
External API synchronization tasks
"""
import asyncio
import hashlib
import json
from datetime import datetime, timezone
from typing import Dict, List, Optional, Any
import httpx
from celery import Celery
from sqlalchemy.orm import Session

from ..core.celery_app import celery_app
from ..core.database import get_db_session
from ..models.external_api import ExternalApiConnection, ExternalApiSyncLog, ExternalApiData
from ..models.ipam import Contact, Device, DeviceLibrary
from ..services.external_api_client import ExternalApiClient
from ..utils import get_logger
import uuid

logger = get_logger(__name__)

# 시스템 자동 동기화를 위한 UUID (00000000-0000-0000-0000-000000000000)
SYSTEM_USER_ID = uuid.UUID('00000000-0000-0000-0000-000000000000')


@celery_app.task(name="external_api.sync_connection")
def sync_external_api_connection(connection_id: int) -> Dict[str, Any]:
    """
    특정 API 연결에 대한 동기화 작업을 수행합니다.
    """
    with get_db_session() as db:
        connection = db.query(ExternalApiConnection).filter(
            ExternalApiConnection.id == connection_id,
            ExternalApiConnection.is_active == True
        ).first()
        
        if not connection:
            logger.warning(f"Connection {connection_id} not found or inactive")
            return {"status": "error", "message": "Connection not found or inactive"}
        
        # 동기화 로그 시작
        sync_log = ExternalApiSyncLog(
            connection_id=connection_id,
            status="in_progress",
            request_url=connection.base_url,
            request_method="GET",
            started_at=datetime.now(timezone.utc)
        )
        db.add(sync_log)
        db.commit()
        
        try:
            # API 클라이언트 생성
            client = ExternalApiClient(
                base_url=connection.base_url,
                headers=connection.headers or {},
                auth_config=connection.auth_config or {}
            )
            
            # API 데이터 fetch
            result = asyncio.run(client.fetch_data())
            
            if result["status"] == "success":
                # 데이터 처리 및 저장
                processed_count = _process_and_save_data(
                    db, connection, result["data"]
                )
                
                # 연결 정보 업데이트
                connection.last_sync_at = datetime.now(timezone.utc)
                connection.sync_count += 1
                connection.last_error_message = None
                connection.next_sync_at = datetime.now(timezone.utc).replace(
                    second=0, microsecond=0
                ).timestamp() + connection.sync_interval
                
                # 성공 로그 업데이트
                sync_log.status = "success"
                sync_log.response_status = 200
                sync_log.records_processed = processed_count
                sync_log.completed_at = datetime.now(timezone.utc)
                
                db.commit()
                
                logger.info(f"Successfully synced connection {connection_id}, processed {processed_count} records")
                return {
                    "status": "success", 
                    "records_processed": processed_count,
                    "connection_id": connection_id
                }
                
            else:
                # 에러 처리
                connection.last_error_message = result.get("error", "Unknown error")
                sync_log.status = "error"
                sync_log.error_message = result.get("error")
                sync_log.completed_at = datetime.now(timezone.utc)
                
                db.commit()
                
                logger.error(f"Failed to sync connection {connection_id}: {result.get('error')}")
                return {"status": "error", "message": result.get("error")}
                
        except Exception as e:
            # 예외 처리
            error_msg = str(e)
            connection.last_error_message = error_msg
            sync_log.status = "error"
            sync_log.error_message = error_msg
            sync_log.completed_at = datetime.now(timezone.utc)
            
            db.commit()
            
            logger.error(f"Exception during sync for connection {connection_id}: {error_msg}")
            return {"status": "error", "message": error_msg}


@celery_app.task(name="external_api.sync_all_active")
def sync_all_active_connections() -> Dict[str, Any]:
    """
    모든 활성화된 API 연결에 대한 동기화를 수행합니다.
    """
    with get_db_session() as db:
        # 동기화가 필요한 연결들을 찾음
        now = datetime.now(timezone.utc)
        connections = db.query(ExternalApiConnection).filter(
            ExternalApiConnection.is_active == True,
            ExternalApiConnection.next_sync_at <= now.timestamp()
        ).all()
        
        if not connections:
            logger.info("No connections need syncing")
            return {"status": "success", "connections_synced": 0}
        
        # 각 연결에 대해 동기화 작업을 큐에 추가
        task_ids = []
        for connection in connections:
            task = sync_external_api_connection.delay(connection.id)
            task_ids.append(task.id)
            
        logger.info(f"Queued {len(task_ids)} sync tasks")
        return {
            "status": "success", 
            "connections_queued": len(task_ids),
            "task_ids": task_ids
        }


@celery_app.task(name="external_api.test_connection")
def test_api_connection(connection_data: Dict[str, Any]) -> Dict[str, Any]:
    """
    API 연결을 테스트합니다 (실제 저장하지 않음).
    """
    try:
        client = ExternalApiClient(
            base_url=connection_data["base_url"],
            headers=connection_data.get("headers", {}),
            auth_config=connection_data.get("auth_config", {})
        )
        
        result = asyncio.run(client.test_connection())
        
        logger.info(f"Connection test result: {result['status']}")
        return result
        
    except Exception as e:
        error_msg = str(e)
        logger.error(f"Connection test failed: {error_msg}")
        return {"status": "error", "message": error_msg}


def _process_and_save_data(
    db: Session,
    connection: ExternalApiConnection,
    api_data: List[Dict[str, Any]]
) -> int:
    """
    API에서 받은 데이터를 처리하고 데이터베이스에 저장합니다.

    처리 로직:
    1. 기존에 없는 데이터라면 신규로 추가
    2. 기존에 있는 데이터라면 수정시간을 변경
    3. API에도 없는데, 기존에 있는 데이터라면 status를 archived로 변경
    """
    processed_count = 0
    seen_identifiers = set()  # API에서 받은 데이터의 식별자 추적

    for item in api_data:
        # 필드 매핑 적용
        processed_item = _apply_field_mapping(item, connection.field_mapping or {})

        # 필터 조건으로 인해 제외된 경우 건너뛰기
        if processed_item is None:
            logger.debug(f"Item filtered out by field mapping conditions")
            continue

        # 데이터 해시 생성 (중복 감지용)
        data_hash = hashlib.sha256(
            json.dumps(item, sort_keys=True).encode()
        ).hexdigest()

        # 외부 ID 추출 (있는 경우)
        external_id = item.get("id") or item.get("_id") or item.get("uuid")

        # 식별자 추적 (external_id가 있으면 사용, 없으면 해시 사용)
        identifier = str(external_id) if external_id else data_hash
        seen_identifiers.add(identifier)

        # 기존 데이터 확인 (external_id가 None인 경우 해시로 비교)
        if external_id:
            existing_data = db.query(ExternalApiData).filter(
                ExternalApiData.connection_id == connection.id,
                ExternalApiData.external_id == str(external_id)
            ).first()
        else:
            # external_id가 없으면 해시로 중복 확인
            existing_data = db.query(ExternalApiData).filter(
                ExternalApiData.connection_id == connection.id,
                ExternalApiData.hash == data_hash
            ).first()

        if existing_data:
            # 기존 데이터 업데이트 (해시가 다르거나 status가 archived인 경우)
            needs_update = (existing_data.hash != data_hash or
                          existing_data.status == "archived")

            if needs_update:
                existing_data.raw_data = item
                existing_data.processed_data = processed_item
                existing_data.hash = data_hash
                existing_data.status = "active"  # archived였다면 다시 active로
                existing_data.last_sync_at = datetime.now(timezone.utc)
                logger.debug(f"Updated existing data with external_id={external_id}, hash={data_hash[:8]}")
            else:
                # 데이터는 같지만 수정시간은 업데이트
                existing_data.last_sync_at = datetime.now(timezone.utc)
                logger.debug(f"Synced existing data (no change) with external_id={external_id}")

            # 동기화된 레코드로 카운트
            processed_count += 1
        else:
            # 새로운 데이터 저장
            new_data = ExternalApiData(
                connection_id=connection.id,
                external_id=str(external_id) if external_id else None,
                data_type=_determine_data_type(item),
                raw_data=item,
                processed_data=processed_item,
                hash=data_hash,
                status="active",
                last_sync_at=datetime.now(timezone.utc)
            )
            db.add(new_data)
            processed_count += 1
            logger.debug(f"Added new data with external_id={external_id}, hash={data_hash[:8]}")

    # API 응답에 없는 기존 active 데이터를 archived로 변경
    existing_active_data = db.query(ExternalApiData).filter(
        ExternalApiData.connection_id == connection.id,
        ExternalApiData.status == "active"
    ).all()

    archived_count = 0
    for existing in existing_active_data:
        # 식별자 확인 (external_id가 있으면 사용, 없으면 해시 사용)
        identifier = existing.external_id if existing.external_id else existing.hash

        if identifier not in seen_identifiers:
            existing.status = "archived"
            existing.last_sync_at = datetime.now(timezone.utc)
            archived_count += 1
            logger.debug(f"Archived data with identifier={identifier}")

    # target_type에 따라 실제 테이블에 동기화
    if connection.target_type:
        synced_to_target = _sync_to_target_tables(db, connection, api_data)
        logger.info(f"Synced {synced_to_target} records to {connection.target_type} table")

    db.commit()
    logger.info(f"Processed {processed_count} records, archived {archived_count} records out of {len(api_data)} total items")
    return processed_count


def _sync_to_target_tables(
    db: Session,
    connection: ExternalApiConnection,
    api_data: List[Dict[str, Any]]
) -> int:
    """
    target_type에 따라 실제 비즈니스 테이블에 데이터를 동기화합니다.
    """
    synced_count = 0
    target_type = connection.target_type

    for item in api_data:
        # 필드 매핑 적용
        processed_item = _apply_field_mapping(item, connection.field_mapping or {})

        # 필터 조건으로 인해 제외된 경우 건너뛰기
        if processed_item is None:
            continue

        try:
            if target_type == "contact":
                synced_count += _sync_contact(db, processed_item, item)
            elif target_type == "device":
                synced_count += _sync_device(db, processed_item, item)
            elif target_type == "device_library":
                synced_count += _sync_device_library(db, processed_item, item)
            else:
                logger.warning(f"Unknown target_type: {target_type}")
        except Exception as e:
            logger.error(f"Failed to sync item to {target_type}: {str(e)}")
            continue

    return synced_count


def _sync_contact(db: Session, processed_data: Dict[str, Any], raw_data: Dict[str, Any]) -> int:
    """담당자 테이블에 데이터 동기화"""
    # email을 고유 식별자로 사용
    email = processed_data.get("email")
    if not email:
        logger.warning("Contact sync skipped: no email field")
        return 0

    existing = db.query(Contact).filter(Contact.email == email).first()

    if existing:
        # 기존 데이터 업데이트
        for key, value in processed_data.items():
            if hasattr(existing, key) and key not in ['id', 'created_at', 'created_by']:
                setattr(existing, key, value)
        existing.updated_at = datetime.now(timezone.utc)
        logger.debug(f"Updated contact: {email}")
    else:
        # 새 데이터 추가
        contact_data = {
            'id': uuid.uuid4(),
            'name': processed_data.get('name'),
            'title': processed_data.get('title'),
            'department': processed_data.get('department'),
            'phone': processed_data.get('phone'),
            'mobile': processed_data.get('mobile'),
            'email': email,
            'office_location': processed_data.get('office_location'),
            'responsibilities': processed_data.get('responsibilities'),
            'created_by': SYSTEM_USER_ID,
            'is_active': True,
            'created_at': datetime.now(timezone.utc),
            'updated_at': datetime.now(timezone.utc)
        }
        new_contact = Contact(**contact_data)
        db.add(new_contact)
        logger.debug(f"Created new contact: {email}")

    return 1


def _sync_device(db: Session, processed_data: Dict[str, Any], raw_data: Dict[str, Any]) -> int:
    """디바이스 테이블에 데이터 동기화"""
    # serial_number 또는 name을 고유 식별자로 사용
    serial_number = processed_data.get("serial_number")
    name = processed_data.get("name")

    if not serial_number and not name:
        logger.warning("Device sync skipped: no serial_number or name")
        return 0

    # serial_number가 있으면 우선 사용
    if serial_number:
        existing = db.query(Device).filter(Device.serial_number == serial_number).first()
    else:
        existing = db.query(Device).filter(Device.name == name).first()

    if existing:
        # 기존 데이터 업데이트
        for key, value in processed_data.items():
            if hasattr(existing, key) and key not in ['id', 'created_at', 'created_by']:
                setattr(existing, key, value)
        existing.updated_at = datetime.now(timezone.utc)
        logger.debug(f"Updated device: {serial_number or name}")
    else:
        # 새 데이터 추가
        device_data = {
            'id': uuid.uuid4(),
            'name': name,
            'description': processed_data.get('description'),
            'device_type': processed_data.get('device_type'),
            'manufacturer': processed_data.get('manufacturer'),
            'model': processed_data.get('model'),
            'serial_number': serial_number,
            'rack_position': processed_data.get('rack_position'),
            'rack_size': processed_data.get('rack_size'),
            'power_consumption': processed_data.get('power_consumption'),
            'status': processed_data.get('status', 'active'),
            'created_by': SYSTEM_USER_ID,
            'is_active': True,
            'created_at': datetime.now(timezone.utc),
            'updated_at': datetime.now(timezone.utc)
        }
        new_device = Device(**device_data)
        db.add(new_device)
        logger.debug(f"Created new device: {serial_number or name}")

    return 1


def _sync_device_library(db: Session, processed_data: Dict[str, Any], raw_data: Dict[str, Any]) -> int:
    """디바이스 라이브러리 테이블에 데이터 동기화"""
    # model 또는 name을 고유 식별자로 사용
    model = processed_data.get("model")
    name = processed_data.get("name")

    if not model and not name:
        logger.warning("Device library sync skipped: no model or name")
        return 0

    # model이 있으면 우선 사용
    if model:
        existing = db.query(DeviceLibrary).filter(DeviceLibrary.model == model).first()
    else:
        existing = db.query(DeviceLibrary).filter(DeviceLibrary.name == name).first()

    if existing:
        # 기존 데이터 업데이트
        for key, value in processed_data.items():
            if hasattr(existing, key) and key not in ['id', 'created_at', 'created_by']:
                setattr(existing, key, value)
        existing.updated_at = datetime.now(timezone.utc)
        logger.debug(f"Updated device library: {model or name}")
    else:
        # 새 데이터 추가
        library_data = {
            'id': uuid.uuid4(),
            'name': name,
            'description': processed_data.get('description'),
            'device_type': processed_data.get('device_type'),
            'manufacturer': processed_data.get('manufacturer'),
            'model': model,
            'default_rack_size': processed_data.get('default_rack_size'),
            'default_power_consumption': processed_data.get('default_power_consumption'),
            'default_config': processed_data.get('default_config'),
            'created_by': SYSTEM_USER_ID,
            'is_active': True,
            'created_at': datetime.now(timezone.utc),
            'updated_at': datetime.now(timezone.utc)
        }
        new_library = DeviceLibrary(**library_data)
        db.add(new_library)
        logger.debug(f"Created new device library: {model or name}")

    return 1


def _apply_field_mapping(data: Dict[str, Any], field_mapping: Dict[str, Any]) -> Dict[str, Any]:
    """
    필드 매핑 설정을 적용하여 데이터를 변환합니다.

    field_mapping 구조:
    {
        "mappings": [
            {
                "source_field": "api_field_name",
                "target_field": "desired_field_name",
                "data_type": "string|number|boolean|date",
                "default_value": "default_if_missing",
                "transformation": "uppercase|lowercase|trim|..."
            }
        ],
        "filter_conditions": [
            {
                "field": "status",
                "operator": "equals|contains|not_equals",
                "value": "active"
            }
        ]
    }
    """
    if not field_mapping or not isinstance(field_mapping, dict):
        return data

    mappings = field_mapping.get("mappings", [])

    # 필터 조건 확인 (먼저 확인하여 불필요한 매핑 작업 방지)
    filter_conditions = field_mapping.get("filter_conditions", [])
    if filter_conditions and not _check_filter_conditions(data, filter_conditions):
        return None  # 필터 조건에 맞지 않으면 제외

    # 매핑이 없으면 원본 데이터 반환
    if not mappings:
        return data

    result = {}

    # 필드 매핑 적용
    for mapping in mappings:
        source_field = mapping.get("source_field")
        target_field = mapping.get("target_field")
        data_type = mapping.get("data_type", "string")
        default_value = mapping.get("default_value")
        transformation = mapping.get("transformation")

        if not source_field or not target_field:
            continue

        # 원본 데이터에서 값 추출 (중첩 필드 지원)
        value = _get_nested_value(data, source_field)

        # 기본값 사용
        if value is None and default_value is not None:
            value = default_value

        # 데이터 타입 변환
        value = _convert_data_type(value, data_type)

        # 변환 적용
        value = _apply_transformation(value, transformation)

        # 타겟 필드에 값 설정 (중첩 필드 지원)
        _set_nested_value(result, target_field, value)

    return result


def _get_nested_value(data: Dict[str, Any], field_path: str) -> Any:
    """중첩된 필드 경로에서 값을 추출합니다. 예: 'user.profile.name'"""
    try:
        value = data
        for key in field_path.split('.'):
            if isinstance(value, dict) and key in value:
                value = value[key]
            else:
                return None
        return value
    except:
        return None


def _set_nested_value(data: Dict[str, Any], field_path: str, value: Any):
    """중첩된 필드 경로에 값을 설정합니다."""
    keys = field_path.split('.')
    current = data
    
    for key in keys[:-1]:
        if key not in current:
            current[key] = {}
        current = current[key]
    
    current[keys[-1]] = value


def _convert_data_type(value: Any, data_type: str) -> Any:
    """데이터 타입을 변환합니다."""
    if value is None:
        return None
        
    try:
        if data_type == "string":
            return str(value)
        elif data_type == "number":
            return float(value) if '.' in str(value) else int(value)
        elif data_type == "boolean":
            if isinstance(value, bool):
                return value
            return str(value).lower() in ('true', '1', 'yes', 'on')
        elif data_type == "date":
            if isinstance(value, str):
                from dateutil.parser import parse
                return parse(value).isoformat()
            return value
    except:
        return value
    
    return value


def _apply_transformation(value: Any, transformation: str) -> Any:
    """값에 변환을 적용합니다."""
    if value is None or transformation is None:
        return value
        
    try:
        if isinstance(value, str):
            if transformation == "uppercase":
                return value.upper()
            elif transformation == "lowercase":
                return value.lower()
            elif transformation == "trim":
                return value.strip()
            elif transformation == "title":
                return value.title()
    except:
        pass
    
    return value


def _check_filter_conditions(data: Dict[str, Any], conditions: List[Dict[str, Any]]) -> bool:
    """필터 조건을 확인합니다."""
    for condition in conditions:
        field = condition.get("field")
        operator = condition.get("operator")
        expected_value = condition.get("value")
        
        if not field or not operator:
            continue
            
        actual_value = _get_nested_value(data, field)
        
        if operator == "equals" and actual_value != expected_value:
            return False
        elif operator == "not_equals" and actual_value == expected_value:
            return False
        elif operator == "contains" and expected_value not in str(actual_value):
            return False
            
    return True


def _determine_data_type(data: Dict[str, Any]) -> str:
    """
    API 데이터의 타입을 결정합니다.
    """
    # 데이터 구조를 기반으로 타입 추론
    if "device" in str(data).lower() or "serial_number" in data:
        return "device"
    elif "user" in str(data).lower() or "email" in data:
        return "user"
    elif "log" in str(data).lower() or "timestamp" in data:
        return "log"
    else:
        return "general"


@celery_app.task(name="external_api.cleanup_old_data")
def cleanup_old_api_data(days_to_keep: int = 30) -> Dict[str, Any]:
    """
    오래된 API 데이터를 정리합니다.
    """
    from datetime import timedelta
    
    with get_db_session() as db:
        cutoff_date = datetime.now(timezone.utc) - timedelta(days=days_to_keep)
        
        # 오래된 로그 삭제
        deleted_logs = db.query(ExternalApiSyncLog).filter(
            ExternalApiSyncLog.started_at < cutoff_date
        ).delete()
        
        # 비활성 상태인 오래된 데이터 삭제
        deleted_data = db.query(ExternalApiData).filter(
            ExternalApiData.status == "archived",
            ExternalApiData.last_sync_at < cutoff_date
        ).delete()
        
        db.commit()
        
        logger.info(f"Cleaned up {deleted_logs} old sync logs and {deleted_data} old data records")
        return {
            "status": "success",
            "deleted_logs": deleted_logs,
            "deleted_data": deleted_data
        }