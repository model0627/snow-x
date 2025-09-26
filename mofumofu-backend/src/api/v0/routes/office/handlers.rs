use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;

use crate::{
    dto::auth::internal::access_token::AccessTokenClaims,
    entity::office::{self, Entity as Office},
    state::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct CreateOfficeRequest {
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOfficeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct ListOfficesQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct OfficeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub is_active: bool,
}

#[derive(Serialize, ToSchema)]
pub struct OfficeListResponse {
    pub offices: Vec<OfficeResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

/// Create new office
#[utoipa::path(
    post,
    path = "/v0/ipam/office",
    tag = "Office",
    request_body = CreateOfficeRequest,
    responses(
        (status = 201, description = "Office created successfully", body = OfficeResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn create_office(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(request): Json<CreateOfficeRequest>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    let office = office::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(request.name),
        description: Set(request.description),
        address: Set(request.address),
        contact_person: Set(request.contact_person),
        phone: Set(request.phone),
        email: Set(request.email),
        created_by: Set(claims.sub),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        is_active: Set(true),
    };

    let result = office.insert(&state.conn).await;

    match result {
        Ok(office) => {
            let response = OfficeResponse {
                id: office.id,
                name: office.name,
                description: office.description,
                address: office.address,
                contact_person: office.contact_person,
                phone: office.phone,
                email: office.email,
                created_by: office.created_by,
                created_at: office.created_at,
                updated_at: office.updated_at,
                is_active: office.is_active,
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            eprintln!("Failed to create office: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get all offices with pagination
#[utoipa::path(
    get,
    path = "/v0/ipam/office",
    tag = "Office",
    params(ListOfficesQuery),
    responses(
        (status = 200, description = "Office list retrieved successfully", body = OfficeListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn get_offices(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Query(query): Query<ListOfficesQuery>,
) -> impl IntoResponse {
    println!("DEBUG: get_offices handler called!");
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    let mut select = Office::find()
        .filter(office::Column::IsActive.eq(true));

    if let Some(search) = query.search {
        select = select.filter(
            office::Column::Name
                .contains(&search)
                .or(office::Column::Address.contains(&search)),
        );
    }

    let paginator = select
        .order_by_desc(office::Column::CreatedAt)
        .paginate(&state.conn, limit);

    let total = paginator.num_items().await.unwrap_or(0);
    let offices = paginator.fetch_page(page - 1).await.unwrap_or_default();

    let response = OfficeListResponse {
        offices: offices
            .into_iter()
            .map(|o| OfficeResponse {
                id: o.id,
                name: o.name,
                description: o.description,
                address: o.address,
                contact_person: o.contact_person,
                phone: o.phone,
                email: o.email,
                created_by: o.created_by,
                created_at: o.created_at,
                updated_at: o.updated_at,
                is_active: o.is_active,
            })
            .collect(),
        total,
        page,
        limit,
    };

    (StatusCode::OK, Json(response))
}

/// Get single office by ID
#[utoipa::path(
    get,
    path = "/v0/ipam/office/{id}",
    tag = "Office",
    params(
        ("id" = Uuid, Path, description = "Office ID")
    ),
    responses(
        (status = 200, description = "Office retrieved successfully", body = OfficeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Office not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn get_office(
    State(state): State<AppState>,
    Extension(_claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let office = Office::find_by_id(id)
        .filter(office::Column::IsActive.eq(true))
        .one(&state.conn)
        .await;

    match office {
        Ok(Some(office)) => {
            let response = OfficeResponse {
                id: office.id,
                name: office.name,
                description: office.description,
                address: office.address,
                contact_person: office.contact_person,
                phone: office.phone,
                email: office.email,
                created_by: office.created_by,
                created_at: office.created_at,
                updated_at: office.updated_at,
                is_active: office.is_active,
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Update office
#[utoipa::path(
    put,
    path = "/v0/ipam/office/{id}",
    tag = "Office",
    params(
        ("id" = Uuid, Path, description = "Office ID")
    ),
    request_body = UpdateOfficeRequest,
    responses(
        (status = 200, description = "Office updated successfully", body = OfficeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - only creator can update"),
        (status = 404, description = "Office not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn update_office(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateOfficeRequest>,
) -> impl IntoResponse {
    let office = Office::find_by_id(id)
        .filter(office::Column::IsActive.eq(true))
        .one(&state.conn)
        .await;

    match office {
        Ok(Some(office)) => {
            // Check if user is the creator
            if office.created_by != claims.sub {
                return Err(StatusCode::FORBIDDEN);
            }

            let mut office: office::ActiveModel = office.into();

            if let Some(name) = request.name {
                office.name = Set(name);
            }
            if let Some(description) = request.description {
                office.description = Set(Some(description));
            }
            if let Some(address) = request.address {
                office.address = Set(address);
            }
            if let Some(contact_person) = request.contact_person {
                office.contact_person = Set(Some(contact_person));
            }
            if let Some(phone) = request.phone {
                office.phone = Set(Some(phone));
            }
            if let Some(email) = request.email {
                office.email = Set(Some(email));
            }

            office.updated_at = Set(chrono::Utc::now().into());

            let updated = office.update(&state.conn).await;

            match updated {
                Ok(office) => {
                    let response = OfficeResponse {
                        id: office.id,
                        name: office.name,
                        description: office.description,
                        address: office.address,
                        contact_person: office.contact_person,
                        phone: office.phone,
                        email: office.email,
                        created_by: office.created_by,
                        created_at: office.created_at,
                        updated_at: office.updated_at,
                        is_active: office.is_active,
                    };
                    Ok((StatusCode::OK, Json(response)))
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Delete office (soft delete)
#[utoipa::path(
    delete,
    path = "/v0/ipam/office/{id}",
    tag = "Office",
    params(
        ("id" = Uuid, Path, description = "Office ID")
    ),
    responses(
        (status = 204, description = "Office deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - only creator can delete"),
        (status = 404, description = "Office not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer" = []))
)]
pub async fn delete_office(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let office = Office::find_by_id(id)
        .filter(office::Column::IsActive.eq(true))
        .one(&state.conn)
        .await;

    match office {
        Ok(Some(office)) => {
            // Check if user is the creator
            if office.created_by != claims.sub {
                return Err(StatusCode::FORBIDDEN);
            }

            let mut office: office::ActiveModel = office.into();
            office.is_active = Set(false);
            office.updated_at = Set(chrono::Utc::now().into());

            match office.update(&state.conn).await {
                Ok(_) => Ok(StatusCode::NO_CONTENT),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}