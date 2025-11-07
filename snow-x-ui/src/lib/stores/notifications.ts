import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import {
	notificationApi,
	type Notification,
	type NotificationListResponse
} from '$lib/api/notification';

function createNotificationStore() {
	const notifications = writable<Notification[]>([]);
	const unreadCount = writable(0);
	const isLoading = writable(false);
	const error = writable<string | null>(null);
	const POLL_INTERVAL = 2000;
	let poller: number | null = null;
	let visibilityListenerAdded = false;
	let isVisible = true;

	async function load(options?: { skipSpinner?: boolean }) {
		if (!options?.skipSpinner) {
			isLoading.set(true);
		}
		error.set(null);
		try {
			const response: NotificationListResponse = await notificationApi.getNotifications({
				page: 1,
				limit: 20,
				channel: 'web'
			});
			notifications.set(response.notifications);
			unreadCount.set(response.notifications.filter((notification) => notification.status !== 'done').length);
		} catch (err) {
			console.error('Failed to load notifications:', err);
			error.set('알림을 불러오지 못했습니다.');
		} finally {
			if (!options?.skipSpinner) {
				isLoading.set(false);
			}
		}
	}

	async function markAsRead(id: string, options?: { skipReload?: boolean }) {
		try {
			await notificationApi.updateNotificationStatus(id, { status: 'done' });
			if (!options?.skipReload) {
				await load();
			}
		} catch (err) {
			console.error('Failed to mark notification as read:', err);
		}
	}

	async function markAllAsRead(current: Notification[]) {
		await Promise.all(current.map((notification) => markAsRead(notification.id, { skipReload: true })));
		await load();
	}

	function tick() {
		if (!isVisible) {
			return;
		}
		load({ skipSpinner: true });
	}

	function handleVisibilityChange() {
		if (!browser) return;
		isVisible = !document.hidden;
		if (isVisible) {
			tick();
		}
	}

	function startPolling() {
		if (!browser || poller) {
			return;
		}
		isVisible = !document.hidden;
		if (!visibilityListenerAdded) {
			document.addEventListener('visibilitychange', handleVisibilityChange);
			visibilityListenerAdded = true;
		}
		poller = window.setInterval(tick, POLL_INTERVAL);
		tick();
	}

	function stopPolling() {
		if (poller) {
			window.clearInterval(poller);
			poller = null;
		}
		if (visibilityListenerAdded && browser) {
			document.removeEventListener('visibilitychange', handleVisibilityChange);
			visibilityListenerAdded = false;
		}
	}

	return {
		notifications,
		unreadCount,
		isLoading,
		error,
		load,
		markAsRead,
		markAllAsRead,
		startPolling,
		stopPolling
	};
}

export const notificationStore = createNotificationStore();
