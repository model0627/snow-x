import { browser } from '$app/environment';

/**
 * 알림 클릭 이벤트 리스너를 초기화합니다
 * Tauri v2에서는 알림 클릭 시 앱이 포커스되지만, 별도의 클릭 이벤트는 제공하지 않습니다.
 * 대신 앱 내에서 직접 처리하거나, 알림 본문에 정보를 포함시켜야 합니다.
 */
export async function initNotificationListener(): Promise<void> {
	if (!browser || !(window as any).__TAURI__) {
		return;
	}

	// Tauri v2는 기본적으로 알림 클릭 시 앱을 포커스하지만
	// 특정 페이지로 이동하는 기능은 직접 구현해야 합니다
	console.log('Notification listener initialized (Tauri v2)');
}

/**
 * 알림 리스너를 정리합니다
 */
export function cleanupNotificationListener(): void {
	console.log('Notification listener cleaned up');
}
