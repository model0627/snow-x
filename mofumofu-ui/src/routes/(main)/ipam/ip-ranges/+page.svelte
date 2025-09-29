<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		Search, Plus, Network, Eye, Edit, Trash2,
		Server, Wifi, Shield, Activity
	} from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import IpRangeFormDialog from '$lib/components/ipam/IpRangeFormDialog.svelte';

	interface IpRange {
		id: string;
		network: string;
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
	}

	interface IpRangeStats {
		total_ranges: number;
		total_ips: number;
		used_ips: number;
		available_ips: number;
	}

	let ipRanges = $state<IpRange[]>([]);
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

	const isDesktop = $derived(desktopStore.isDesktop);

	// No mock data - start with empty state

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}
		await loadIpRanges();
	});

	async function loadIpRanges() {
		try {
			isLoading = true;
			error = '';

			// TODO: Replace with actual API calls
			// const response = await privateApi.get('v0/ipam/ip-ranges').json();
			// ipRanges = response.ip_ranges;
			// stats = response.stats;

			// Start with empty state
			await new Promise(resolve => setTimeout(resolve, 500)); // Simulate API delay
			ipRanges = [];
			stats = {
				total_ranges: 0,
				total_ips: 0,
				used_ips: 0,
				available_ips: 0
			};

		} catch (err) {
			console.error('Failed to load IP ranges:', err);
			error = 'IP 대역 목록을 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
	}

	function handleAddIpRange() {
		showAddDialog = true;
	}

	function handleAddSuccess() {
		// Reload IP ranges after successful addition
		loadIpRanges();
	}

	function handleViewIpRange(ipRange: IpRange) {
		console.log('View IP range:', ipRange);
		// TODO: Navigate to IP range detail page
	}

	function handleEditIpRange(ipRange: IpRange) {
		console.log('Edit IP range:', ipRange);
		// TODO: Open edit IP range dialog
	}

	function handleDeleteIpRange(ipRange: IpRange) {
		if (confirm(`정말 ${ipRange.network} 대역을 삭제하시겠습니까?`)) {
			console.log('Delete IP range:', ipRange);
			// TODO: Call delete API
		}
	}

	// Filter IP ranges based on search query
	const filteredIpRanges = $derived(() => {
		if (!searchQuery.trim()) return ipRanges;

		const query = searchQuery.toLowerCase();
		return ipRanges.filter(range =>
			range.network.toLowerCase().includes(query) ||
			range.description.toLowerCase().includes(query) ||
			range.gateway.toLowerCase().includes(query)
		);
	});

	function getUsageColor(percentage: number): string {
		if (percentage >= 90) return 'text-red-600 dark:text-red-400';
		if (percentage >= 70) return 'text-yellow-600 dark:text-yellow-400';
		return 'text-green-600 dark:text-green-400';
	}

	function getUsageBgColor(percentage: number): string {
		if (percentage >= 90) return 'bg-red-500';
		if (percentage >= 70) return 'bg-yellow-500';
		return 'bg-green-500';
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="py-6">
				<!-- Title and Add Button -->
				<div class="flex items-center justify-between mb-6">
					<div class="flex items-center gap-3">
						<div class="p-3 bg-blue-100 dark:bg-blue-900 rounded-lg">
							<Network class="w-6 h-6 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
								IP 대역 관리
							</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								네트워크 IP 대역을 관리합니다.
							</p>
						</div>
					</div>
					<button
						onclick={handleAddIpRange}
						class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
					>
						<Plus class="w-4 h-4" />
						IP 대역 추가
					</button>
				</div>

				<!-- Search -->
				<div class="relative max-w-md">
					<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
						<Search class="h-4 w-4 text-gray-400" />
					</div>
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="IP 대역명, 네트워크 주소 또는 설명으로 검색..."
						class="block w-full pl-10 pr-3 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg
							bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
							focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors
							{isDesktop ? 'text-sm' : 'text-base'}"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Statistics Cards -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
		<div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
			<!-- Total IP Ranges -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">IP 대역</h3>
					<div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
						<Network class="w-5 h-5 text-blue-600 dark:text-blue-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.total_ranges}</p>
			</div>

			<!-- Total IPs -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">IP 총수</h3>
					<div class="p-2 bg-purple-100 dark:bg-purple-900 rounded-lg">
						<Server class="w-5 h-5 text-purple-600 dark:text-purple-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.total_ips.toLocaleString()}</p>
			</div>

			<!-- Used IPs -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용중인 IP</h3>
					<div class="p-2 bg-orange-100 dark:bg-orange-900 rounded-lg">
						<Activity class="w-5 h-5 text-orange-600 dark:text-orange-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.used_ips}</p>
			</div>

			<!-- Available IPs -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용가능한 IP</h3>
					<div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
						<Wifi class="w-5 h-5 text-green-600 dark:text-green-400" />
					</div>
				</div>
				<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{stats.available_ips.toLocaleString()}</p>
			</div>
		</div>

		<!-- IP Ranges Table -->
		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden">
			<div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					IP 대역 목록
				</h2>
				<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
					{filteredIpRanges.length}개 IP 대역
				</p>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 dark:border-blue-400 mx-auto"></div>
						<p class="mt-2 text-gray-500 dark:text-gray-400 {isDesktop ? 'text-sm' : 'text-base'}">로딩 중...</p>
					</div>
				</div>
			{:else if error}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<p class="text-red-600 dark:text-red-400 mb-4">{error}</p>
						<button
							onclick={() => loadIpRanges()}
							class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
						>
							다시 시도
						</button>
					</div>
				</div>
			{:else if filteredIpRanges.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<Network class="h-12 w-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
						<p class="text-gray-500 dark:text-gray-400 mb-4">
							{searchQuery ? '검색 결과가 없습니다.' : '등록된 IP 대역이 없습니다.'}
						</p>
						{#if !searchQuery}
							<button
								onclick={handleAddIpRange}
								class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
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
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									네트워크
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									대역명
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									게이트웨이
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									VLAN
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									사용률
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									IP 현황
								</th>
								<th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
									작업
								</th>
							</tr>
						</thead>
						<tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
							{#each filteredIpRanges as ipRange (ipRange.id)}
								<tr class="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<div class="flex-shrink-0 w-8 h-8 flex items-center justify-center bg-blue-100 dark:bg-blue-900 rounded-lg mr-3">
												<Network class="w-4 h-4 text-blue-600 dark:text-blue-400" />
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
										<div class="text-sm text-gray-900 dark:text-gray-100">{ipRange.description}</div>
										<div class="text-xs text-gray-500 dark:text-gray-400">
											서울 본사 서버 네트워크
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<Shield class="w-4 h-4 text-gray-400 mr-1" />
											<span class="text-sm text-gray-900 dark:text-gray-100">{ipRange.gateway}</span>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
										{ipRange.vlan}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<span class="text-sm font-medium {getUsageColor(ipRange.usage_percentage)}">
												{ipRange.usage_percentage}%
											</span>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="text-sm text-gray-900 dark:text-gray-100">
											사용: {ipRange.used_ips}
										</div>
										<div class="text-sm text-gray-500 dark:text-gray-400">
											사용가능: {ipRange.available_ips}
										</div>
										<div class="text-xs text-gray-400 dark:text-gray-500">
											총 {ipRange.total_ips}개
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
										<div class="flex items-center justify-end gap-2">
											<button
												onclick={() => handleViewIpRange(ipRange)}
												class="text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
												title="상세보기"
											>
												<Eye class="w-4 h-4" />
											</button>
											<button
												onclick={() => handleEditIpRange(ipRange)}
												class="text-gray-600 dark:text-gray-400 hover:text-yellow-600 dark:hover:text-yellow-400 transition-colors"
												title="수정"
											>
												<Edit class="w-4 h-4" />
											</button>
											<button
												onclick={() => handleDeleteIpRange(ipRange)}
												class="text-gray-600 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
												title="삭제"
											>
												<Trash2 class="w-4 h-4" />
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

<!-- IP Range Add Dialog -->
<IpRangeFormDialog
	open={showAddDialog}
	onClose={() => showAddDialog = false}
	onSuccess={handleAddSuccess}
/>