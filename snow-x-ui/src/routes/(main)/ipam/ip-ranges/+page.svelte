<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Search, Plus, Network, Eye, Edit, Trash2, Server, Wifi, Shield, Activity } from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import IpRangeFormDialog from '$lib/components/ipam/IpRangeFormDialog.svelte';
	import { ipRangeApi, type IpRange } from '$lib/api/office';

	interface IpRangeDisplay {
		id: string;
		network: string;
		name: string;
		description: string;
		gateway: string;
		vlan?: string;
	usage_percentage: number;
	total_ips: number;
	used_ips: number;
	available_ips: number;
	version: 'IPv4' | 'IPv6';
	created_at: string;
	updated_at: string;
	breakdown: {
		allocated: number;
		reserved: number;
		unavailable: number;
		expired: number;
		other: number;
	};
	breakdownText: string;
}

	interface IpRangeStats {
		total_ranges: number;
		total_ips: number;
		used_ips: number;
		available_ips: number;
	}

	let ipRanges = $state<IpRangeDisplay[]>([]);
	let stats = $state<IpRangeStats>({
		total_ranges: 0,
		total_ips: 0,
		used_ips: 0,
		available_ips: 0
	});
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let showAddDialog = $state(false);
	let editingIpRange = $state<IpRange | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}
	await loadIpRanges();
});

function formatUsagePercentage(value: number): string {
	const rounded = Math.round((value ?? 0) * 10) / 10;
	if (Number.isNaN(rounded)) return '0';
	return Number.isInteger(rounded) ? rounded.toFixed(0) : rounded.toFixed(1);
}

function createBreakdownText(counts: { allocated: number; reserved: number; unavailable: number; expired: number; other: number }): string {
	const parts: string[] = [];
	if (counts.allocated) parts.push(`할당 ${counts.allocated.toLocaleString()}`);
	if (counts.reserved) parts.push(`예약 ${counts.reserved.toLocaleString()}`);
	if (counts.unavailable) parts.push(`비가용 ${counts.unavailable.toLocaleString()}`);
	if (counts.expired) parts.push(`만료 ${counts.expired.toLocaleString()}`);
	if (counts.other) parts.push(`기타 ${counts.other.toLocaleString()}`);
	return parts.length ? parts.join(' · ') : '세부 현황 없음';
}

function getUsageBarClass(percentage: number): string {
	if (percentage >= 90) return 'bg-red-500 dark:bg-red-400';
	if (percentage >= 70) return 'bg-yellow-500 dark:bg-yellow-400';
	return 'bg-green-500 dark:bg-green-400';
}

function clampUsage(value: number | undefined): number {
	if (value === undefined || Number.isNaN(value)) return 0;
	return Math.min(100, Math.max(0, value));
}

async function loadIpRanges() {
	try {
		isLoading = true;
		error = '';

		console.log('Loading IP ranges...');
		const response = await ipRangeApi.getIpRanges({ page: 1, limit: 100 });

		const transformedRanges = response.ip_ranges.map((range) => {
			let total = range.total_ips ?? 0;
			let used = range.used_ips ?? 0;
			let available = range.available_ips ?? 0;

			if (total === 0) {
				total = used + available;
			}

			const clampedUsed = total > 0 ? Math.min(Math.max(used, 0), total) : Math.max(used, 0);
			const clampedAvailable = total > 0
				? Math.min(Math.max(available, 0), total - clampedUsed)
				: Math.max(available, 0);
			const rawPercentage = range.usage_percentage ?? (total > 0 ? (clampedUsed / total) * 100 : 0);
			const usagePercentage = Math.min(100, Math.max(0, Math.round(rawPercentage * 10) / 10));
			const breakdown = {
				allocated: range.allocated_ips ?? 0,
				reserved: range.reserved_ips ?? 0,
				unavailable: range.unavailable_ips ?? 0,
				expired: range.expired_ips ?? 0,
				other: range.other_ips ?? 0
			};
			const breakdownText = createBreakdownText(breakdown);

			return {
				id: range.id,
				network: `${range.network_address}/${range.subnet_mask}`,
				name: range.name,
				description: range.description || '',
				gateway: range.gateway || '-',
				vlan: range.vlan_id !== undefined && range.vlan_id !== null ? range.vlan_id.toString() : '-',
				usage_percentage: usagePercentage,
				total_ips: total,
				used_ips: clampedUsed,
				available_ips: total > 0 ? clampedAvailable : Math.max(clampedAvailable, 0),
				version: range.ip_version === 6 ? 'IPv6' : 'IPv4',
				created_at: range.created_at,
				updated_at: range.updated_at,
				breakdown,
				breakdownText
			};
		});

		ipRanges = transformedRanges;

		// Calculate stats
		stats = {
			total_ranges: response.total,
			total_ips: ipRanges.reduce((sum, range) => sum + range.total_ips, 0),
			used_ips: ipRanges.reduce((sum, range) => sum + range.used_ips, 0),
			available_ips: ipRanges.reduce((sum, range) => sum + range.available_ips, 0)
		};
	} catch (err) {
		console.error('Failed to load IP ranges:', err);
		error = 'IP 대역 목록을 불러오는데 실패했습니다.';
	} finally {
		isLoading = false;
	}
}

	function handleAddIpRange() {
		editingIpRange = null;
		showAddDialog = true;
	}

	async function handleAddSuccess(newIpRange?: IpRange) {
		// Always reload to get the latest data
		await loadIpRanges();
	}

	function handleViewIpRange(ipRange: IpRangeDisplay) {
		console.log('View IP range:', ipRange);
		// TODO: Navigate to IP range detail page
	}

	async function handleEditIpRange(ipRange: IpRangeDisplay) {
		try {
			// Fetch full IP range data from API
			const fullIpRange = await ipRangeApi.getIpRange(ipRange.id);
			editingIpRange = fullIpRange;
			showAddDialog = true;
		} catch (err) {
			console.error('Failed to load IP range for editing:', err);
			alert('IP 대역 정보를 불러오는데 실패했습니다.');
		}
	}

	async function handleDeleteIpRange(ipRange: IpRangeDisplay) {
		if (confirm(`정말 ${ipRange.network} 대역을 삭제하시겠습니까?`)) {
			try {
				await ipRangeApi.deleteIpRange(ipRange.id);
				// Reload the list after successful deletion
				await loadIpRanges();
			} catch (err) {
				console.error('Failed to delete IP range:', err);
				alert('IP 대역 삭제에 실패했습니다.');
			}
		}
	}

	function handleDialogClose() {
		showAddDialog = false;
		editingIpRange = null;
	}

	// Filter IP ranges based on search query
	const filteredIpRanges = $derived.by(() => {
		if (!searchQuery.trim()) return ipRanges;

		const query = searchQuery.toLowerCase();
		return ipRanges.filter(
			(range) =>
				range.network.toLowerCase().includes(query) ||
				range.name.toLowerCase().includes(query) ||
				range.description.toLowerCase().includes(query) ||
				range.gateway.toLowerCase().includes(query)
		);
	});

	function getUsageColor(percentage: number): string {
		if (percentage >= 90) return 'text-red-600 dark:text-red-400';
		if (percentage >= 70) return 'text-yellow-600 dark:text-yellow-400';
		return 'text-green-600 dark:text-green-400';
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
							<Network class="h-6 w-6 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">IP 대역 관리</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">네트워크 IP 대역을 관리합니다.</p>
						</div>
					</div>
					<button
						onclick={handleAddIpRange}
						class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
					>
						<Plus class="h-4 w-4" />
						IP 대역 추가
					</button>
				</div>

				<!-- Search -->
				<div class="relative max-w-md">
					<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<Search class="h-4 w-4 text-gray-400" />
					</div>
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="IP 대역명, 네트워크 주소 또는 설명으로 검색..."
						class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white
							{isDesktop ? 'text-sm' : 'text-base'}"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Statistics Cards -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="mb-8 grid grid-cols-1 gap-6 md:grid-cols-4">
			<!-- Total IP Ranges -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">IP 대역</h3>
					<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
						<Network class="h-5 w-5 text-blue-600 dark:text-blue-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.total_ranges}</p>
			</div>

			<!-- Total IPs -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">IP 총수</h3>
					<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
						<Server class="h-5 w-5 text-purple-600 dark:text-purple-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.total_ips.toLocaleString()}</p>
			</div>

			<!-- Used IPs -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용중인 IP</h3>
					<div class="rounded-lg bg-orange-100 p-2 dark:bg-orange-900">
						<Activity class="h-5 w-5 text-orange-600 dark:text-orange-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.used_ips.toLocaleString()}</p>
			</div>

			<!-- Available IPs -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용가능한 IP</h3>
					<div class="rounded-lg bg-green-100 p-2 dark:bg-green-900">
						<Wifi class="h-5 w-5 text-green-600 dark:text-green-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.available_ips.toLocaleString()}</p>
			</div>
		</div>

		<!-- IP Ranges Table -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">IP 대역 목록</h2>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					{filteredIpRanges.length}개 IP 대역
				</p>
			</div>

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
							onclick={() => loadIpRanges()}
							class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
						>
							다시 시도
						</button>
					</div>
				</div>
			{:else if filteredIpRanges.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<Network class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
						<p class="mb-4 text-gray-500 dark:text-gray-400">
							{searchQuery ? '검색 결과가 없습니다.' : '등록된 IP 대역이 없습니다.'}
						</p>
						{#if !searchQuery}
							<button
								onclick={handleAddIpRange}
								class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
							>
								첫 번째 IP 대역 추가
							</button>
						{/if}
					</div>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-700">
							<tr>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									네트워크
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									대역명
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									게이트웨이
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									VLAN
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									사용률
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									IP 현황
								</th>
								<th
									class="px-6 py-3 text-right text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									작업
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each filteredIpRanges as ipRange (ipRange.id)}
								<tr class="transition-colors hover:bg-gray-50 dark:hover:bg-gray-700">
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<div
												class="mr-3 flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900"
											>
												<Network class="h-4 w-4 text-blue-600 dark:text-blue-400" />
											</div>
											<div>
												<div class="text-sm font-medium text-gray-900 dark:text-gray-100">
													{ipRange.network}
												</div>
												<div class="text-xs text-gray-500 dark:text-gray-400">
													{ipRange.version}
												</div>
											</div>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="text-sm font-medium text-gray-900 dark:text-gray-100">{ipRange.name}</div>
										{#if ipRange.description}
											<div class="text-xs text-gray-500 dark:text-gray-400">
												{ipRange.description}
											</div>
										{/if}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<Shield class="mr-1 h-4 w-4 text-gray-400" />
											<span class="text-sm text-gray-900 dark:text-gray-100">{ipRange.gateway}</span>
										</div>
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
										{ipRange.vlan}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex flex-col">
											<div class="flex items-center">
												<span class="text-sm font-medium {getUsageColor(ipRange.usage_percentage)}">
													{formatUsagePercentage(ipRange.usage_percentage)}%
												</span>
											</div>
											<div class="mt-1 h-2 w-32 rounded-full bg-gray-200 dark:bg-gray-700">
												<div
													class={`h-full rounded-full transition-all duration-300 ${getUsageBarClass(ipRange.usage_percentage)}`}
													style={`width: ${clampUsage(ipRange.usage_percentage)}%`}
												></div>
											</div>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
				<div class="text-sm text-gray-900 dark:text-gray-100">
					사용: {ipRange.used_ips.toLocaleString()}
				</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">
					사용가능: {ipRange.available_ips.toLocaleString()}
				</div>
				<div class="text-xs text-gray-400 dark:text-gray-500">
					총 {ipRange.total_ips.toLocaleString()}개
				</div>
				{#if ipRange.breakdownText !== '세부 현황 없음'}
					<div class="text-xs text-gray-400 dark:text-gray-500">
						{ipRange.breakdownText}
					</div>
				{/if}
									</td>
									<td class="px-6 py-4 text-right text-sm font-medium whitespace-nowrap">
										<div class="flex items-center justify-end gap-2">
											<button
												onclick={() => handleViewIpRange(ipRange)}
												class="text-gray-600 transition-colors hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
												title="상세보기"
											>
												<Eye class="h-4 w-4" />
											</button>
											<button
												onclick={() => handleEditIpRange(ipRange)}
												class="text-gray-600 transition-colors hover:text-yellow-600 dark:text-gray-400 dark:hover:text-yellow-400"
												title="수정"
											>
												<Edit class="h-4 w-4" />
											</button>
											<button
												onclick={() => handleDeleteIpRange(ipRange)}
												class="text-gray-600 transition-colors hover:text-red-600 dark:text-gray-400 dark:hover:text-red-400"
												title="삭제"
											>
												<Trash2 class="h-4 w-4" />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- IP Range Add/Edit Dialog -->
<IpRangeFormDialog
	open={showAddDialog}
	onClose={handleDialogClose}
	onSuccess={handleAddSuccess}
	editData={editingIpRange}
/>
