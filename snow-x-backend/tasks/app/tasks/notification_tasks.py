from celery.utils.log import get_task_logger

from app.core.celery_app import celery_app
from app.core.database import get_db_session_context
from app.services.notification_service import (
    deliver_notification,
    fetch_pending_notifications,
    mark_done,
    mark_processing,
    reschedule_failure,
)

logger = get_task_logger(__name__)


@celery_app.task(name="notifications.dispatch_pending")
def dispatch_pending(limit: int = 20) -> dict:
    """주기적으로 대기 중인 알림을 찾아 전송한다."""
    with get_db_session_context() as session:
        pending = fetch_pending_notifications(session, limit=limit)

        if not pending:
            logger.debug("No pending notifications to process")
            return {"processed": 0}

        for notification in pending:
            mark_processing(session, notification)

        session.flush()

        processed = 0
        for notification in pending:
            try:
                deliver_notification(notification)
            except Exception as exc:  # pylint: disable=broad-except
                logger.exception(
                    "Failed to deliver notification %s: %s", notification.id, exc
                )
                reschedule_failure(session, notification, str(exc))
            else:
                mark_done(session, notification)
                processed += 1

        session.flush()
        logger.info(
            "Notification dispatcher processed %s notifications (total fetched=%s)",
            processed,
            len(pending),
        )

        return {"processed": processed, "fetched": len(pending)}
