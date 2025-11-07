<!-- src/lib/components/sidebar/Sidebar.svelte -->
<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { hasMenuAccess, MENU_IDS } from '$lib/config/roles';
	import {
		Home,
		Search,
		Menu,
		ChevronDown,
		ChevronRight,
		FileText,
		Server,
		Network,
		Shield,
		Globe,
		HardDrive,
		Database,
		Monitor,
		Activity,
		BarChart,
		LogOut,
		Settings,
		User,
		LogIn,
		Plug,
		Share2,
		Bell
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { signOut } from '$lib/api/auth/authApi';
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
import { notificationStore } from '$lib/stores/notifications';
import { onMount, onDestroy } from 'svelte';
import NotificationDiffDialog from '$lib/components/notification/NotificationDiffDialog.svelte';
import type { Notification } from '$lib/api/notification';

	let { sidebarOpen = $bindable(true), isMobile = false } = $props();

	// 현재 경로 확인
	const currentPath = $derived($page.url.pathname);

	// 각 섹션이 활성 상태인지 확인
	const isIpamActive = $derived(currentPath.startsWith('/ipam'));
	const isSoarActive = $derived(currentPath.startsWith('/soar'));
	const isCollaborationActive = $derived(currentPath.startsWith('/collaboration'));

	// 활성 섹션에 따라 자동으로 확장
	let ipamExpanded = $state(true);
	let soarExpanded = $state(false);
	let diskExpanded = $state(false);
let isNotificationOpen = $state(false);
let diffDialogOpen = $state(false);
let diffDialogNotification = $state(null as Notification | null);

	// 경로 변경 시 해당 섹션 자동 확장
	$effect(() => {
		if (isCollaborationActive) {
			diskExpanded = true;
		}
		if (isIpamActive) {
			ipamExpanded = true;
		}
		if (isSoarActive) {
			soarExpanded = true;
		}
	});

	const userInfo = $derived(userStore.user);
	const isAuthenticated = $derived(authStore.isAuthenticated);
	const userRole = $derived(userStore.role);

	// 역할별 메뉴 접근 권한 확인
	const canAccessDashboard = $derived(hasMenuAccess(userRole, MENU_IDS.DASHBOARD));
	const canAccessIpam = $derived(hasMenuAccess(userRole, MENU_IDS.IPAM));
	const canAccessAdminSettings = $derived(hasMenuAccess(userRole, MENU_IDS.ADMIN_SETTINGS));
	const canAccessSettings = $derived(hasMenuAccess(userRole, MENU_IDS.SETTINGS));
	const notificationsStore = notificationStore.notifications;
	const unreadStore = notificationStore.unreadCount;
	const notificationLoadingStore = notificationStore.isLoading;
	const notificationList = $derived($notificationsStore);
	const unreadCount = $derived($unreadStore);
	const notificationsLoading = $derived($notificationLoadingStore);

	onMount(() => {
		if (isAuthenticated) {
			notificationStore.load();
			notificationStore.startPolling();
		}
	});

	onDestroy(() => {
		notificationStore.stopPolling();
	});

	$effect(() => {
		if (isAuthenticated) {
			notificationStore.startPolling();
		} else {
			notificationStore.stopPolling();
		}
	});

	// 활성 메뉴 확인 함수
	function isActive(path: string): boolean {
		return currentPath === path || currentPath.startsWith(path + '/');
	}

	async function handleLogout() {
		try {
			await signOut();
		} catch (error) {
			console.error('Logout API failed:', error);
		} finally {
			// 무조건 세션 정리하고 로그인 페이지로 강제 이동
			authStore.clearToken();
			userStore.clear();
			await invalidateAll();
			// window.location 사용하여 강제 리다이렉션
			window.location.href = '/account/signin';
		}
	}

	function handleNavigation(path: string) {
		goto(path);
		if (isMobile) {
			sidebarOpen = false;
		}
	}

function toggleNotifications() {
	isNotificationOpen = !isNotificationOpen;
	if (isNotificationOpen) {
		notificationStore.load();
	}
}

function openDiff(notification: Notification) {
	diffDialogNotification = notification;
	diffDialogOpen = true;
}

function closeDiff() {
	diffDialogOpen = false;
}
</script>

<!-- Sidebar Overlay (모바일에서만) -->
{#if sidebarOpen && isMobile}
	<button
		type="button"
		class="fixed inset-0 z-40 bg-black/50 md:hidden"
		onclick={() => (sidebarOpen = false)}
		aria-label="Close sidebar"
	></button>
{/if}

<!-- Sidebar -->
<div
	class="fixed top-0 left-0 z-40 h-full transition-transform duration-300 {sidebarOpen
		? 'translate-x-0'
		: '-translate-x-full md:translate-x-0'}"
>
	<div
		class="flex h-full w-60 flex-col border-r border-gray-200 bg-white transition-all duration-300 dark:border-gray-800 dark:bg-gray-900"
	>
		<!-- Logo Header -->
		<div class="flex h-16 items-center border-b border-gray-200 px-4 dark:border-gray-800">
			<div class="flex items-center gap-2">
				<div class="flex h-8 w-8 items-center justify-center rounded bg-orange-500">
					<Shield class="h-5 w-5 text-white" />
				</div>
				<div>
					<div class="text-sm font-semibold text-gray-900 dark:text-white">GuardianX</div>
					<div class="text-xs text-gray-500 dark:text-gray-400">IPAM</div>
				</div>
			</div>
		</div>

		<!-- User Profile Section -->
		{#if isAuthenticated && userInfo}
			<div class="border-b border-gray-200 bg-gray-50 px-4 py-4 dark:border-gray-800 dark:bg-gray-800/50">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-orange-400 to-orange-600 shadow-sm">
						<User class="h-5 w-5 text-white" />
					</div>
					<div class="min-w-0 flex-1 text-left">
						<div class="truncate text-sm font-semibold text-gray-900 dark:text-white">
							{userInfo.handle || userInfo.display_name || '사용자'}
						</div>
						<div class="truncate text-xs text-gray-500 dark:text-gray-400">
							{userInfo.email || 'No email'}
						</div>
					</div>
					<div class="flex items-center gap-2">
						<button
							type="button"
							class="relative rounded-full border border-gray-200 p-2 text-gray-600 hover:bg-white dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700"
							onclick={toggleNotifications}
							aria-label="알림"
						>
							<Bell class="h-4 w-4" />
							{#if unreadCount > 0}
								<span
									class="absolute -right-1 -top-1 flex h-4 min-w-[16px] items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-semibold text-white"
								>
									{unreadCount > 9 ? '9+' : unreadCount}
								</span>
							{/if}
						</button>
						<button
							type="button"
							class="rounded-full border border-gray-200 p-2 text-gray-600 hover:bg-white dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700"
							title="계정 설정으로 이동"
							onclick={() => handleNavigation('/settings#personal')}
						>
							<Settings class="h-4 w-4" />
						</button>
					</div>
				</div>
				{#if isNotificationOpen}
					<div class="mt-3 rounded-lg border border-gray-200 bg-white p-3 text-left shadow-sm dark:border-gray-700 dark:bg-gray-900">
						{#if notificationsLoading}
							<p class="text-sm text-gray-500 dark:text-gray-400">알림을 불러오는 중...</p>
						{:else if notificationList.length === 0}
							<p class="text-sm text-gray-500 dark:text-gray-400">새 알림이 없습니다.</p>
						{:else}
							<ul class="max-h-60 space-y-3 overflow-y-auto pr-1">
									{#each notificationList as notification}
										{@const isUnread = notification.status !== 'done'}
										<li class="rounded border border-gray-100 p-2 text-sm dark:border-gray-700">
											<div
												class={`text-sm ${isUnread ? 'font-semibold text-gray-900 dark:text-gray-100' : 'text-gray-500 dark:text-gray-400'}`}
											>
												{notification.title || '알림'}
											</div>
											{#if notification.message}
												<div class={`text-xs leading-snug ${isUnread ? 'text-gray-600 dark:text-gray-300' : 'text-gray-500 dark:text-gray-500'}`}>
													{notification.message}
												</div>
											{/if}
											{#if notification.payload?.diff?.length}
												<div class="mt-2 space-y-1 rounded bg-gray-50 p-2 text-[11px] dark:bg-gray-800/70">
													{#each notification.payload.diff as change}
														<div class="flex items-start justify-between gap-2">
															<div class="font-medium text-gray-600 dark:text-gray-300">{change.field}</div>
															<div class="text-right text-[10px] text-gray-500 dark:text-gray-400">
																<div class="line-through text-red-500/80 dark:text-red-400/80">
																	{change.before ?? '-'}
																</div>
																<div class="text-green-600 dark:text-green-400">
																	{change.after ?? '-'}
																</div>
															</div>
														</div>
													{/each}
												</div>
											{/if}
											<div class="mt-1 flex items-center gap-2 text-[11px]">
												{#if notification.payload?.actor_name}
													<span class="text-gray-500 dark:text-gray-400">
														{notification.payload?.actor_name}
														{#if notification.payload?.actor_handle}
															<span class="text-gray-400 dark:text-gray-500">@{notification.payload?.actor_handle}</span>
														{/if}
													</span>
												{/if}
												<span
													class={`rounded px-1 py-0.5 text-[10px] ${isUnread ? 'bg-orange-100 text-orange-600 dark:bg-orange-900/40 dark:text-orange-300' : 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'}`}
												>
													{isUnread ? '미확인' : '읽음'}
												</span>
											</div>
											<div class="text-[11px] text-gray-400 dark:text-gray-500">
												{new Date(notification.created_at).toLocaleString('ko-KR')}
											</div>
											<div class="mt-2 flex flex-wrap justify-end gap-1">
												{#if notification.payload?.link}
													<Button
														variant="ghost"
														size="xs"
														class="px-2"
														onclick={() => goto(notification.payload?.link as string)}
													>
														바로가기
													</Button>
												{/if}
												{#if notification.payload?.diff?.length}
													<Button variant="ghost" size="xs" class="px-2" onclick={() => openDiff(notification)}>
														상세
													</Button>
												{/if}
												{#if isUnread}
													<Button
														variant="outline"
														size="xs"
														class="px-2"
														onclick={() => notificationStore.markAsRead(notification.id)}
													>
														확인
													</Button>
												{:else}
													<Button
														variant="ghost"
														size="xs"
														class="px-2"
														onclick={() => notificationStore.markAsRead(notification.id)}
													>
														다시읽기
													</Button>
												{/if}
											</div>
										</li>
									{/each}
								</ul>
								<div class="mt-3 flex justify-between gap-2 text-xs">
									<Button variant="ghost" size="xs" class="flex-1" onclick={() => notificationStore.load()}>
										새로고침
									</Button>
									<Button
										variant="outline"
										size="xs"
										class="flex-1"
										onclick={() => notificationStore.markAllAsRead(notificationList)}
									>
										모두 읽음
									</Button>
								</div>
							{/if}
						</div>
				{/if}
			</div>
		{/if}

		<!-- Domain Selector -->
		<div class="border-b border-gray-200 px-4 py-3 dark:border-gray-800">
			<div class="mb-1 text-xs text-gray-500 dark:text-gray-400">SX</div>
			<button
				class="flex w-full items-center justify-between rounded px-2 py-1 text-sm text-gray-700 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-800"
			>
				<span>domain.com</span>
				<ChevronDown class="h-3 w-3" />
			</button>
		</div>

		<!-- Navigation -->
		<nav class="flex-1 overflow-y-auto">
			<div class="px-3 py-2">
				<!-- 메인 대시보드 -->
				{#if canAccessDashboard}
					<button
						class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors {currentPath === '/'
							? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
							: 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800'}"
						onclick={() => handleNavigation('/')}
					>
						<BarChart class="h-4 w-4 {currentPath === '/' ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
						<span>메인 대시보드</span>
					</button>
				{/if}

				<!-- IPAM 관리 섹션 -->
				{#if canAccessIpam}
					<div class="mt-4">
						<button
							class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm transition-colors {isIpamActive
								? 'bg-orange-100 font-medium text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
								: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
							onclick={() => (ipamExpanded = !ipamExpanded)}
						>
							<div class="flex items-center gap-3">
								<Network class="h-4 w-4 {isIpamActive ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
								<span>IPAM 관리</span>
							</div>
							{#if ipamExpanded}
								<ChevronDown class="h-4 w-4" />
							{:else}
								<ChevronRight class="h-4 w-4" />
							{/if}
						</button>

					{#if ipamExpanded}
						<div class="mt-1 ml-7 space-y-1">
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/offices')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/offices')}
							>
								<FileText class="h-3.5 w-3.5 {isActive('/ipam/offices') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>사무실</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/server-rooms')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/server-rooms')}
							>
								<Server class="h-3.5 w-3.5 {isActive('/ipam/server-rooms') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>서버실</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/racks')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/racks')}
							>
								<Monitor class="h-3.5 w-3.5 {isActive('/ipam/racks') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>랙</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/device')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/device')}
							>
								<HardDrive class="h-3.5 w-3.5 {isActive('/ipam/device') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>디바이스</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/ip-ranges')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/ip-ranges')}
							>
								<Globe class="h-3.5 w-3.5 {isActive('/ipam/ip-ranges') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>IP 대역</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/ip-addresses')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/ip-addresses')}
							>
								<Network class="h-3.5 w-3.5 {isActive('/ipam/ip-addresses') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>IP 주소</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/responsible')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/responsible')}
							>
								<Activity class="h-3.5 w-3.5 {isActive('/ipam/responsible') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>라이브러리</span>
							</button>
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/manager')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/manager')}
							>
								<Database class="h-3.5 w-3.5 {isActive('/ipam/manager') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>담당자</span>
							</button>
							<!-- 데이터 연결 -->
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/data-connections')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/data-connections')}
							>
								<Plug class="h-3.5 w-3.5 {isActive('/ipam/data-connections') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>데이터 연결</span>
							</button>
							<!-- 데이터 관계 -->
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/ipam/relationships')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/ipam/relationships')}
							>
								<Share2 class="h-3.5 w-3.5 {isActive('/ipam/relationships') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>데이터 관계</span>
							</button>
						</div>
					{/if}
				</div>
				{/if}

				<!-- SOAR 보안 -->
				<div class="mt-2">
					<button
						class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
						onclick={() => (soarExpanded = !soarExpanded)}
					>
						<div class="flex items-center gap-3">
							<Shield class="h-4 w-4 text-gray-500 dark:text-gray-400" />
							<span>SOAR 보안</span>
						</div>
						{#if soarExpanded}
							<ChevronDown class="h-4 w-4" />
						{:else}
							<ChevronRight class="h-4 w-4" />
						{/if}
					</button>
				</div>

				<!-- 자산 협업 -->
				<div class="mt-2">
					<button
						class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm transition-colors {isCollaborationActive
							? 'bg-orange-100 font-medium text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
							: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
						onclick={() => (diskExpanded = !diskExpanded)}
					>
						<div class="flex items-center gap-3">
							<HardDrive class="h-4 w-4 {isCollaborationActive ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
							<span>자산 협업</span>
						</div>
						{#if diskExpanded}
							<ChevronDown class="h-4 w-4" />
						{:else}
							<ChevronRight class="h-4 w-4" />
						{/if}
					</button>

					{#if diskExpanded}
						<div class="mt-1 ml-7 space-y-1">
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-1.5 text-sm transition-colors {isActive('/collaboration/custodian')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/collaboration/custodian')}
							>
								<FileText class="h-3.5 w-3.5 {isActive('/collaboration/custodian') ? 'text-orange-600 dark:text-orange-400' : ''}" />
								<span>Custodian</span>
							</button>
						</div>
					{/if}
				</div>

					{#if canAccessAdminSettings}
						<div class="mt-4">
							<button
								class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors {isActive('/admin/settings')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
								: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
							onclick={() => handleNavigation('/admin/settings')}
						>
								<Settings class="h-4 w-4 {isActive('/admin/settings') ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
								<span>관리자 설정</span>
							</button>
							<button
								class="mt-2 flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors {isActive('/notifications')
									? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
									: 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'}"
								onclick={() => handleNavigation('/notifications')}
							>
								<Bell class="h-4 w-4 {isActive('/notifications') ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
								<span>알림 큐</span>
							</button>
						</div>
					{/if}

				<!-- 환경 설정 -->
				{#if canAccessSettings}
					<div class="mt-4">
						<button
							class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors {isActive('/settings')
								? 'bg-orange-100 text-orange-700 dark:bg-orange-950/30 dark:text-orange-400'
								: 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800'}"
							onclick={() => handleNavigation('/settings')}
						>
							<Settings class="h-4 w-4 {isActive('/settings') ? 'text-orange-600 dark:text-orange-400' : 'text-gray-500 dark:text-gray-400'}" />
							<span>환경 설정</span>
						</button>
					</div>
				{/if}
			</div>
		</nav>

		<!-- Bottom Section -->
		<div class="border-t border-gray-200 p-3 dark:border-gray-800">
			{#if isAuthenticated && userInfo}
				<!-- 로그아웃 버튼 -->
				<button
					class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm text-red-600 transition-colors hover:bg-red-50 dark:text-red-400 dark:hover:bg-red-950/20"
					onclick={handleLogout}
				>
					<LogOut class="h-4 w-4" />
					<span>로그아웃</span>
				</button>
			{:else}
				<!-- 로그인 버튼 -->
				<button
					class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm text-blue-600 transition-colors hover:bg-blue-50 dark:text-blue-400 dark:hover:bg-blue-950/20"
					onclick={() => handleNavigation('/account/signin')}
				>
					<LogIn class="h-4 w-4" />
					<span>로그인</span>
				</button>
			{/if}
		</div>
	</div>
</div>

<NotificationDiffDialog open={diffDialogOpen} notification={diffDialogNotification} onClose={closeDiff} />
