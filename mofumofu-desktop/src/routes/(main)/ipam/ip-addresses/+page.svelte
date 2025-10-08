<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Search, Plus, Globe } from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import IpAddressBulkDialog from '$lib/components/ipam/IpAddressBulkDialog.svelte';

	interface IpAddressDisplay {
		id: string;
		ip_address: string;
		status: 'allocated' | 'reserved' | 'available' | 'unavailable' | 'expired';
		mac_address?: string;
		ip_range: string;
		description?: string;
		hostname?: string;
	}

	let ipAddresses = $state<IpAddressDisplay[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let showBulkDialog = $state(false);
	let showAddDialog = $state(false);
	let currentPage = $state(1);
	let itemsPerPage = $state(50);
	let totalItems = $state(0);

	// Status counts
	let statusCounts = $state({
		total: 0,
		allocated: 0,
		available: 0,
		unavailable: 0,
		reserved: 0,
		expired: 0
	});

	const isDesktop = $derived(desktopStore.isDesktop);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}
		await loadIpAddresses();
	});

	async function loadIpAddresses() {
		try {
			isLoading = true;
			error = '';

			const { ipAddressApi } = await import('$lib/api/office');
			const response = await ipAddressApi.getIpAddresses({
				page: currentPage,
				limit: itemsPerPage
			});

			totalItems = response.total;

			ipAddresses = response.ip_addresses
				.map((addr) => ({
					id: addr.id,
					ip_address: addr.ip_address || '',
					status: (addr.status as any) || 'available',
					mac_address: addr.mac_address,
					ip_range: '', // TODO: Join with IP range data
					description: addr.description,
					hostname: addr.hostname
				}))
				.sort((a, b) => {
					// IP 주소를 숫자로 변환하여 정렬
					const ipToNumber = (ip: string) => {
						const parts = ip.split('.').map(Number);
						return (parts[0] << 24) + (parts[1] << 16) + (parts[2] << 8) + parts[3];
					};
					return ipToNumber(a.ip_address) - ipToNumber(b.ip_address);
				});

			// Calculate status counts
			statusCounts = {
				total: ipAddresses.length,
				allocated: ipAddresses.filter((a) => a.status === 'allocated').length,
				available: ipAddresses.filter((a) => a.status === 'available').length,
				unavailable: ipAddresses.filter((a) => a.status === 'unavailable').length,
				reserved: ipAddresses.filter((a) => a.status === 'reserved').length,
				expired: ipAddresses.filter((a) => a.status === 'expired').length
			};
		} catch (err) {
			console.error('Failed to load IP addresses:', err);
			error = 'IP 주소 목록을 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
	}

	function handleAddIpAddress() {
		showAddDialog = true;
	}

	function handleBulkAdd() {
		showBulkDialog = true;
	}

	function handleBulkSuccess() {
		loadIpAddresses();
	}

	function handleCloseBulk() {
		showBulkDialog = false;
	}

	function handleCloseAdd() {
		showAddDialog = false;
	}

	// Filter IP addresses based on search query
	const filteredIpAddresses = $derived.by(() => {
		if (!searchQuery.trim()) return ipAddresses;

		const query = searchQuery.toLowerCase();
		return ipAddresses.filter(
			(addr) =>
				addr.ip_address.toLowerCase().includes(query) ||
				addr.mac_address?.toLowerCase().includes(query) ||
				addr.hostname?.toLowerCase().includes(query) ||
				addr.description?.toLowerCase().includes(query)
		);
	});

	function getStatusColor(status: string): string {
		switch (status) {
			case 'allocated':
				return 'text-blue-600 dark:text-blue-400';
			case 'reserved':
				return 'text-green-600 dark:text-green-400';
			case 'available':
				return 'text-gray-600 dark:text-gray-400';
			case 'unavailable':
				return 'text-orange-600 dark:text-orange-400';
			case 'expired':
				return 'text-red-600 dark:text-red-400';
			default:
				return 'text-gray-600 dark:text-gray-400';
		}
	}

	function getStatusBadgeColor(status: string): string {
		switch (status) {
			case 'allocated':
				return 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200';
			case 'reserved':
				return 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200';
			case 'available':
				return 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200';
			case 'unavailable':
				return 'bg-orange-100 dark:bg-orange-900 text-orange-800 dark:text-orange-200';
			case 'expired':
				return 'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200';
			default:
				return 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200';
		}
	}

	function getStatusLabel(status: string): string {
		switch (status) {
			case 'allocated':
				return '할당됨';
			case 'reserved':
				return '예약됨';
			case 'available':
				return '사용가능';
			case 'unavailable':
				return '사용불가';
			case 'expired':
				return '만료됨';
			default:
				return status;
		}
	}

	const totalPages = $derived(Math.ceil(totalItems / itemsPerPage));

	function handlePageChange(page: number) {
		currentPage = page;
		loadIpAddresses();
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="py-6">
				<!-- Title and Add Button -->
				<div class="mb-6 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
							<Globe class="h-6 w-6 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">IP 주소 관리</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">개별 IP 주소를 관리합니다.</p>
						</div>
					</div>
					<div class="flex gap-3">
						<button
							onclick={handleBulkAdd}
							class="flex items-center gap-2 rounded-lg border border-blue-600 px-4 py-2 text-blue-600 transition-colors hover:bg-blue-50 dark:hover:bg-blue-900/20"
						>
							범위로 추가
						</button>
						<button
							onclick={handleAddIpAddress}
							class="flex items-center gap-2 rounded-lg bg-orange-500 px-4 py-2 text-white transition-colors hover:bg-orange-600"
						>
							<Plus class="h-4 w-4" />
							개별 추가
						</button>
					</div>
				</div>

				<!-- Tabs -->
				<div class="flex gap-6 border-b border-gray-200 dark:border-gray-700">
					<button class="border-b-2 border-blue-600 px-1 py-2 font-medium text-blue-600"> 활성 IP 주소 (0) </button>
					<button class="px-1 py-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
						삭제된 IP 주소 (0)
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<!-- Search and Filters -->
		<div class="mb-6 flex items-center gap-4">
			<div class="relative max-w-md flex-1">
				<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
					<Search class="h-4 w-4 text-gray-400" />
				</div>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="IP 주소, MAC 주소, 호스트명으로 검색..."
					class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white
						{isDesktop ? 'text-sm' : 'text-base'}"
				/>
			</div>
			<div class="flex gap-2">
				<select
					class="rounded-lg border border-gray-300 bg-white px-3 py-2.5 text-gray-900 dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop
						? 'text-sm'
						: 'text-base'}"
				>
					<option>모든 IP 대역</option>
				</select>
				<select
					class="rounded-lg border border-gray-300 bg-white px-3 py-2.5 text-gray-900 dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop
						? 'text-sm'
						: 'text-base'}"
				>
					<option>모든 상태</option>
				</select>
			</div>
		</div>

		<!-- Status Counts -->
		<div class="mb-6 flex items-center gap-6 text-sm">
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-gray-400"></div>
				<span class="text-gray-600 dark:text-gray-400">0개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-blue-500"></div>
				<span class="text-gray-600 dark:text-gray-400">할당됨 0개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-green-500"></div>
				<span class="text-gray-600 dark:text-gray-400">사용가능 0개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-orange-500"></div>
				<span class="text-gray-600 dark:text-gray-400">사용불가 0개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-gray-500"></div>
				<span class="text-gray-600 dark:text-gray-400">비활성 0개</span>
			</div>
		</div>

		<!-- Table -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<div
							class="mx-auto h-8 w-8 animate-spin rounded-full border-b-2 border-blue-600 dark:border-blue-400"
						></div>
						<p class="mt-2 text-gray-500 dark:text-gray-400 {isDesktop ? 'text-sm' : 'text-base'}">로딩 중...</p>
					</div>
				</div>
			{:else if error}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<p class="mb-4 text-red-600 dark:text-red-400">{error}</p>
						<button
							onclick={() => loadIpAddresses()}
							class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
						>
							다시 시도
						</button>
					</div>
				</div>
			{:else if filteredIpAddresses.length === 0}
				<div class="flex items-center justify-center py-20">
					<div class="text-center">
						<Globe class="mx-auto mb-4 h-16 w-16 text-gray-300 dark:text-gray-600" />
						<p class="mb-2 text-lg font-medium text-gray-900 dark:text-gray-100">등록된 IP 주소가 없습니다</p>
						<p class="mb-6 text-sm text-gray-500 dark:text-gray-400">첫 번째 IP 주소를 추가해보세요.</p>
						<button
							onclick={handleAddIpAddress}
							class="rounded-lg bg-orange-500 px-6 py-3 font-medium text-white transition-colors hover:bg-orange-600"
						>
							IP 주소 추가
						</button>
					</div>
				</div>
			{:else}
				<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
					<thead class="bg-gray-50 dark:bg-gray-700">
						<tr>
							<th class="w-8 px-6 py-3">
								<input type="checkbox" class="rounded border-gray-300 dark:border-gray-600" />
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								IP 주소
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								상태
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								MAC 주소
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								IP 대역
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								설명
							</th>
							<th
								class="px-6 py-3 text-right text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
							>
								작업
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
						{#each filteredIpAddresses as addr (addr.id)}
							<tr class="transition-colors hover:bg-gray-50 dark:hover:bg-gray-700">
								<td class="px-6 py-4">
									<input type="checkbox" class="rounded border-gray-300 dark:border-gray-600" />
								</td>
								<td class="px-6 py-4 whitespace-nowrap">
									<div class="text-sm font-medium text-gray-900 dark:text-gray-100">
										{addr.ip_address}
									</div>
									{#if addr.hostname}
										<div class="text-xs text-gray-500 dark:text-gray-400">
											{addr.hostname}
										</div>
									{/if}
								</td>
								<td class="px-6 py-4 whitespace-nowrap">
									<span class="rounded-full px-2 py-1 text-xs font-medium {getStatusBadgeColor(addr.status)}">
										{getStatusLabel(addr.status)}
									</span>
								</td>
								<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
									{addr.mac_address || '-'}
								</td>
								<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
									{addr.ip_range}
								</td>
								<td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
									{addr.description || '-'}
								</td>
								<td class="px-6 py-4 text-right text-sm font-medium whitespace-nowrap">
									<button class="text-gray-600 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400">
										편집
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="mt-6 flex items-center justify-between">
				<div class="text-sm text-gray-700 dark:text-gray-300">
					총 {totalItems}개 중 {(currentPage - 1) * itemsPerPage + 1} - {Math.min(
						currentPage * itemsPerPage,
						totalItems
					)}개 표시
				</div>
				<div class="flex gap-2">
					<button
						onclick={() => handlePageChange(currentPage - 1)}
						disabled={currentPage === 1}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						이전
					</button>

					{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
						{#if page === 1 || page === totalPages || (page >= currentPage - 2 && page <= currentPage + 2)}
							<button
								onclick={() => handlePageChange(page)}
								class="rounded-lg border px-3 py-2 {page === currentPage
									? 'border-blue-600 bg-blue-600 text-white'
									: 'border-gray-300 bg-white text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'}"
							>
								{page}
							</button>
						{:else if page === currentPage - 3 || page === currentPage + 3}
							<span class="px-3 py-2">...</span>
						{/if}
					{/each}

					<button
						onclick={() => handlePageChange(currentPage + 1)}
						disabled={currentPage === totalPages}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						다음
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Bulk Add Dialog -->
<IpAddressBulkDialog open={showBulkDialog} onClose={handleCloseBulk} onSuccess={handleBulkSuccess} />
