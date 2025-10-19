<script lang="ts">
	import {
		AlertTriangle,
		Building,
		Calendar,
		ChevronRight,
		Edit,
		Eye,
		HardDrive,
		LayoutGrid,
		List,
		MapPin,
		Plus,
		Search,
		Server,
		Snowflake,
		Trash2,
		Zap,
		X
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
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

	const SEARCH_DEBOUNCE = 150;
	const VIEW_MODE_STORAGE_KEY = 'ipam-racks-view-mode';

	let searchQuery = $state('');
	let loading = $state(false);
	let errorMessage = $state<string | null>(null);
	let racks = $state<Rack[]>([]);
	let total = $state(0);
	let page = $state(1);
	let limit = $state(12);
	let viewMode = $state<'grid' | 'list'>('grid');

	// Dialog state
	let showDialog = $state(false);
	let editingRack = $state<Rack | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);
	const totalPages = $derived(Math.max(1, Math.ceil(total / limit)));
	const showingRange = $derived(
		total
			? `${((page - 1) * limit + 1).toLocaleString('ko-KR')} - ${Math.min(total, page * limit).toLocaleString('ko-KR')}`
			: '0 - 0'
	);
	const formattedTotal = $derived(total.toLocaleString('ko-KR'));
	const skeletonItems = $derived(Array.from({ length: viewMode === 'grid' ? (isDesktop ? 6 : 4) : 4 }, (_, index) => index));

	interface RackListResponse {
		racks: Rack[];
		total: number;
		page: number;
		limit: number;
	}

	// Load racks data
	async function loadRacks(requestedPage = page) {
		loading = true;
		errorMessage = null;
		try {
			const searchParams = new URLSearchParams({
				page: requestedPage.toString(),
				limit: limit.toString()
			});
			if (searchQuery.trim()) {
				searchParams.set('search', searchQuery.trim());
			}

			const response = await privateApi.get(`v0/ipam/racks?${searchParams}`).json<RackListResponse>();
			racks = response.racks;
			total = response.total;
			page = response.page ?? requestedPage;
			limit = response.limit ?? limit;
		} catch (error) {
			console.error('Failed to load racks:', error);
			errorMessage = '랙 정보를 불러오지 못했습니다.';
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
		const date = new Date(dateString);
		if (Number.isNaN(date.getTime())) {
			return '-';
		}
		return date.toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: 'numeric',
			day: 'numeric'
		});
	}

	function handleLimitChange(event: Event) {
		const value = Number((event.target as HTMLSelectElement).value);
		if (!Number.isNaN(value) && value !== limit) {
			limit = value;
			loadRacks(1);
		}
	}

	function clearSearch() {
		if (searchQuery) {
			searchQuery = '';
			loadRacks(1);
		}
	}

	function setViewMode(mode: 'grid' | 'list') {
		viewMode = mode;
		if (browser) {
			try {
				window.localStorage.setItem(VIEW_MODE_STORAGE_KEY, mode);
			} catch (error) {
				console.warn('Unable to persist view mode', error);
			}
		}
	}

	function handleCardKey(event: KeyboardEvent, rackId: string) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			goto(`/ipam/racks/${rackId}`);
		}
	}

	function openRackDetail(rackId: string) {
		goto(`/ipam/racks/${rackId}`);
	}

	let searchTimeout: ReturnType<typeof setTimeout>;
	$effect(() => {
		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			loadRacks(1);
		}, SEARCH_DEBOUNCE);
		return () => {
			if (searchTimeout) clearTimeout(searchTimeout);
		};
	});

	onMount(() => {
		if (browser) {
			const storedMode = window.localStorage.getItem(VIEW_MODE_STORAGE_KEY);
			if (storedMode === 'grid' || storedMode === 'list') {
				viewMode = storedMode;
			}
		}
		loadRacks();
	});
</script>

<div class="flex min-h-screen flex-1 flex-col bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
		<div class="px-6 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
						<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-purple-600 dark:text-purple-400" />
					</div>
					<div>
						<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">랙 관리</h1>
						<p class="text-sm text-gray-500 dark:text-gray-400">서버실 내 랙을 관리합니다.</p>
					</div>
				</div>
				<Button
					on:click={openCreateDialog}
					class="bg-purple-500 text-white hover:bg-purple-600 {isDesktop ? 'px-3 py-1.5 text-xs' : ''}"
				>
					<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
					랙 추가
				</Button>
			</div>
		</div>
	</div>

	<!-- Search and Stats -->
	<div class="border-b border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-800">
		<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
			<div class="relative w-full md:max-w-lg">
				<Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400" />
				<input
					type="text"
					placeholder="랙명, 서버실명, 사무실명 또는 랙번호를 검색..."
					bind:value={searchQuery}
					class="w-full rounded-lg border border-gray-300 bg-gray-50 py-2 pl-10 pr-4 text-sm text-gray-900 focus:ring-2 focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-900 dark:text-white"
				/>
				{#if searchQuery}
					<button
						type="button"
						class="absolute top-1/2 right-2 flex h-6 w-6 -translate-y-1/2 items-center justify-center rounded-full border border-transparent text-gray-400 transition hover:text-gray-600 dark:hover:text-gray-200"
						on:click={clearSearch}
						aria-label="검색어 초기화"
					>
						<X class="h-4 w-4" />
					</button>
				{/if}
			</div>
			<div class="flex flex-wrap items-center gap-3 text-sm text-gray-600 dark:text-gray-300">
				<span class="rounded-full bg-gray-100 px-3 py-1 text-xs font-medium dark:bg-gray-700">
					표시 구간 {showingRange} / 총 {formattedTotal}개
				</span>
				<label class="flex items-center gap-2">
					<span class="text-xs uppercase tracking-wide text-gray-500 dark:text-gray-400">Page size</span>
					<select
						class="rounded-md border border-gray-300 bg-white px-2 py-1 text-xs font-medium text-gray-700 focus:border-purple-500 focus:outline-none focus:ring-1 focus:ring-purple-500 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-200"
						on:change={handleLimitChange}
						value={limit}
					>
						<option value="12">12</option>
						<option value="20">20</option>
						<option value="50">50</option>
					</select>
				</label>
				<div class="ml-auto flex items-center gap-1 rounded-md border border-gray-300 bg-white p-1 text-gray-500 shadow-sm dark:border-gray-600 dark:bg-gray-900">
					<button
						type="button"
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-purple-500 {viewMode === 'grid'
							? 'bg-purple-500 text-white shadow-sm'
							: 'hover:text-gray-800 dark:hover:text-gray-200'}"
						on:click={() => setViewMode('grid')}
						aria-pressed={viewMode === 'grid'}
					>
						<LayoutGrid class="h-3.5 w-3.5" />
						<span class="hidden sm:inline">그리드</span>
					</button>
					<button
						type="button"
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-purple-500 {viewMode === 'list'
							? 'bg-purple-500 text-white shadow-sm'
							: 'hover:text-gray-800 dark:hover:text-gray-200'}"
						on:click={() => setViewMode('list')}
						aria-pressed={viewMode === 'list'}
					>
						<List class="h-3.5 w-3.5" />
						<span class="hidden sm:inline">리스트</span>
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Rack Cards Grid -->
	<div class="flex-1 p-6">
		{#if errorMessage}
			<div class="mb-4 flex items-center gap-2 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-600 dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-300">
				<AlertTriangle class="h-4 w-4" />
				<span>{errorMessage}</span>
			</div>
		{/if}

		{#if loading}
			{#if viewMode === 'grid'}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{#each skeletonItems as index (index)}
						<div class="animate-pulse rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
							<div class="mb-4 h-5 w-32 rounded bg-gray-200 dark:bg-gray-700"></div>
							<div class="space-y-2">
								<div class="h-4 w-full rounded bg-gray-200 dark:bg-gray-700"></div>
								<div class="h-4 w-3/4 rounded bg-gray-200 dark:bg-gray-700"></div>
								<div class="h-4 w-2/3 rounded bg-gray-200 dark:bg-gray-700"></div>
							</div>
							<div class="mt-6 flex items-center justify-between">
								<div class="h-4 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
								<div class="flex gap-2">
									<div class="h-8 w-8 rounded bg-gray-200 dark:bg-gray-700"></div>
									<div class="h-8 w-8 rounded bg-gray-200 dark:bg-gray-700"></div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="space-y-3">
					{#each skeletonItems as index (index)}
						<div class="animate-pulse rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
							<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
								<div>
									<div class="mb-2 h-5 w-48 rounded bg-gray-200 dark:bg-gray-700"></div>
									<div class="h-4 w-64 rounded bg-gray-200 dark:bg-gray-700"></div>
								</div>
								<div class="h-6 w-20 rounded-full bg-gray-200 dark:bg-gray-700"></div>
							</div>
							<div class="mt-4 grid gap-2 md:grid-cols-3">
								<div class="h-4 rounded bg-gray-200 dark:bg-gray-700"></div>
								<div class="h-4 rounded bg-gray-200 dark:bg-gray-700"></div>
								<div class="h-4 rounded bg-gray-200 dark:bg-gray-700"></div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{:else if racks.length === 0}
			<div class="flex h-64 items-center justify-center">
				<div class="text-center">
					<Server class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
					<p class="text-gray-500 dark:text-gray-400">
						{searchQuery ? '검색 결과가 없습니다.' : '등록된 랙이 없습니다.'}
					</p>
					{#if !searchQuery}
						<Button
							on:click={openCreateDialog}
							class="mt-4 bg-purple-500 text-white hover:bg-purple-600 {isDesktop ? 'px-3 py-1.5 text-xs' : ''}"
						>
							<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							첫 랙 추가하기
						</Button>
					{/if}
				</div>
			</div>
		{:else}
			{#if viewMode === 'grid'}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{#each racks as rack (rack.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white transition-all hover:-translate-y-1 hover:border-purple-300 hover:shadow-lg focus-within:border-purple-400 focus-within:ring-2 focus-within:ring-purple-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-purple-500/60 dark:focus-within:ring-purple-500/20"
							role="button"
							tabindex="0"
							on:click={() => openRackDetail(rack.id)}
							on:keydown={(event) => handleCardKey(event, rack.id)}
						>
							<div class="border-b border-gray-200 p-4 dark:border-gray-700">
								<div class="flex items-start justify-between gap-3">
									<div class="flex items-start gap-3">
										<div
											class="{isDesktop
												? 'h-8 w-8'
												: 'h-10 w-10'} flex items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900/30"
										>
											<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-purple-500" />
										</div>
										<div>
											<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
												{rack.name}
											</h3>
											<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} mt-0.5 line-clamp-2 text-gray-500 dark:text-gray-400">
												{rack.description || '설명이 등록되지 않았습니다.'}
											</p>
										</div>
									</div>
									<span class="rounded-full bg-purple-100 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-purple-600 dark:bg-purple-900/30 dark:text-purple-300">
										랙
									</span>
								</div>
							</div>

							<div class="space-y-3 p-4">
								<div class="flex items-start gap-2 text-xs text-gray-600 dark:text-gray-300">
									<Building class="h-3 w-3 text-gray-400" />
									<span>서버실 ID: {rack.server_room_id}</span>
								</div>
								<div class="space-y-1">
									<div class="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-300">
										<HardDrive class="h-3 w-3 text-gray-400" />
										<span>{rack.rack_height}U 랙</span>
									</div>
									{#if rack.power_capacity}
										<div class="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-300">
											<Zap class="h-3 w-3 text-yellow-500" />
											<span>{rack.power_capacity}W 전력 용량</span>
										</div>
									{/if}
									{#if rack.cooling_type}
										<div class="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-300">
											<Snowflake class="h-3 w-3 text-blue-400" />
											<span>{rack.cooling_type}</span>
										</div>
									{/if}
									{#if rack.location_x || rack.location_y}
										<div class="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-300">
											<MapPin class="h-3 w-3 text-emerald-400" />
											<span>
												좌표 {rack.location_x ?? '-'} , {rack.location_y ?? '-'}
											</span>
										</div>
									{/if}
								</div>
								<div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
									<Calendar class="h-3 w-3" />
									생성일: {formatDate(rack.created_at)}
								</div>
							</div>

							<div class="border-t border-gray-200 bg-gray-50 px-4 py-3 dark:border-gray-700 dark:bg-gray-900/50">
								<div class="flex items-center justify-between">
									<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-purple-500 dark:text-gray-400 dark:group-hover:text-purple-300">
										상세 보기
										<ChevronRight class="h-4 w-4" />
									</span>
									<div class="flex items-center gap-2">
										<button
											type="button"
											on:click|stopPropagation={() => openEditDialog(rack)}
											class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
											title="수정"
										>
											<Edit class="h-4 w-4" />
										</button>
										<button
											type="button"
											on:click|stopPropagation={() => deleteRack(rack)}
											class="rounded-md p-1 text-red-600 transition hover:bg-red-50 hover:text-red-700 focus:outline-none focus:ring-2 focus:ring-red-200 dark:text-red-300 dark:hover:bg-red-900/30 dark:hover:text-red-200 dark:focus:ring-red-500/40"
											title="삭제"
										>
											<Trash2 class="h-4 w-4" />
										</button>
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="space-y-3">
					{#each racks as rack (rack.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white p-4 transition-all hover:-translate-y-[2px] hover:border-purple-300 hover:shadow-md focus-within:border-purple-400 focus-within:ring-2 focus-within:ring-purple-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-purple-500/60 dark:focus-within:ring-purple-500/20"
							role="button"
							tabindex="0"
							on:click={() => openRackDetail(rack.id)}
							on:keydown={(event) => handleCardKey(event, rack.id)}
						>
							<div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
								<div class="flex items-start gap-3">
									<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900/30">
										<Server class="h-5 w-5 text-purple-500" />
									</div>
									<div>
										<h3 class="text-base font-semibold text-gray-900 dark:text-white">
											{rack.name}
										</h3>
										<p class="text-sm text-gray-500 dark:text-gray-400">{rack.description || '설명 없음'}</p>
										<div class="mt-1 flex flex-wrap gap-1 text-[11px] text-gray-500 dark:text-gray-400">
											<span>{rack.rack_height}U</span>
											{#if rack.power_capacity}
												<span>· {rack.power_capacity}W</span>
											{/if}
											{#if rack.cooling_type}
												<span>· {rack.cooling_type}</span>
											{/if}
										</div>
									</div>
								</div>
								<div class="flex items-center gap-2 md:flex-col md:items-end md:gap-1 text-xs text-gray-500 dark:text-gray-400">
									<span class="flex items-center gap-1">
										<Calendar class="h-3 w-3" />
										{formatDate(rack.created_at)}
									</span>
									{#if rack.location_x || rack.location_y}
										<span class="flex items-center gap-1">
											<MapPin class="h-3 w-3" />
											{rack.location_x ?? '-'} , {rack.location_y ?? '-'}
										</span>
									{/if}
								</div>
							</div>
							<div class="mt-4 flex items-center justify-between">
								<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-purple-500 dark:text-gray-400 dark:group-hover:text-purple-300">
									상세 보기
									<ChevronRight class="h-4 w-4" />
								</span>
								<div class="flex items-center gap-2">
									<button
										type="button"
										on:click|stopPropagation={() => openEditDialog(rack)}
										class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
										title="수정"
									>
										<Edit class="h-4 w-4" />
									</button>
									<button
										type="button"
										on:click|stopPropagation={() => deleteRack(rack)}
										class="rounded-md p-1 text-red-600 transition hover:bg-red-50 hover:text-red-700 focus:outline-none focus:ring-2 focus:ring-red-200 dark:text-red-300 dark:hover:bg-red-900/30 dark:hover:text-red-200 dark:focus:ring-red-500/40"
										title="삭제"
									>
										<Trash2 class="h-4 w-4" />
									</button>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			{#if totalPages > 1}
				<div class="mt-6 flex items-center justify-center space-x-2">
					<button
						type="button"
						disabled={page === 1}
						on:click={() => {
							const nextPage = Math.max(1, page - 1);
							if (nextPage !== page) {
								loadRacks(nextPage);
							}
						}}
						class="px-3 py-2 text-sm text-gray-600 hover:text-gray-900 disabled:opacity-50 dark:text-gray-400 dark:hover:text-white"
					>
						이전
					</button>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{page} / {totalPages}
					</span>
					<button
						type="button"
						disabled={page >= totalPages}
						on:click={() => {
							const nextPage = Math.min(totalPages, page + 1);
							if (nextPage !== page) {
								loadRacks(nextPage);
							}
						}}
						class="px-3 py-2 text-sm text-gray-600 hover:text-gray-900 disabled:opacity-50 dark:text-gray-400 dark:hover:text-white"
					>
						다음
					</button>
				</div>
			{/if}
		{/if}
	</div>
</div>

<RackFormDialog open={showDialog} rack={editingRack} onClose={closeDialog} onSuccess={handleFormSuccess} />
