import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';

/**
 * 테스트 알림을 보냅니다
 */
export async function sendTestNotification(): Promise<void> {
	try {
		console.log('🧪 Testing notification...');

		let permissionGranted = await isPermissionGranted();
		console.log('Permission status:', permissionGranted);

		if (!permissionGranted) {
			const permission = await requestPermission();
			permissionGranted = permission === 'granted';
			console.log('Permission request result:', permission);
		}

		if (!permissionGranted) {
			console.error('Notification permission denied');
			return;
		}

		await sendNotification({
			title: '테스트 알림',
			body: '이것은 테스트 알림입니다.',
		});

		console.log('✅ Test notification sent');
	} catch (error) {
		console.error('❌ Test notification failed:', error);
	}
}

// 전역 함수로 등록하여 콘솔에서 호출 가능하게 함
if (typeof window !== 'undefined') {
	(window as any).sendTestNotification = sendTestNotification;
}
