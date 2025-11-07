from __future__ import annotations

import logging
from datetime import datetime, timedelta, timezone
from typing import Iterable, List, Optional

import httpx
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.core.config import settings
from app.models.notification import NotificationOutbox

logger = logging.getLogger(__name__)

STATUS_PENDING = "pending"
STATUS_PROCESSING = "processing"
STATUS_DONE = "done"
STATUS_FAILED = "failed"


def _now() -> datetime:
    return datetime.now(timezone.utc)


def fetch_pending_notifications(
    session: Session,
    limit: int = 20,
    exclude_channels: Iterable[str] = ("web",),
) -> List[NotificationOutbox]:
    """가공 대기 중인 알림을 가져온다. (행 잠금 + skip locked)"""
    stmt = (
        select(NotificationOutbox)
        .where(NotificationOutbox.status == STATUS_PENDING)
        .where(NotificationOutbox.scheduled_at <= _now())
        .order_by(NotificationOutbox.scheduled_at, NotificationOutbox.created_at)
        .limit(limit)
        .with_for_update(skip_locked=True)
    )

    exclude = tuple(exclude_channels or [])
    if exclude:
        stmt = stmt.where(~NotificationOutbox.channel.in_(exclude))

    result = session.execute(stmt)
    notifications = result.scalars().all()
    logger.debug("Fetched %s pending notifications", len(notifications))
    return notifications


def mark_processing(session: Session, notification: NotificationOutbox) -> None:
    now = _now()
    notification.status = STATUS_PROCESSING
    notification.processing_started_at = now
    notification.updated_at = now
    session.flush()


def mark_done(session: Session, notification: NotificationOutbox) -> None:
    now = _now()
    notification.status = STATUS_DONE
    notification.processed_at = now
    notification.updated_at = now
    notification.last_error = None
    session.flush()


def reschedule_failure(
    session: Session,
    notification: NotificationOutbox,
    error_message: str,
) -> None:
    now = _now()
    notification.retry_count += 1
    notification.last_error = error_message[:1000]
    notification.updated_at = now

    if notification.retry_count >= notification.max_retries:
        notification.status = STATUS_FAILED
        notification.processed_at = now
        logger.warning(
            "Notification %s failed permanently after %s retries",
            notification.id,
            notification.retry_count,
        )
    else:
        backoff_minutes = min(15, max(1, notification.retry_count))
        notification.status = STATUS_PENDING
        notification.processing_started_at = None
        notification.scheduled_at = now + timedelta(minutes=backoff_minutes)
        logger.info(
            "Notification %s scheduled for retry %s at %s",
            notification.id,
            notification.retry_count,
            notification.scheduled_at,
        )
    session.flush()


def deliver_notification(notification: NotificationOutbox) -> None:
    channel = (notification.channel or "").lower()

    if channel in ("slack", "slack_webhook"):
        _deliver_slack(notification)
    elif channel in ("webhook", "http_webhook"):
        _deliver_webhook(notification)
    else:
        raise ValueError(f"Unsupported notification channel: '{notification.channel}'")


def _extract_payload(notification: NotificationOutbox) -> dict:
    payload = notification.payload or {}
    if not isinstance(payload, dict):
        logger.warning(
            "Notification %s payload is not a dict. Falling back to empty dict.",
            notification.id,
        )
        return {}
    return payload


def _deliver_slack(notification: NotificationOutbox) -> None:
    payload = _extract_payload(notification)
    slack_payload = payload.get("slack_payload") or payload.get("slack") or {}

    if slack_payload and not isinstance(slack_payload, dict):
        logger.warning(
            "Notification %s slack payload must be a dict. Ignoring provided value.",
            notification.id,
        )
        slack_payload = {}

    webhook_url: Optional[str] = (
        slack_payload.get("webhook_url")
        or payload.get("webhook_url")
        or settings.SLACK_ALERT_WEBHOOK
    )

    if not webhook_url:
        raise ValueError(
            "Slack webhook URL not configured (missing webhook_url payload or SLACK_ALERT_WEBHOOK)"
        )

    body = dict(slack_payload)
    text = notification.message or notification.title or payload.get("text") or "새로운 알림이 도착했습니다."
    body.setdefault("text", text)

    if "blocks" not in body and notification.title and notification.message:
        body["blocks"] = [
            {
                "type": "section",
                "text": {"type": "mrkdwn", "text": f"*{notification.title}*\n{notification.message}"},
            }
        ]

    logger.debug("Sending Slack notification %s to %s", notification.id, webhook_url)
    with httpx.Client(timeout=10.0) as client:
        response = client.post(webhook_url, json=body)
        if response.status_code >= 400:
            raise ValueError(
                f"Slack webhook responded with {response.status_code}: {response.text[:200]}"
            )


def _deliver_webhook(notification: NotificationOutbox) -> None:
    payload = _extract_payload(notification)
    url = payload.get("url") or payload.get("webhook_url")
    if not url:
        raise ValueError("Generic webhook requires 'url' or 'webhook_url' in payload")

    method = (payload.get("method") or "POST").upper()
    body = payload.get("body") or {}
    headers = payload.get("headers") or {}

    logger.debug(
        "Sending webhook notification %s to %s with method %s", notification.id, url, method
    )

    with httpx.Client(timeout=10.0) as client:
        response = client.request(method, url, json=body, headers=headers)
        if response.status_code >= 400:
            raise ValueError(
                f"Webhook responded with {response.status_code}: {response.text[:200]}"
            )
