<!-- src/lib/components/desktop/DesktopSidebar.svelte -->
<script lang="ts">
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { Home, Search, Plus, User, Settings, Menu, Bell } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { signOut } from '$lib/api/auth/authApi';
	import { invalidateAll } from '$app/navigation';
	import * as m from '../../../paraglide/messages';

	const { sidebarOpen } = $derived({ sidebarOpen: desktopStore.sidebarOpen });
	
	const userInfo = $derived(userStore.user);
	const isLoading = $derived(userStore.isLoading);
	const isAuthenticated = $derived(authStore.isAuthenticated);

	async function handleLogout() {
		try {
			await signOut();
			authStore.clearToken();
			userStore.clear();
			await invalidateAll();
			window.location.reload();
		} catch (error) {
			console.error('Logout failed:', error);
			return;
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
		<div class="h-full {sidebarOpen ? 'w-64' : 'w-16'} bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 transition-all duration-300 flex flex-col">
			
			<!-- macOS Title Bar Spacer -->
			{#if browser && (window as any).__TAURI__}
				<div class="h-8 bg-transparent"></div>
			{/if}
			
			<!-- Sidebar Header -->
			<div class="p-4 border-b border-gray-200 dark:border-gray-800">
				<div class="flex items-center justify-between">
					{#if sidebarOpen}
						<h1 class="text-lg font-bold text-gray-900 dark:text-white">Mofumofu</h1>
					{/if}
					<Button
						variant="ghost"
						size="sm"
						onclick={() => desktopStore.toggleSidebar()}
						class="p-2 {sidebarOpen ? '' : 'w-full'}"
					>
						<Menu class="h-4 w-4" />
					</Button>
				</div>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 p-4">
				<div class="space-y-2">
					<!-- Home -->
					<Button
						variant="ghost"
						class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
						onclick={() => goto('/')}
					>
						<Home class="h-5 w-5" />
						{#if sidebarOpen}
							<span class="ml-3">홈</span>
						{/if}
					</Button>

					<!-- Search -->
					<Button
						variant="ghost"
						class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
						onclick={() => goto('/search')}
					>
						<Search class="h-5 w-5" />
						{#if sidebarOpen}
							<span class="ml-3">검색</span>
						{/if}
					</Button>

					{#if isAuthenticated && userInfo}
						<!-- Notifications -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/')}
						>
							<Bell class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">알림</span>
							{/if}
						</Button>

						<!-- Write -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/write')}
						>
							<Plus class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">{m.navbar_new_post()}</span>
							{/if}
						</Button>

						<Separator class="my-4" />

						<!-- Profile -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto(`/@${userInfo.handle}/profile`)}
						>
							<User class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">{m.navbar_my_page()}</span>
							{/if}
						</Button>

						<!-- Drafts -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/drafts')}
						>
							<Plus class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">내 초안</span>
							{/if}
						</Button>

						<!-- Settings -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/settings')}
						>
							<Settings class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">{m.navbar_settings()}</span>
							{/if}
						</Button>

						<!-- Logout -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={handleLogout}
						>
							<User class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">{m.navbar_sign_out()}</span>
							{/if}
						</Button>
					{:else}
						<!-- Login -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/account/signin')}
						>
							<User class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">{m.navbar_sign_in()}</span>
							{/if}
						</Button>

						<!-- Settings for non-authenticated -->
						<Button
							variant="ghost"
							class="w-full justify-start {sidebarOpen ? 'px-4' : 'px-2 justify-center'}"
							onclick={() => goto('/settings')}
						>
							<Settings class="h-5 w-5" />
							{#if sidebarOpen}
								<span class="ml-3">설정</span>
							{/if}
						</Button>
					{/if}
				</div>
			</nav>

			<!-- User Info -->
			{#if authStore.isAuthenticated && sidebarOpen}
				<div class="p-4 border-t border-gray-200 dark:border-gray-800">
					<div class="flex items-center space-x-3">
						<div class="w-8 h-8 bg-gray-300 dark:bg-gray-700 rounded-full flex items-center justify-center">
							{#if userStore.user?.profile_image}
								<img 
									src={userStore.user.profile_image} 
									alt="프로필"
									class="w-8 h-8 rounded-full"
								/>
							{:else}
								<User class="h-4 w-4" />
							{/if}
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-sm font-medium text-gray-900 dark:text-white truncate">
								{userStore.user?.name || '사용자'}
							</p>
							<p class="text-xs text-gray-500 dark:text-gray-400 truncate">
								@{userStore.user?.handle || 'user'}
							</p>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}