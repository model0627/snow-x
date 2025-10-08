use crate::entity::{custodian_executions, custodian_policies};
use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct UpdatePolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExecutePolicyRequest {
    pub policy_id: Uuid,
    pub dry_run: bool,
}

/// Get all custodian policies
pub async fn get_all_policies(
    db: &DatabaseConnection,
) -> Result<Vec<custodian_policies::Model>, DbErr> {
    custodian_policies::Entity::find()
        .order_by_desc(custodian_policies::Column::CreatedAt)
        .all(db)
        .await
}

/// Get a single custodian policy by ID
pub async fn get_policy_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<custodian_policies::Model>, DbErr> {
    custodian_policies::Entity::find_by_id(id).one(db).await
}

/// Create a new custodian policy
pub async fn create_policy(
    db: &DatabaseConnection,
    request: CreatePolicyRequest,
) -> Result<custodian_policies::Model, DbErr> {
    let now = Utc::now();
    let policy = custodian_policies::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(request.name),
        description: Set(request.description),
        content: Set(request.content),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    policy.insert(db).await
}

/// Update an existing custodian policy
pub async fn update_policy(
    db: &DatabaseConnection,
    id: Uuid,
    request: UpdatePolicyRequest,
) -> Result<custodian_policies::Model, DbErr> {
    let policy = custodian_policies::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "Policy with id {} not found",
            id
        )))?;

    let mut active_model: custodian_policies::ActiveModel = policy.into();

    if let Some(name) = request.name {
        active_model.name = Set(name);
    }
    if let Some(description) = request.description {
        active_model.description = Set(Some(description));
    }
    if let Some(content) = request.content {
        active_model.content = Set(content);
    }
    active_model.updated_at = Set(Utc::now().into());

    active_model.update(db).await
}

/// Delete a custodian policy
pub async fn delete_policy(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
    let result = custodian_policies::Entity::delete_by_id(id)
        .exec(db)
        .await?;

    if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "Policy with id {} not found",
            id
        )));
    }

    Ok(())
}

/// Create an execution record
pub async fn create_execution(
    db: &DatabaseConnection,
    policy_id: Uuid,
    dry_run: bool,
) -> Result<custodian_executions::Model, DbErr> {
    let execution = custodian_executions::ActiveModel {
        id: Set(Uuid::new_v4()),
        policy_id: Set(policy_id),
        status: Set("pending".to_string()),
        dry_run: Set(dry_run),
        task_id: Set(None),
        output: Set(None),
        error: Set(None),
        started_at: Set(Utc::now().into()),
        completed_at: Set(None),
    };

    execution.insert(db).await
}

/// Update execution with task_id
pub async fn update_execution_task_id(
    db: &DatabaseConnection,
    execution_id: Uuid,
    task_id: String,
) -> Result<custodian_executions::Model, DbErr> {
    let execution = custodian_executions::Entity::find_by_id(execution_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "Execution with id {} not found",
            execution_id
        )))?;

    let mut active_model: custodian_executions::ActiveModel = execution.into();
    active_model.task_id = Set(Some(task_id));
    active_model.update(db).await
}

/// Update execution status
pub async fn update_execution_status(
    db: &DatabaseConnection,
    execution_id: Uuid,
    status: String,
    output: Option<String>,
    error: Option<String>,
) -> Result<custodian_executions::Model, DbErr> {
    let execution = custodian_executions::Entity::find_by_id(execution_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "Execution with id {} not found",
            execution_id
        )))?;

    let mut active_model: custodian_executions::ActiveModel = execution.into();
    let is_final_status = status == "completed" || status == "failed";
    active_model.status = Set(status);
    if output.is_some() {
        active_model.output = Set(output);
    }
    if error.is_some() {
        active_model.error = Set(error);
    }
    if is_final_status {
        active_model.completed_at = Set(Some(Utc::now().into()));
    }

    active_model.update(db).await
}

/// Get execution by ID
pub async fn get_execution_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<custodian_executions::Model>, DbErr> {
    custodian_executions::Entity::find_by_id(id).one(db).await
}

/// Get executions for a policy
pub async fn get_executions_for_policy(
    db: &DatabaseConnection,
    policy_id: Uuid,
) -> Result<Vec<custodian_executions::Model>, DbErr> {
    custodian_executions::Entity::find()
        .filter(custodian_executions::Column::PolicyId.eq(policy_id))
        .order_by_desc(custodian_executions::Column::StartedAt)
        .all(db)
        .await
}
