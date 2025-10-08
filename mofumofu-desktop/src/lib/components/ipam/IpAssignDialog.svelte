<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { Network, Search } from '@lucide/svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { ipAddressApi, deviceApi, type IpAddress } from '$lib/api/office';

	interface Props {
		open: boolean;
		deviceId: string;
		deviceName: string;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, deviceId, deviceName, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let ipAddresses = $state<IpAddress[]>([]);
	let assignedIps = $state<IpAddress[]>([]);
	let selectedIp = $state<IpAddress | null>(null);
	let activeTab = $state<'assign' | 'manage'>('assign');
	let searchQuery = $state('');
	let assignSearchQuery = $state('');

	const isDesktop = $derived(desktopStore.isDesktop);

	// Load data when dialog opens
	$effect(() => {
		if (open) {
			loadAvailableIpAddresses();
			loadAssignedIpAddresses();
			selectedIp = null;
			activeTab = 'assign';
			searchQuery = '';
			assignSearchQuery = '';
		}
	});

	const filteredAvailableIps = $derived.by(() => {
		if (!assignSearchQuery.trim()) return ipAddresses;
		const query = assignSearchQuery.toLowerCase();
		return ipAddresses.filter(ip =>
			ip.ip_address?.toLowerCase().includes(query) ||
			ip.description?.toLowerCase().includes(query)
		);
	});

	async function loadAvailableIpAddresses() {
		try {
			const response = await ipAddressApi.getIpAddresses({
				status: 'available',
				page: 1,
				limit: 100
			});
			ipAddresses = response.ip_addresses;
		} catch (error) {
			console.error('Failed to load IP addresses:', error);
		}
	}

	async function loadAssignedIpAddresses() {
		try {
			assignedIps = await deviceApi.getAssignedIpAddresses(deviceId);
		} catch (error) {
			console.error('Failed to load assigned IP addresses:', error);
			assignedIps = [];
		}
	}

	const filteredAssignedIps = $derived.by(() => {
		if (!searchQuery.trim()) return assignedIps;
		const query = searchQuery.toLowerCase();
		return assignedIps.filter(ip =>
			ip.ip_address?.toLowerCase().includes(query) ||
			ip.description?.toLowerCase().includes(query)
		);
	});

	async function handleAssignIp(ip: IpAddress) {
		loading = true;
		try {
			await deviceApi.assignIpAddress(deviceId, ip.id);
			await loadAssignedIpAddresses();
			await loadAvailableIpAddresses();
			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to assign IP:', error);
			alert('IP 할당에 실패했습니다.');
		} finally {
			loading = false;
		}
	}

	async function handleUnassignIp(ip: IpAddress) {
		if (!confirm(`${ip.ip_address} IP 할당을 해제하시겠습니까?`)) {
			return;
		}

		loading = true;
		try {
			await deviceApi.unassignIpAddress(deviceId, ip.id);
			await loadAssignedIpAddresses();
			await loadAvailableIpAddresses();
			onSuccess();
		} catch (error) {
			console.error('Failed to unassign IP:', error);
			alert('IP 할당 해제에 실패했습니다.');
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="max-h-[90vh] w-full max-w-4xl overflow-y-auto border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader
			class="sticky top-0 z-10 bg-white pb-4 dark:bg-gray-800"
		>
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
					<Network class="h-5 w-5 text-blue-600 dark:text-blue-400" />
				</div>
				<div>
					<DialogTitle class="{isDesktop ? 'text-lg' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
						IP 주소 관리
					</DialogTitle>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{deviceName}</p>
				</div>
			</div>
		</DialogHeader>

		<!-- Tabs -->
		<div class="border-b border-gray-200 dark:border-gray-700">
			<div class="flex gap-6">
				<button
					onclick={() => activeTab = 'assign'}
					class="border-b-2 pb-3 text-sm font-medium transition-colors {activeTab === 'assign'
						? 'border-blue-600 text-blue-600 dark:border-blue-400 dark:text-blue-400'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					IP 할당
				</button>
				<button
					onclick={() => activeTab = 'manage'}
					class="border-b-2 pb-3 text-sm font-medium transition-colors {activeTab === 'manage'
						? 'border-blue-600 text-blue-600 dark:border-blue-400 dark:text-blue-400'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					할당된 IP 관리 ({assignedIps.length})
				</button>
			</div>
		</div>

		<!-- Tab Content -->
		<div class="pt-4">
			{#if activeTab === 'assign'}
				<!-- IP 할당 탭 -->
				<div class="space-y-4">
					<!-- 검색창 -->
					<div class="relative">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<Search class="h-4 w-4 text-gray-400" />
						</div>
						<input
							type="text"
							bind:value={assignSearchQuery}
							placeholder="IP 주소로 검색..."
							class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10
								text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
								focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white
								{isDesktop ? 'text-sm' : 'text-base'}"
						/>
					</div>

					{#if filteredAvailableIps.length === 0}
						<div class="flex flex-col items-center justify-center py-12">
							<Network class="mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
							<p class="mb-2 text-sm font-medium text-gray-900 dark:text-gray-100">
								사용 가능한 IP 주소가 없습니다
							</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								사용 가능한 IP 주소를 찾을 수 없습니다.
							</p>
						</div>
					{:else}
						<!-- IP 목록 -->
						<div class="max-h-96 space-y-2 overflow-y-auto">
							{#each filteredAvailableIps as ip (ip.id)}
								<button
									onclick={() => handleAssignIp(ip)}
									disabled={loading}
									class="w-full rounded-lg border border-gray-200 p-3 text-left transition-all hover:border-blue-500 hover:bg-blue-50 dark:border-gray-700 dark:hover:border-blue-400 dark:hover:bg-blue-900/20 disabled:opacity-50"
								>
									<div class="flex items-center justify-between">
										<div>
											<p class="font-medium text-gray-900 dark:text-gray-100">{ip.ip_address}</p>
											{#if ip.description}
												<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{ip.description}</p>
											{/if}
										</div>
										<div class="text-sm text-blue-600 dark:text-blue-400">
											선택
										</div>
									</div>
								</button>
							{/each}
						</div>
					{/if}

					<!-- 닫기 버튼 -->
					<div class="flex justify-end border-t border-gray-200 pt-6 dark:border-gray-700">
						<Button
							type="button"
							variant="outline"
							onclick={onClose}
							disabled={loading}
							class="{isDesktop ? 'px-4 py-2 text-sm' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
						>
							닫기
						</Button>
					</div>
				</div>
			{:else}
				<!-- 할당된 IP 관리 탭 -->
				<div class="space-y-4">
					<!-- 검색창 -->
					<div class="relative">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<Search class="h-4 w-4 text-gray-400" />
						</div>
						<input
							type="text"
							bind:value={searchQuery}
							placeholder="IP 주소로 검색..."
							class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10
								text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
								focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white
								{isDesktop ? 'text-sm' : 'text-base'}"
						/>
					</div>

					{#if filteredAssignedIps.length === 0}
						<div class="flex flex-col items-center justify-center py-12">
							<Network class="mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
							<p class="mb-2 text-sm font-medium text-gray-900 dark:text-gray-100">
								사용 가능한 IP 주소가 없습니다
							</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								사용 가능한 IP 주소를 찾을 수 없습니다.
							</p>
						</div>
					{:else}
						<!-- IP 목록 -->
						<div class="max-h-96 space-y-2 overflow-y-auto">
							{#each filteredAssignedIps as ip (ip.id)}
								<div class="rounded-lg border border-gray-200 p-3 dark:border-gray-700">
									<div class="flex items-center justify-between">
										<div>
											<p class="font-medium text-gray-900 dark:text-gray-100">{ip.ip_address}</p>
											{#if ip.description}
												<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{ip.description}</p>
											{/if}
										</div>
										<button
											onclick={() => handleUnassignIp(ip)}
											disabled={loading}
											class="text-sm text-red-600 transition-colors hover:text-red-700 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
										>
											할당 해제
										</button>
									</div>
								</div>
							{/each}
						</div>
					{/if}

					<!-- 닫기 버튼 -->
					<div class="flex justify-end border-t border-gray-200 pt-6 dark:border-gray-700">
						<Button
							type="button"
							variant="outline"
							onclick={onClose}
							class="{isDesktop ? 'px-4 py-2 text-sm' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
						>
							닫기
						</Button>
					</div>
				</div>
			{/if}
		</div>
	</DialogContent>
</Dialog>
