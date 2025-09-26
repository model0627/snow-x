<!-- src/lib/components/Navbar.svelte -->
<script lang="ts">
	import { Cog6Tooth, Icon } from 'svelte-hero-icons';
	import {
		Rss,
		Bell,
		MagnifyingGlass,
		ChevronDown,
		User,
		ArrowRightOnRectangle,
		DocumentText
	} from 'svelte-hero-icons';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { signOut } from '$lib/api/auth/authApi';
	import { fly, scale } from 'svelte/transition';
	import { Button } from '../ui/button';
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/state';
	import { postsStore } from '$lib/stores/posts.svelte';
	import * as m from '../../../paraglide/messages';
	import { cn } from '$lib/utils';

	let { isVisible, isAtTop } = $props();

	// 현재 경로에 따른 활성 상태 확인

	// 네비게이션 클릭 시 postsStore 리셋 (다른 페이지로 이동할 때만)
	const handleNavClick = (targetPath: string) => {
		if (page.url.pathname !== targetPath) {
			postsStore.reset();
		}
	};

	let isDropdownOpen = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	const userInfo = $derived(userStore.user);
	const isLoading = $derived(userStore.isLoading);
	const isAuthenticated = $derived(authStore.isAuthenticated);

	onMount(async () => {
		// 초기 로드시 항상 refresh 시도하여 토큰 상태 확인
		// refresh token이 있으면 새 access token을 받고, 없으면 로그아웃 상태 유지
		const refreshSuccess = await authStore.tryRefreshToken();

		// refresh 성공하거나 이미 유효한 토큰이 있으면 프로필 로드
		if ((refreshSuccess || authStore.isAuthenticated) && !userStore.user) {
			await userStore.loadProfile();
		}
	});

	async function handleLogout() {
		try {
			await signOut();
		} catch (error) {
			console.error('Logout API failed:', error);
		} finally {
			// 무조건 세션 정리하고 로그인 페이지로 강제 이동
			authStore.clearToken();
			userStore.clear();
			isDropdownOpen = false;
			await invalidateAll();
			// window.location 사용하여 강제 리다이렉션
			window.location.href = '/account/signin';
		}
	}

	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}
</script>

