use crate::service::custodian_service;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdatePolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExecutePolicyRequest {
    pub policy_id: Uuid,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ValidateYamlRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ValidateYamlResponse {
    pub valid: bool,
    pub error: Option<String>,
}

/// Get all custodian policies
#[utoipa::path(
    get,
    path = "/v0/custodian/policies",
    tag = "custodian",
    responses(
        (status = 200, description = "List of custodian policies", body = Vec<crate::entity::custodian_policies::Model>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_policies(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let policies = custodian_service::get_all_policies(&state.conn)
        .await
        .map_err(|e| {
            eprintln!("Failed to get policies: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(policies))
}

/// Get a single custodian policy by ID
#[utoipa::path(
    get,
    path = "/v0/custodian/policies/{id}",
    tag = "custodian",
    params(
        ("id" = Uuid, Path, description = "Policy ID")
    ),
    responses(
        (status = 200, description = "Custodian policy", body = crate::entity::custodian_policies::Model),
        (status = 404, description = "Policy not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_policy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let policy = custodian_service::get_policy_by_id(&state.conn, id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get policy: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(policy))
}

/// Create a new custodian policy
#[utoipa::path(
    post,
    path = "/v0/custodian/policies",
    tag = "custodian",
    request_body = CreatePolicyRequest,
    responses(
        (status = 201, description = "Policy created", body = crate::entity::custodian_policies::Model),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_policy(
    State(state): State<AppState>,
    Json(request): Json<CreatePolicyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let service_request = custodian_service::CreatePolicyRequest {
        name: request.name,
        description: request.description,
        content: request.content,
    };

    let policy = custodian_service::create_policy(&state.conn, service_request)
        .await
        .map_err(|e| {
            eprintln!("Failed to create policy: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(policy)))
}

/// Update an existing custodian policy
#[utoipa::path(
    put,
    path = "/v0/custodian/policies/{id}",
    tag = "custodian",
    params(
        ("id" = Uuid, Path, description = "Policy ID")
    ),
    request_body = UpdatePolicyRequest,
    responses(
        (status = 200, description = "Policy updated", body = crate::entity::custodian_policies::Model),
        (status = 404, description = "Policy not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_policy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePolicyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let service_request = custodian_service::UpdatePolicyRequest {
        name: request.name,
        description: request.description,
        content: request.content,
    };

    let policy = custodian_service::update_policy(&state.conn, id, service_request)
        .await
        .map_err(|e| {
            eprintln!("Failed to update policy: {}", e);
            match e {
                sea_orm::DbErr::RecordNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(policy))
}

/// Delete a custodian policy
#[utoipa::path(
    delete,
    path = "/v0/custodian/policies/{id}",
    tag = "custodian",
    params(
        ("id" = Uuid, Path, description = "Policy ID")
    ),
    responses(
        (status = 204, description = "Policy deleted"),
        (status = 404, description = "Policy not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_policy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    custodian_service::delete_policy(&state.conn, id)
        .await
        .map_err(|e| {
            eprintln!("Failed to delete policy: {}", e);
            match e {
                sea_orm::DbErr::RecordNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

/// Execute a custodian policy
#[utoipa::path(
    post,
    path = "/v0/custodian/execute",
    tag = "custodian",
    request_body = ExecutePolicyRequest,
    responses(
        (status = 200, description = "Execution started", body = crate::entity::custodian_executions::Model),
        (status = 404, description = "Policy not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn execute_policy(
    State(state): State<AppState>,
    Json(request): Json<ExecutePolicyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify policy exists
    let policy = custodian_service::get_policy_by_id(&state.conn, request.policy_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get policy: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create execution record
    let execution = custodian_service::create_execution(&state.conn, policy.id, request.dry_run)
        .await
        .map_err(|e| {
            eprintln!("Failed to create execution: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Send execution request to Task API (Celery)
    let task_api_url = std::env::var("TASK_API_URL").unwrap_or_else(|_| "http://localhost:7000".to_string());
    let task_request = serde_json::json!({
        "policy_id": policy.id.to_string(),
        "policy_content": policy.content,
        "execution_id": execution.id.to_string(),
        "dry_run": request.dry_run
    });

    let task_response = state
        .http_client
        .post(format!("{}/tasks/custodian/execute", task_api_url))
        .json(&task_request)
        .send()
        .await;

    match task_response {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!("Task API returned error: {}", response.status());
                // Continue anyway, execution record is created
            } else {
                match response.json::<serde_json::Value>().await {
                    Ok(task_data) => {
                        println!("Task queued successfully: {:?}", task_data);
                        // Update execution with task_id
                        if let Some(task_id) = task_data.get("task_id").and_then(|v| v.as_str()) {
                            let _ = custodian_service::update_execution_task_id(
                                &state.conn,
                                execution.id,
                                task_id.to_string(),
                            ).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse task response: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to call Task API: {}", e);
            // Continue anyway, execution record is created
            // The execution will remain in "pending" status
        }
    }

    // Reload execution to get updated task_id
    let execution = custodian_service::get_execution_by_id(&state.conn, execution.id)
        .await
        .map_err(|e| {
            eprintln!("Failed to reload execution: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(execution))
}

/// Get execution result
#[utoipa::path(
    get,
    path = "/v0/custodian/executions/{execution_id}",
    tag = "custodian",
    params(
        ("execution_id" = Uuid, Path, description = "Execution ID")
    ),
    responses(
        (status = 200, description = "Execution result", body = crate::entity::custodian_executions::Model),
        (status = 404, description = "Execution not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_execution_result(
    State(state): State<AppState>,
    Path(execution_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let execution = custodian_service::get_execution_by_id(&state.conn, execution_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get execution: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(execution))
}

/// Get all executions for a policy
#[utoipa::path(
    get,
    path = "/v0/custodian/policies/{id}/executions",
    tag = "custodian",
    params(
        ("id" = Uuid, Path, description = "Policy ID")
    ),
    responses(
        (status = 200, description = "List of executions", body = Vec<crate::entity::custodian_executions::Model>),
        (status = 404, description = "Policy not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_policy_executions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify policy exists
    let _policy = custodian_service::get_policy_by_id(&state.conn, id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get policy: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let executions = custodian_service::get_executions_for_policy(&state.conn, id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get executions: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(executions))
}

/// Validate YAML syntax
#[utoipa::path(
    post,
    path = "/v0/custodian/validate",
    tag = "custodian",
    request_body = ValidateYamlRequest,
    responses(
        (status = 200, description = "Validation result", body = ValidateYamlResponse),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn validate_yaml(
    Json(request): Json<ValidateYamlRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Basic YAML validation
    match serde_yaml::from_str::<serde_yaml::Value>(&request.content) {
        Ok(_) => Ok(Json(ValidateYamlResponse {
            valid: true,
            error: None,
        })),
        Err(e) => Ok(Json(ValidateYamlResponse {
            valid: false,
            error: Some(e.to_string()),
        })),
    }
}
