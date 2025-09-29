<script lang="ts">
	import {
		Server, Building, Plus, Edit, Trash2, Calendar, Search,
		Eye, Settings, Activity, HardDrive, Zap, Snowflake, MapPin
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { privateApi } from '$lib/api/private';
	import RackFormDialog from '$lib/components/rack/RackFormDialog.svelte';

	interface Rack {
		id: string;
		server_room_id: string;
		name: string;
		description?: string;
		rack_height: number;
		power_capacity?: number;
		cooling_type?: string;
		location_x?: number;
		location_y?: number;
		created_at: string;
		updated_at: string;
	}

	let searchQuery = $state('');
	let loading = $state(false);
	let racks = $state<Rack[]>([]);
	let total = $state(0);
	let page = $state(1);
	let limit = $state(20);

	// Dialog state
	let showDialog = $state(false);
	let editingRack = $state<Rack | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);

	interface RackListResponse {
		racks: Rack[];
		total: number;
		page: number;
		limit: number;
	}

	// Load racks data
	async function loadRacks() {
		loading = true;
		try {
			const searchParams = new URLSearchParams({
				page: page.toString(),
				limit: limit.toString()
			});

			const response = await privateApi.get(`v0/ipam/racks?${searchParams}`).json<RackListResponse>();
			racks = response.racks;
			total = response.total;
		} catch (error) {
			console.error('Failed to load racks:', error);
			racks = [];
			total = 0;
		} finally {
			loading = false;
		}
	}

	// Dialog functions
	function openCreateDialog() {
		editingRack = null;
		showDialog = true;
	}

	function openEditDialog(rack: Rack) {
		editingRack = rack;
		showDialog = true;
	}

	function closeDialog() {
		showDialog = false;
		editingRack = null;
	}

	function handleFormSuccess() {
		closeDialog();
		loadRacks();
	}

	// Delete rack
	async function deleteRack(rack: Rack) {
		if (!confirm(`'${rack.name}' 랙을 삭제하시겠습니까?`)) {
			return;
		}

		try {
			await privateApi.delete(`v0/ipam/racks/${rack.id}`);
			await loadRacks();
		} catch (error) {
			console.error('Failed to delete rack:', error);
			alert('랙 삭제에 실패했습니다.');
		}
	}

	// Format date
	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}

	onMount(() => {
		loadRacks();
	});
</script>

<div class="flex-1 min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="px-6 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="p-2 bg-purple-100 dark:bg-purple-900 rounded-lg">
						<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-purple-600 dark:text-purple-400" />
					</div>
					<div>
						<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">랙 관리</h1>
						<p class="text-sm text-gray-500 dark:text-gray-400">서버실 내 랙을 관리합니다.</p>
					</div>
				</div>
				<Button
					onclick={openCreateDialog}
					class="bg-purple-500 hover:bg-purple-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
				>
					<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
					랙 추가
				</Button>
			</div>
		</div>
	</div>

	<!-- Search and Stats -->
	<div class="p-6 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="flex items-center justify-between mb-4">
			<div class="relative flex-1 max-w-md">
				<Search class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
				<input
					type="text"
					placeholder="랙명, 서버실명, 사무실명 또는 랙번호를 검색..."
					bind:value={searchQuery}
					oninput={loadRacks}
					class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
				/>
			</div>
			<div class="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
				<span>총 {total}개</span>
			</div>
		</div>
	</div>

	<!-- Rack Cards Grid -->
	<div class="flex-1 p-6">
		{#if loading}
			<div class="flex items-center justify-center h-64">
				<div class="text-center">
					<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600 dark:border-purple-400 mx-auto"></div>
					<p class="mt-4 text-gray-600 dark:text-gray-400">랙을 불러오는 중...</p>
				</div>
			</div>
		{:else if racks.length === 0}
			<div class="flex items-center justify-center h-64">
				<div class="text-center">
					<Server class="h-12 w-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
					<p class="text-gray-500 dark:text-gray-400">
						{searchQuery ? '검색 결과가 없습니다.' : '등록된 랙이 없습니다.'}
					</p>
					{#if !searchQuery}
						<Button
							onclick={openCreateDialog}
							class="mt-4 bg-purple-500 hover:bg-purple-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
						>
							<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							첫 랙 추가하기
						</Button>
					{/if}
				</div>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
				{#each racks as rack (rack.id)}
					<div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:shadow-lg transition-shadow">
						<!-- Card Header -->
						<div class="p-4 border-b border-gray-200 dark:border-gray-700">
							<div class="flex items-start justify-between">
								<div class="flex items-start gap-3">
									<div class="{isDesktop ? 'w-8 h-8' : 'w-10 h-10'} bg-purple-100 dark:bg-purple-900/30 rounded-lg flex items-center justify-center">
										<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-purple-500" />
									</div>
									<div class="flex-1">
										<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">{rack.name}</h3>
										{#if rack.description}
											<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-gray-500 dark:text-gray-400 mt-0.5">{rack.description}</p>
										{/if}
									</div>
								</div>
								<div class="flex items-center gap-1">
									<button
										onclick={() => goto(`/ipam/racks/${rack.id}`)}
										class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 p-1"
										title="상세 보기"
									>
										<Eye class="h-3 w-3" />
									</button>
									<button
										onclick={() => openEditDialog(rack)}
										class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 p-1"
										title="수정"
									>
										<Edit class="h-3 w-3" />
									</button>
									<button
										onclick={() => deleteRack(rack)}
										class="text-gray-400 hover:text-red-600 dark:hover:text-red-400 p-1"
										title="삭제"
									>
										<Trash2 class="h-3 w-3" />
									</button>
								</div>
							</div>
						</div>

						<!-- Card Body -->
						<div class="p-4 space-y-3">
							<!-- Location Info -->
							<div class="space-y-1">
								<div class="flex items-center gap-2">
									<Building class="{isDesktop ? 'h-3 w-3' : 'h-3 w-3'} text-gray-400" />
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">서버실 ID: {rack.server_room_id}</p>
								</div>
								<div class="flex items-center gap-2">
									<Server class="{isDesktop ? 'h-3 w-3' : 'h-3 w-3'} text-gray-400" />
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">{rack.description || '설명 없음'}</p>
								</div>
							</div>

							<!-- Rack Specs -->
							<div class="space-y-1">
								<div class="flex items-center gap-2">
									<HardDrive class="{isDesktop ? 'h-3 w-3' : 'h-3 w-3'} text-gray-400" />
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
										{rack.rack_height}U 랙
									</p>
								</div>
								{#if rack.cooling_type}
									<div class="flex items-center gap-2">
										<Snowflake class="{isDesktop ? 'h-3 w-3' : 'h-3 w-3'} text-blue-400" />
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">{rack.cooling_type}</p>
									</div>
								{/if}
							</div>

							<!-- Usage Status -->
							<div class="space-y-1">
								<div class="flex items-center justify-between">
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">높이</span>
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white">{rack.rack_height}U</span>
								</div>
								{#if rack.power_capacity}
								<div class="flex items-center justify-between">
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">전력 용량</span>
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white">{rack.power_capacity}W</span>
								</div>
								{/if}
								{#if rack.cooling_type}
								<div class="flex items-center justify-between">
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">냉각 방식</span>
									<span class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white">{rack.cooling_type}</span>
								</div>
								{/if}
							</div>

							<div class="flex items-center gap-2">
								<Calendar class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">생성일: {formatDate(rack.created_at)}</p>
							</div>
						</div>

						<!-- Card Footer -->
						<div class="px-4 py-3 bg-gray-50 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-700">
							<div class="flex items-center justify-between">
								<div class="space-y-1">
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">상태</p>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-green-600 dark:text-green-400">활성</p>
								</div>
								<button
									onclick={() => goto(`/ipam/racks/${rack.id}`)}
									class="{isDesktop ? 'text-xs' : 'text-sm'} text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 transition-colors font-medium"
								>
									상세 보기
								</button>
							</div>
						</div>
					</div>
				{/each}
			</div>

			<!-- Pagination -->
			{#if total > limit}
				<div class="flex items-center justify-center mt-6 space-x-2">
					<button
						disabled={page === 1}
						onclick={() => { page--; loadRacks(); }}
						class="px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white disabled:opacity-50"
					>
						이전
					</button>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{page} / {Math.ceil(total / limit)}
					</span>
					<button
						disabled={page >= Math.ceil(total / limit)}
						onclick={() => { page++; loadRacks(); }}
						class="px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white disabled:opacity-50"
					>
						다음
					</button>
				</div>
			{/if}
		{/if}
	</div>
</div>

<RackFormDialog
	open={showDialog}
	rack={editingRack}
	onClose={closeDialog}
	onSuccess={handleFormSuccess}
/>