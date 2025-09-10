<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import DesktopSidebar from '$lib/components/desktop/DesktopSidebar.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';

	let { children } = $props();
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

<div class="dark:bg-mofu-dark-900 font-pretendard bg-mofu-light-900 min-h-screen max-w-screen flex">
	<!-- Desktop Sidebar -->
	<DesktopSidebar />
	
	<!-- Main Content -->
	<main class="flex-1 {desktopStore.isDesktop ? (desktopStore.sidebarOpen ? 'ml-64' : 'ml-16') : ''} transition-all duration-300">
		{@render children()}
	</main>
</div>
