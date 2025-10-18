from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from app.workers.custodian_worker import execute_custodian_policy
from typing import Optional
import uuid

router = APIRouter(prefix="/custodian", tags=["custodian"])


class ExecutePolicyRequest(BaseModel):
    policy_id: str
    policy_content: str
    execution_id: str
    dry_run: bool = False


class ExecutePolicyResponse(BaseModel):
    task_id: str
    execution_id: str
    status: str


@router.post("/execute", response_model=ExecutePolicyResponse)
async def execute_policy(request: ExecutePolicyRequest):
    """
    Execute a Cloud Custodian policy using Celery worker.

    Args:
        request: Policy execution request with content and options

    Returns:
        Task information including task_id and execution_id
    """
    try:
        # Queue the task with Celery
        task = execute_custodian_policy.delay(
            policy_id=request.policy_id,
            policy_content=request.policy_content,
            execution_id=request.execution_id,
            dry_run=request.dry_run
        )

        return ExecutePolicyResponse(
            task_id=task.id,
            execution_id=request.execution_id,
            status="queued"
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to queue policy execution: {str(e)}")


@router.get("/task-status/{task_id}")
async def get_task_status(task_id: str):
    """
    Get the status of a Celery task.

    Args:
        task_id: The Celery task ID

    Returns:
        Task status information
    """
    from celery.result import AsyncResult
    from app.core.celery_app import celery_app

    task = AsyncResult(task_id, app=celery_app)

    return {
        "task_id": task_id,
        "status": task.status,
        "result": task.result if task.ready() else None,
        "error": str(task.info) if task.failed() else None
    }
