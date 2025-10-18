<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import DesktopSidebar from '$lib/components/desktop/DesktopSidebar.svelte';
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { initNotificationListener, cleanupNotificationListener } from '$lib/stores/notification.svelte';

	let { children } = $props();

	// 사이드바 상태
	let sidebarOpen = $state(true);
	let isMobile = $state(false);

	// 전역 클립보드 모니터링을 위한 변수들
	let isClipboardSupported = $state(false);
	let lastClipboardContent = $state('');
	let clipboardCheckInterval: ReturnType<typeof setInterval> | null = null;
	let notificationPermission = $state('default');
	let isTauri = $state(false);
	let clipboardItems = $state([]);

	// 인증 초기화 상태 추가
	let authInitialized = $state(false);
	let authChecking = $state(true);

	// 인증이 필요하지 않은 페이지들 (보다 정확한 매칭)
	const publicRoutes = [
		'/account/signin',
		'/account/signup',
		'/account/forgot-password',
		'/account/verify-email',
		'/account/oauth/callback',
		'/account/oauth/link',
		'/account/set-handle'
	];

	// 현재 페이지가 공개 페이지인지 확인
	const isPublicRoute = $derived(publicRoutes.some((route) => $page.url.pathname.startsWith(route)));

	// 사이드바를 숨겨야 하는 페이지들 (로그인 관련 페이지)
	const shouldHideSidebar = $derived(isPublicRoute);

	// 인증 상태 감시 및 리다이렉트 (초기화 완료 후에만)
	$effect(() => {
		if (browser && !isPublicRoute && authInitialized) {
			const isAuthenticated = authStore.isAuthenticated;
			const hasUser = userStore.user;
			const isUserLoading = userStore.isLoading;

			// 사용자 정보 로딩 중이면 대기
			if (isUserLoading) {
				return;
			}

			// 인증되지 않았거나 사용자 정보가 없으면 로그인 페이지로 리다이렉트
			if (!isAuthenticated || !hasUser) {
				console.log('🔒 User not authenticated or no user info, redirecting to signin');
				console.log('Auth status:', { isAuthenticated, hasUser, authInitialized });
				window.location.href = '/account/signin';
			}
		}
	});

	onMount(async () => {
		console.log('🚀 Global app mounted - initializing authentication');
		console.log('Current path:', $page.url.pathname);
		console.log('Is public route:', isPublicRoute);

		// 인증 상태 초기화 (공개 페이지가 아닌 경우에만)
		if (browser && !isPublicRoute) {
			console.log('🔐 Initializing authentication for protected route...');

			try {
				// 토큰이 있는지 확인
				if (authStore.isAuthenticated) {
					console.log('📝 Access token found, loading user profile...');

					// 사용자 정보가 없으면 프로필 로드 시도
					if (!userStore.user && !userStore.isLoading) {
						await userStore.loadProfile();
					}

					// 사용자 정보 로드에 실패하면 토큰 갱신 시도
					if (!userStore.user) {
						console.log('🔄 User profile load failed, trying to refresh token...');
						const refreshSuccess = await authStore.tryRefreshToken();

						if (refreshSuccess && !userStore.user) {
							await userStore.loadProfile();
						}
					}
				} else {
					console.log('🔄 No access token, trying to refresh...');
					const refreshSuccess = await authStore.tryRefreshToken();

					if (refreshSuccess) {
						// 토큰 갱신 성공 후 사용자 정보 로드
						if (!userStore.user && !userStore.isLoading) {
							await userStore.loadProfile();
						}
					}
				}

				console.log('✅ Authentication initialization complete');
				console.log('Final auth state:', {
					isAuthenticated: authStore.isAuthenticated,
					hasUser: !!userStore.user,
					userEmail: userStore.user?.email
				});
			} catch (error) {
				console.error('❌ Authentication initialization error:', error);
			}
		} else if (isPublicRoute) {
			console.log('🔓 Public route, skipping authentication');
		}

		// 인증 초기화 완료 표시
		authInitialized = true;
		authChecking = false;

		// 모바일 감지 및 반응형 처리
		if (browser) {
			const checkMobile = () => {
				isMobile = window.innerWidth < 768;
				if (isMobile) {
					sidebarOpen = false;
				}
			};

			checkMobile();
			window.addEventListener('resize', checkMobile);
		}

		// Tauri 환경 감지
		if (browser && (window as any).__TAURI__) {
			isTauri = true;
			console.log('🖥️ Running in Tauri desktop app');
			isClipboardSupported = true; // Tauri에서는 항상 지원됨

			// 알림 클릭 리스너 초기화
			await initNotificationListener();

			// 알림 권한 확인 및 요청
			try {
				const { isPermissionGranted, requestPermission } = await import('@tauri-apps/plugin-notification');

				console.log('🔔 Checking Tauri notification permissions...');
				let permissionGranted = await isPermissionGranted();
				console.log('🔔 Tauri notification permission granted:', permissionGranted);

				if (!permissionGranted) {
					console.log('🔔 Requesting Tauri notification permission...');
					const permission = await requestPermission();
					permissionGranted = permission === 'granted';
					console.log('🔔 Tauri permission request result:', permission);
				}

				notificationPermission = permissionGranted ? 'granted' : 'denied';

				// 클립보드 모니터링 시작
				if (isClipboardSupported && notificationPermission === 'granted') {
					console.log('🚀 Starting global clipboard monitoring');
					startGlobalClipboardMonitoring();
				}
			} catch (error) {
				console.error('❌ Tauri notification setup failed:', error);
			}

			// 컴포넌트 언마운트 시 리스너 해제
			return () => {
				cleanupNotificationListener();
				if (clipboardCheckInterval) {
					clearInterval(clipboardCheckInterval);
				}
				if (browser) {
					window.removeEventListener('resize', checkMobile);
				}
			};
		} else {
			console.log('🌐 Running in web browser - clipboard monitoring disabled');
		}
	});

	async function startGlobalClipboardMonitoring() {
		if (!isClipboardSupported || !isTauri) return;

		console.log('📋 Initializing global clipboard monitoring...');

		// 초기 클립보드 내용 저장
		try {
			lastClipboardContent = await readClipboard();
			console.log('📋 Initial clipboard content set');
		} catch (error) {
			console.warn('❌ Initial clipboard read failed:', error);
			return;
		}

		// 1초마다 클립보드 내용 확인
		clipboardCheckInterval = setInterval(async () => {
			try {
				const currentContent = await readClipboard();

				// 클립보드 내용이 변경되었고 비어있지 않은 경우
				if (currentContent && currentContent !== lastClipboardContent && currentContent.trim()) {
					console.log('✅ Global clipboard change detected');
					lastClipboardContent = currentContent;
					await handleGlobalClipboardChange(currentContent);
				}
			} catch (error) {
				// 클립보드 읽기 실패 시 무시
			}
		}, 1000);

		console.log('✅ Global clipboard monitoring started');
	}

	async function readClipboard(): Promise<string> {
		if (isTauri) {
			try {
				if (typeof globalThis !== 'undefined' && (globalThis as any).__TAURI_INTERNALS__) {
					const { invoke } = (globalThis as any).__TAURI_INTERNALS__;
					const result = await invoke('plugin:clipboard-manager|read_text');
					return result || '';
				}
				return await navigator.clipboard.readText();
			} catch (error) {
				try {
					return await navigator.clipboard.readText();
				} catch (webError) {
					return '';
				}
			}
		}
		return '';
	}

	async function handleGlobalClipboardChange(content: string) {
		console.log('🔍 Processing global clipboard change:', `"${content.substring(0, 50)}..."`);

		// localStorage에 클립보드 아이템 저장
		const newItem = {
			id: Date.now(),
			type: detectContentType(content),
			content: content,
			title: generateAutoTitle(content),
			timestamp: new Date().toLocaleString('ko-KR', {
				year: 'numeric',
				month: '2-digit',
				day: '2-digit',
				hour: '2-digit',
				minute: '2-digit'
			}),
			tags: ['자동추가']
		};

		// localStorage에서 기존 아이템 가져오기
		try {
			const existingItems = JSON.parse(localStorage.getItem('clipboardItems') || '[]');

			// 중복 확인
			const isDuplicate = existingItems.slice(0, 5).some((item: any) => item.content.trim() === content.trim());

			if (!isDuplicate) {
				const updatedItems = [newItem, ...existingItems];
				localStorage.setItem('clipboardItems', JSON.stringify(updatedItems));
				console.log('✅ Saved to localStorage');

				// 알림 표시
				await showGlobalNotification(
					'클립보드에 새 항목이 추가되었습니다',
					content.length > 100 ? content.substring(0, 97) + '...' : content
				);
			}
		} catch (error) {
			console.error('❌ LocalStorage save failed:', error);
		}
	}

	async function showGlobalNotification(title: string, body: string): Promise<void> {
		if (!isTauri || notificationPermission !== 'granted') return;

		try {
			console.log('🔔 Showing global notification');
			const { sendNotification } = await import('@tauri-apps/plugin-notification');

			await sendNotification({
				title,
				body,
				icon: null,
				sound: 'default'
			});
			console.log('✅ Global notification sent');
		} catch (error) {
			console.error('❌ Global notification failed:', error);
		}
	}

	function detectContentType(content: string): string {
		const urlPattern = /^https?:\/\/.+/i;
		if (urlPattern.test(content.trim())) {
			return 'url';
		}

		const codePatterns = [
			/function\s+\w+\s*\(/,
			/const\s+\w+\s*=/,
			/let\s+\w+\s*=/,
			/var\s+\w+\s*=/,
			/class\s+\w+/,
			/import\s+.+from/,
			/export\s+(default\s+)?/,
			/console\.(log|error|warn)/,
			/\$\(.*\)/,
			/\{\s*\n.*\n\s*\}/s
		];

		if (codePatterns.some((pattern) => pattern.test(content))) {
			return 'code';
		}

		return 'text';
	}

	function generateAutoTitle(content: string): string {
		const trimmed = content.trim();

		if (trimmed.startsWith('http')) {
			try {
				const url = new URL(trimmed);
				return url.hostname;
			} catch {
				return 'URL';
			}
		}

		const firstLine = trimmed.split('\n')[0];
		return firstLine.length > 50 ? firstLine.substring(0, 47) + '...' : firstLine || '제목 없음';
	}
</script>

<svelte:head>
	<!-- 기본값들은 각 페이지에서 설정하지 않을 때만 사용됨 -->
	<meta name="keywords" content="blogging, writing, essays, tutorials, minimalist, open-source, blog platform" />
	<meta name="robots" content="index, follow" />
	<meta name="author" content="Mofumofu Team" />
	<meta property="og:site_name" content="Mofumofu" />
</svelte:head>

<ModeWatcher defaultMode="dark" />
<Toaster />

{#if authChecking && !isPublicRoute}
	<!-- 인증 체크 중 로딩 표시 -->
	<div class="flex min-h-screen items-center justify-center bg-gray-50 dark:bg-gray-900">
		<div class="text-center">
			<div class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-b-2 border-orange-500"></div>
			<p class="text-gray-600 dark:text-gray-400">인증 확인 중...</p>
		</div>
	</div>
{:else}
	<div
		class="dark:bg-mofu-dark-900 font-pretendard bg-mofu-light-900 flex min-h-screen max-w-screen {desktopStore.isDesktop
			? 'text-sm'
			: ''}"
	>
		<!-- Desktop Sidebar (Tauri Only) -->
		{#if !shouldHideSidebar}
			{#if desktopStore.isDesktop}
				<DesktopSidebar />
			{:else}
				<!-- Web Sidebar -->
				<Sidebar bind:sidebarOpen {isMobile} />
			{/if}
		{/if}

		<!-- Main Content -->
		<main
			class="flex-1 {shouldHideSidebar
				? ''
				: desktopStore.isDesktop
					? 'ml-60'
					: sidebarOpen && !isMobile
						? 'ml-60'
						: !isMobile
							? 'ml-60'
							: ''} transition-all duration-300"
		>
			{@render children()}
		</main>
	</div>
{/if}
