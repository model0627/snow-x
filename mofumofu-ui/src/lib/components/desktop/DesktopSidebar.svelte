<!-- src/lib/components/desktop/DesktopSidebar.svelte -->
<script lang="ts">
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import {
		Home, Search, Menu, ChevronDown, ChevronRight,
		FileText, Server, Network, Shield, Globe, HardDrive,
		Database, Monitor, Activity, BarChart, LogOut, Settings,
		User, LogIn
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { signOut } from '$lib/api/auth/authApi';
	import { invalidateAll } from '$app/navigation';

	const { sidebarOpen } = $derived({ sidebarOpen: desktopStore.sidebarOpen });
	let ipamExpanded = $state(true);
	let soarExpanded = $state(false);
	let diskExpanded = $state(false);

	const userInfo = $derived(userStore.user);
	const isAuthenticated = $derived(authStore.isAuthenticated);

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
</script>

{#if desktopStore.isDesktop}
	<!-- Sidebar Overlay (모바일에서만) -->
	{#if sidebarOpen}
		<div 
			class="fixed inset-0 bg-black/50 z-40 md:hidden"
			onclick={() => desktopStore.setSidebarOpen(false)}
		></div>
	{/if}

	<!-- Sidebar -->
	<div class="fixed left-0 top-0 h-full z-40 transition-transform duration-300 {sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'}">
		<div class="h-full w-60 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 transition-all duration-300 flex flex-col">

			<!-- macOS Title Bar Spacer -->
			{#if browser && (window as any).__TAURI__}
				<div class="h-8 bg-transparent"></div>
			{/if}

			<!-- Logo Header -->
			<div class="h-14 px-3 flex items-center border-b border-gray-200 dark:border-gray-800">
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 bg-orange-500 rounded flex items-center justify-center">
						<Shield class="h-3.5 w-3.5 text-white" />
					</div>
					<div>
						<div class="text-xs font-semibold text-gray-900 dark:text-white">GuardianX</div>
						<div class="text-[10px] text-gray-500 dark:text-gray-400">IPAM</div>
					</div>
				</div>
			</div>

			<!-- User Profile Section -->
			{#if isAuthenticated && userInfo}
				<button
					class="w-full px-3 py-3 border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-800/50 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
					onclick={() => goto('/settings#personal')}
					title="계정 설정으로 이동"
				>
					<div class="flex items-center gap-3">
						<div class="w-8 h-8 bg-gradient-to-br from-orange-400 to-orange-600 rounded-full flex items-center justify-center shadow-sm">
							<User class="h-4 w-4 text-white" />
						</div>
						<div class="flex-1 min-w-0 text-left">
							<div class="text-xs font-semibold text-gray-900 dark:text-white truncate">
								{userInfo.handle || userInfo.display_name || '사용자'}
							</div>
							<div class="text-[10px] text-gray-500 dark:text-gray-400 truncate">
								{userInfo.email || 'No email'}
							</div>
						</div>
					</div>
				</button>
			{/if}

			<!-- Domain Selector -->
			<div class="px-3 py-2 border-b border-gray-200 dark:border-gray-800">
				<div class="text-[10px] text-gray-500 dark:text-gray-400 mb-1">SX</div>
				<button class="w-full flex items-center justify-between text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 px-2 py-1 rounded">
					<span>domain.com</span>
					<ChevronDown class="h-2.5 w-2.5" />
				</button>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 overflow-y-auto">
				<div class="px-2 py-1">
					<!-- 메인 대시보드 -->
					<button
						class="w-full flex items-center gap-2 px-2 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
						onclick={() => goto('/')}
					>
						<BarChart class="h-3.5 w-3.5 text-gray-500 dark:text-gray-400" />
						<span>메인 대시보드</span>
					</button>

					<!-- IPAM 관리 섹션 -->
					<div class="mt-3">
						<button
							class="w-full flex items-center justify-between px-2 py-1.5 text-xs font-medium text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-950/20 rounded-md transition-colors"
							onclick={() => ipamExpanded = !ipamExpanded}
						>
							<div class="flex items-center gap-2">
								<Network class="h-3.5 w-3.5" />
								<span>IPAM 관리</span>
							</div>
							{#if ipamExpanded}
								<ChevronDown class="h-3 w-3" />
							{:else}
								<ChevronRight class="h-3 w-3" />
							{/if}
						</button>

						{#if ipamExpanded}
							<div class="ml-5 mt-0.5 space-y-0.5">
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/offices')}
								>
									<FileText class="h-3 w-3" />
									<span>사무실</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/server-rooms')}
								>
									<Server class="h-3 w-3" />
									<span>서버실</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/racks')}
								>
									<Monitor class="h-3 w-3" />
									<span>랙</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/ip-pool')}
								>
									<Globe class="h-3 w-3" />
									<span>IP 대역</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/ip-address')}
								>
									<Network class="h-3 w-3" />
									<span>IP 주소</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/device')}
								>
									<HardDrive class="h-3 w-3" />
									<span>디바이스</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/responsible')}
								>
									<Activity class="h-3 w-3" />
									<span>라이브러리</span>
								</button>
								<button
									class="w-full flex items-center gap-2 px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
									onclick={() => goto('/ipam/manager')}
								>
									<Database class="h-3 w-3" />
									<span>담당자</span>
								</button>
							</div>
						{/if}
					</div>

					<!-- SOAR 보안 -->
					<div class="mt-1">
						<button
							class="w-full flex items-center justify-between px-2 py-1.5 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
							onclick={() => soarExpanded = !soarExpanded}
						>
							<div class="flex items-center gap-2">
								<Shield class="h-3.5 w-3.5 text-gray-500 dark:text-gray-400" />
								<span>SOAR 보안</span>
							</div>
							{#if soarExpanded}
								<ChevronDown class="h-3 w-3" />
							{:else}
								<ChevronRight class="h-3 w-3" />
							{/if}
						</button>
					</div>

					<!-- 자산 협업 -->
					<div class="mt-1">
						<button
							class="w-full flex items-center justify-between px-2 py-1.5 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
							onclick={() => diskExpanded = !diskExpanded}
						>
							<div class="flex items-center gap-2">
								<HardDrive class="h-3.5 w-3.5 text-gray-500 dark:text-gray-400" />
								<span>자산 협업</span>
							</div>
							{#if diskExpanded}
								<ChevronDown class="h-3 w-3" />
							{:else}
								<ChevronRight class="h-3 w-3" />
							{/if}
						</button>
					</div>

					<!-- 환경 설정 -->
					<div class="mt-3">
						<button
							class="w-full flex items-center gap-2 px-2 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
							onclick={() => goto('/settings')}
						>
							<Settings class="h-3.5 w-3.5 text-gray-500 dark:text-gray-400" />
							<span>환경 설정</span>
						</button>
					</div>
				</div>
			</nav>

			<!-- Bottom Section -->
			<div class="border-t border-gray-200 dark:border-gray-800 p-2">
				{#if isAuthenticated && userInfo}
					<!-- 로그아웃 버튼 -->
					<button
						class="w-full flex items-center gap-2 px-2 py-1.5 text-xs text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950/20 rounded-md transition-colors"
						onclick={handleLogout}
					>
						<LogOut class="h-3.5 w-3.5" />
						<span>로그아웃</span>
					</button>
				{:else}
					<!-- 로그인 버튼 -->
					<button
						class="w-full flex items-center gap-2 px-2 py-1.5 text-xs text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-950/20 rounded-md transition-colors"
						onclick={() => goto('/account/signin')}
					>
						<LogIn class="h-3.5 w-3.5" />
						<span>로그인</span>
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}