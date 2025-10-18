import { browser } from '$app/environment';
import { goto } from '$app/navigation';

let focusCheckInterval: ReturnType<typeof setInterval> | null = null;
let lastFocusTime = 0;

/**
 * 알림 클릭 후 앱 포커스 감지를 통한 페이지 이동
 */
export async function initNotificationListener(): Promise<void> {
	if (!browser || !(window as any).__TAURI__) {
		return;
	}

	// 윈도우 포커스 이벤트를 감지하여 알림 클릭 처리
	const handleFocus = () => {
		console.log('👁️ Window focused');
		const now = Date.now();

		// 1초 이내에 여러 번 포커스되는 것 방지
		if (now - lastFocusTime < 1000) {
			console.log('⏭️ Skipping (too soon after last focus)');
			return;
		}
		lastFocusTime = now;

		// localStorage에서 마지막 알림 정보 확인
		try {
			const lastNotification = localStorage.getItem('lastDeviceNotification');
			console.log('📦 Last notification:', lastNotification);

			if (lastNotification) {
				const data = JSON.parse(lastNotification);
				const timeDiff = now - data.timestamp;
				console.log('⏱️ Time difference:', timeDiff, 'ms');

				// 10초 이내의 알림만 처리 (5초에서 10초로 늘림)
				if (timeDiff < 10000) {
					console.log('🔔 Notification click detected, navigating to device page');
					console.log('📱 Device info:', data);
					goto('/ipam/device');

					// 알림 정보 삭제 (한 번만 처리)
					localStorage.removeItem('lastDeviceNotification');
				} else {
					console.log('⏰ Notification too old, ignoring');
				}
			} else {
				console.log('📭 No notification data found');
			}
		} catch (error) {
			console.error('❌ Error handling notification:', error);
		}
	};

	window.addEventListener('focus', handleFocus);
	console.log('✅ Notification listener initialized (focus-based)');
}

/**
 * 알림 리스너를 정리합니다
 */
export function cleanupNotificationListener(): void {
	if (focusCheckInterval) {
		clearInterval(focusCheckInterval);
		focusCheckInterval = null;
	}
	console.log('✅ Notification listener cleaned up');
}
