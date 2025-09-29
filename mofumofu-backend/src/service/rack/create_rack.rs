use crate::dto::rack::request::create_rack::CreateRackRequest;
use crate::dto::rack::response::rack_info::RackInfoResponse;
use crate::repository::rack::repository_create_rack;
use crate::service::error::errors::Errors;
use crate::entity::server_rooms::{self, Entity as ServerRoom};
use sea_orm::{ConnectionTrait, EntityTrait, Set, ActiveModelTrait};
use uuid::Uuid;
use std::str::FromStr;
use chrono::Utc;

pub async fn service_create_rack<C>(
    conn: &C,
    request: CreateRackRequest,
    server_room_id: Uuid,
    created_by: Uuid,
) -> Result<RackInfoResponse, Errors>
where
    C: ConnectionTrait,
{
    // 서버룸이 존재하는지 확인하고, 없으면 생성
    let existing_server_room = ServerRoom::find_by_id(server_room_id).one(conn).await
        .map_err(|e| Errors::DatabaseError(format!("서버룸 조회 오류: {:?}", e)))?;

    if existing_server_room.is_none() {
        // 서버룸이 없으면 기본 서버룸 생성
        let office_id = Uuid::parse_str("299a5075-1460-4bc9-9fe1-3d227f897dcd")
            .map_err(|e| Errors::DatabaseError(format!("Invalid office ID: {:?}", e)))?;

        // 실제 서버룸 ID가 e0d147a1-8790-4112-923a-f790c1c1b326이므로
        // 자동 생성이 필요하지 않을 수도 있지만, 안전을 위해 로직 유지
        let server_room_model = server_rooms::ActiveModel {
            id: Set(server_room_id),
            office_id: Set(office_id),
            name: Set("10A 메인 서버룸".to_string()),
            description: Set(Some("10A 사무실 메인 서버룸".to_string())),
            floor_level: Set(Some("1F".to_string())),
            room_number: Set(Some("SR-001".to_string())),
            temperature_monitoring: Set(true),
            humidity_monitoring: Set(true),
            access_control: Set(true),
            created_by: Set(created_by),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            is_active: Set(true),
        };

        server_room_model.insert(conn).await
            .map_err(|e| Errors::DatabaseError(format!("서버룸 생성 오류: {:?}", e)))?;
    }

    let rack = repository_create_rack(
        conn,
        &server_room_id,
        &request.name,
        request.description.as_deref(),
        request.rack_height,
        request.power_capacity,
        request.cooling_type.as_deref(),
        request.location_x,
        request.location_y,
        &created_by,
    )
    .await?;

    Ok(RackInfoResponse {
        id: rack.id,
        server_room_id: rack.server_room_id,
        name: rack.name,
        description: rack.description,
        rack_height: rack.rack_height,
        power_capacity: rack.power_capacity,
        cooling_type: rack.cooling_type,
        location_x: rack.location_x.and_then(|d| d.to_string().parse().ok()),
        location_y: rack.location_y.and_then(|d| d.to_string().parse().ok()),
        created_at: rack.created_at.into(),
        updated_at: rack.updated_at.into(),
    })
}