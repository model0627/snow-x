<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import { useNavbarScroll } from '$lib/hooks/ui/useNavbarScroll.svelte';
	import { setContext } from 'svelte';

	let { children } = $props();

	const { isVisible, isAtTop } = useNavbarScroll({
		navbarHeight: 60,
		scrollThreshold: 10
	});

	setContext('navbar', { isVisible, isAtTop });
</script>

<div class="flex min-h-screen">
	<div class="flex-1">
		{@render children()}
	</div>
</div>
