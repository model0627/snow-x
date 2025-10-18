import os
import tempfile
import subprocess
from celery import Task
from app.core.celery_app import celery_app
from app.core.database import get_db_session_context
from app.core.config import settings
from sqlalchemy import text
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class CustodianTask(Task):
    """Base task class for custodian execution with error handling."""

    def on_failure(self, exc, task_id, args, kwargs, einfo):
        """Handle task failure by updating execution record."""
        execution_id = kwargs.get('execution_id')
        if execution_id:
            try:
                with get_db_session_context() as db:
                    db.execute(
                        text("""
                            UPDATE custodian_executions
                            SET status = :status,
                                error = :error,
                                completed_at = :completed_at
                            WHERE id = :execution_id
                        """),
                        {
                            "status": "failed",
                            "error": str(exc),
                            "completed_at": datetime.utcnow(),
                            "execution_id": execution_id
                        }
                    )
            except Exception as e:
                logger.error(f"Failed to update execution record: {e}")


@celery_app.task(base=CustodianTask, bind=True)
def execute_custodian_policy(
    self,
    policy_id: str,
    policy_content: str,
    execution_id: str,
    dry_run: bool = False
):
    """
    Execute a Cloud Custodian policy.

    Args:
        policy_id: UUID of the policy
        policy_content: YAML content of the policy
        execution_id: UUID of the execution record
        dry_run: Whether to run in dry-run mode

    Returns:
        dict: Execution result with output and status
    """
    logger.info(f"Starting custodian execution for policy {policy_id}, execution {execution_id}, dry_run={dry_run}")

    # Update status to running
    try:
        with get_db_session_context() as db:
            db.execute(
                text("""
                    UPDATE custodian_executions
                    SET status = :status
                    WHERE id = :execution_id
                """),
                {"status": "running", "execution_id": execution_id}
            )
    except Exception as e:
        logger.error(f"Failed to update execution status to running: {e}")
        raise

    # Create temporary directory for policy and output
    with tempfile.TemporaryDirectory() as temp_dir:
        policy_file = os.path.join(temp_dir, "policy.yml")
        output_dir = os.path.join(temp_dir, "output")

        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        # Write policy content to file
        try:
            with open(policy_file, 'w') as f:
                f.write(policy_content)
            logger.info(f"Policy written to {policy_file}")
        except Exception as e:
            logger.error(f"Failed to write policy file: {e}")
            raise

        # Build custodian command
        cmd = ["custodian", "run"]

        if dry_run:
            cmd.append("--dryrun")

        cmd.extend(["-s", output_dir, policy_file])

        logger.info(f"Executing command: {' '.join(cmd)}")

        # Prepare environment variables with AWS credentials
        env = os.environ.copy()
        env.update({
            'AWS_DEFAULT_REGION': settings.AWS_DEFAULT_REGION,
            'AWS_ACCESS_KEY_ID': settings.AWS_ACCESS_KEY_ID,
            'AWS_SECRET_ACCESS_KEY': settings.AWS_SECRET_ACCESS_KEY,
        })

        logger.info(f"Using AWS region: {settings.AWS_DEFAULT_REGION}")

        # Execute custodian command
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=300,  # 5 minute timeout
                cwd=temp_dir,
                env=env
            )

            output = result.stdout
            error = result.stderr
            status = "completed" if result.returncode == 0 else "failed"

            logger.info(f"Custodian execution completed with status: {status}")
            logger.info(f"Return code: {result.returncode}")
            logger.info(f"Output: {output[:500]}...")  # Log first 500 chars

            if error:
                logger.warning(f"Error output: {error[:500]}...")

            # Read output files if they exist
            output_files = []
            try:
                for root, dirs, files in os.walk(output_dir):
                    for file in files:
                        file_path = os.path.join(root, file)
                        try:
                            with open(file_path, 'r') as f:
                                output_files.append({
                                    "file": file,
                                    "content": f.read()
                                })
                        except Exception as e:
                            logger.warning(f"Failed to read output file {file}: {e}")
            except Exception as e:
                logger.warning(f"Failed to read output directory: {e}")

            # Combine all output
            full_output = f"=== Command Output ===\n{output}\n"

            if error:
                full_output += f"\n=== Error Output ===\n{error}\n"

            if output_files:
                full_output += "\n=== Output Files ===\n"
                for file_info in output_files:
                    full_output += f"\n--- {file_info['file']} ---\n{file_info['content']}\n"

            # Update execution record
            try:
                with get_db_session_context() as db:
                    db.execute(
                        text("""
                            UPDATE custodian_executions
                            SET status = :status,
                                output = :output,
                                error = :error,
                                completed_at = :completed_at
                            WHERE id = :execution_id
                        """),
                        {
                            "status": status,
                            "output": full_output,
                            "error": error if status == "failed" else None,
                            "completed_at": datetime.utcnow(),
                            "execution_id": execution_id
                        }
                    )
                logger.info(f"Execution record updated successfully")
            except Exception as e:
                logger.error(f"Failed to update execution record: {e}")
                raise

            return {
                "status": status,
                "output": full_output,
                "return_code": result.returncode
            }

        except subprocess.TimeoutExpired:
            error_msg = "Custodian execution timed out after 5 minutes"
            logger.error(error_msg)

            try:
                with get_db_session_context() as db:
                    db.execute(
                        text("""
                            UPDATE custodian_executions
                            SET status = :status,
                                error = :error,
                                completed_at = :completed_at
                            WHERE id = :execution_id
                        """),
                        {
                            "status": "failed",
                            "error": error_msg,
                            "completed_at": datetime.utcnow(),
                            "execution_id": execution_id
                        }
                    )
            except Exception as e:
                logger.error(f"Failed to update execution record: {e}")

            raise

        except Exception as e:
            logger.error(f"Failed to execute custodian: {e}")

            try:
                with get_db_session_context() as db:
                    db.execute(
                        text("""
                            UPDATE custodian_executions
                            SET status = :status,
                                error = :error,
                                completed_at = :completed_at
                            WHERE id = :execution_id
                        """),
                        {
                            "status": "failed",
                            "error": str(e),
                            "completed_at": datetime.utcnow(),
                            "execution_id": execution_id
                        }
                    )
            except Exception as db_error:
                logger.error(f"Failed to update execution record: {db_error}")

            raise
