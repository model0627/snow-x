<script lang="ts">
	import {
		AlertTriangle,
		ArrowRight,
		Building,
		Calendar,
		ChevronRight,
		Droplets,
		Edit,
		LayoutGrid,
		List,
		Plus,
		Search,
		Server,
		Shield,
		Thermometer,
		Trash2,
		X
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { serverRoomApi, officeApi, type ServerRoom, type Office } from '$lib/api/office';
	import ServerRoomFormDialogWithOfficeSelector from '$lib/components/office/ServerRoomFormDialogWithOfficeSelector.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	const SEARCH_DEBOUNCE = 150;
	const VIEW_MODE_STORAGE_KEY = 'ipam-server-rooms-view-mode';

	let searchQuery = $state('');
	let loading = $state(false);
	let errorMessage = $state<string | null>(null);
	let offices = $state<Office[]>([]);
	let allServerRooms = $state<(ServerRoom & { office_name: string })[]>([]);
	let filteredServerRooms = $state<(ServerRoom & { office_name: string })[]>([]);
	let page = $state(1);
	let limit = $state(12);
	let viewMode = $state<'grid' | 'list'>('grid');

	// Dialog state
	let showDialog = $state(false);
	let editingServerRoom = $state<ServerRoom | null>(null);
	let selectedOfficeId = $state<string>('');

	const isDesktop = $derived(desktopStore.isDesktop);
	const total = $derived(filteredServerRooms.length);
	const totalPages = $derived(Math.max(1, Math.ceil(total / limit)));
	const showingRange = $derived(
		total
			? `${((page - 1) * limit + 1).toLocaleString('ko-KR')} - ${Math.min(total, page * limit).toLocaleString('ko-KR')}`
			: '0 - 0'
	);
	const formattedTotal = $derived(total.toLocaleString('ko-KR'));
	const skeletonItems = $derived(Array.from({ length: viewMode === 'grid' ? (isDesktop ? 6 : 4) : 4 }, (_, index) => index));
	const filteredOfficeCount = $derived(new Set(filteredServerRooms.map((room) => room.office_id)).size);
	const monitoringStats = $derived(() => {
		return filteredServerRooms.reduce(
			(stats, room) => {
				if (room.temperature_monitoring) stats.temperature += 1;
				if (room.humidity_monitoring) stats.humidity += 1;
				if (room.access_control) stats.access += 1;
				return stats;
			},
			{ temperature: 0, humidity: 0, access: 0 }
		);
	});
	function computeLastUpdated(items: (ServerRoom & { office_name: string })[]): string | null {
		if (!items.length) return null;
		const latest = items.reduce((current, room) => {
			const updatedAt = new Date(room.updated_at ?? room.created_at);
			return updatedAt > current ? updatedAt : current;
		}, new Date(items[0].updated_at ?? items[0].created_at));

		return latest.toLocaleString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
	const lastUpdated = $derived(computeLastUpdated(filteredServerRooms));

	// Load all offices and their server rooms
	async function loadData() {
		loading = true;
		errorMessage = null;
		try {
			const officesResponse = await officeApi.getOffices({
				page: 1,
				limit: 200
			});
			offices = officesResponse.offices;

			if (!offices.length) {
				allServerRooms = [];
				updateFilteredRooms({ resetPage: true });
				return;
			}

			const serverRoomCollections = await Promise.all(
				offices.map(async (office) => {
					try {
						const serverRoomsResponse = await serverRoomApi.getServerRooms(office.id, {
							page: 1,
							limit: 200
						});

						return serverRoomsResponse.server_rooms.map((room) => ({
							...room,
							office_name: office.name
						}));
					} catch (error) {
						console.error(`Failed to load server rooms for office ${office.name}:`, error);
						return [];
					}
				})
			);

			allServerRooms = serverRoomCollections.flat();
			updateFilteredRooms({ resetPage: true });
		} catch (error) {
			console.error('Failed to load data:', error);
			errorMessage = '서버실 정보를 불러오지 못했습니다.';
		} finally {
			loading = false;
		}
	}

	// Navigate to specific office's server room management
	function navigateToOfficeServers(officeId: string) {
		goto(`/ipam/offices/${officeId}/servers`);
	}

	// Navigate to specific office detail
	function navigateToOffice(officeId: string) {
		goto(`/ipam/offices/${officeId}`);
	}

	function openServerRoomDetail(room: ServerRoom & { office_name: string }) {
		goto(`/ipam/server-rooms/${room.id}?office=${room.office_id}`);
	}

	function handleCardKey(event: KeyboardEvent, room: ServerRoom & { office_name: string }) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			openServerRoomDetail(room);
		}
	}

	function setViewMode(mode: 'grid' | 'list') {
		viewMode = mode;
		if (browser) {
			try {
				window.localStorage.setItem(VIEW_MODE_STORAGE_KEY, mode);
			} catch (error) {
				console.warn('Unable to persist server rooms view mode', error);
			}
		}
	}

	function handleLimitChange(event: Event) {
		const value = Number((event.target as HTMLSelectElement).value);
		if (!Number.isNaN(value) && value !== limit) {
			limit = value;
			updateFilteredRooms({ resetPage: true });
		}
	}

	function clearSearch() {
		if (searchQuery) {
			searchQuery = '';
			updateFilteredRooms({ resetPage: true });
		}
	}

	function updateFilteredRooms(options: { resetPage?: boolean } = {}) {
		const { resetPage = false } = options;
		const query = searchQuery.trim().toLowerCase();

		const filtered = query
			? allServerRooms.filter((room) => {
					const name = room.name.toLowerCase();
					const office = room.office_name.toLowerCase();
					const description = room.description?.toLowerCase() ?? '';
					return name.includes(query) || office.includes(query) || description.includes(query);
			  })
			: allServerRooms;

		filteredServerRooms = filtered;

		const effectiveLimit = Math.max(limit, 1);
		const newTotalPages = Math.max(1, Math.ceil(filtered.length / effectiveLimit));

		if (resetPage) {
			page = 1;
		} else if (page > newTotalPages) {
			page = newTotalPages;
		}
		if (filtered.length === 0) {
			page = 1;
		}
	}

	// Delete server room
	async function deleteServerRoom(serverRoom: ServerRoom & { office_name: string }) {
		if (confirm(`"${serverRoom.name}" 서버실을 삭제하시겠습니까?`)) {
			try {
				await serverRoomApi.deleteServerRoom(serverRoom.office_id, serverRoom.id);
				await loadData(); // Reload the list
			} catch (error) {
				console.error('Failed to delete server room:', error);
				alert('서버실 삭제에 실패했습니다.');
			}
		}
	}

	// Open create dialog
	function openCreateDialog() {
		if (offices.length === 0) {
			alert('먼저 사무실을 생성해주세요.');
			goto('/ipam/offices');
			return;
		}
		editingServerRoom = null;
		selectedOfficeId = offices[0].id; // Default to first office
		showDialog = true;
	}

	// Open edit dialog
	function openEditDialog(serverRoom: ServerRoom & { office_name: string }) {
		editingServerRoom = serverRoom;
		selectedOfficeId = serverRoom.office_id;
		showDialog = true;
	}

	// Close dialog
	function closeDialog() {
		showDialog = false;
		editingServerRoom = null;
		selectedOfficeId = '';
	}

	// Handle successful form submission
	function handleFormSuccess() {
		loadData();
		closeDialog();
	}

	// Format date
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}

	// Search handler with debounce
	let searchTimeout: ReturnType<typeof setTimeout>;
	$effect(() => {
		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			updateFilteredRooms({ resetPage: true });
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
		loadData();
	});
</script>

<div class="flex min-h-screen flex-1 flex-col bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
		<div class="px-6 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
					<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">전체 서버실</h1>
				</div>
				<Button
					on:click={openCreateDialog}
					class="bg-blue-500 text-white hover:bg-blue-600 {isDesktop ? 'px-3 py-1.5 text-xs' : ''}"
				>
					<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
					새 서버실 추가
				</Button>
			</div>
			<p class="mt-2 {isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400">
				모든 사무실의 서버실을 한 번에 확인할 수 있습니다.
			</p>
		</div>
	</div>

	<!-- Stats -->
	<div class="bg-gray-50 px-6 py-4 dark:bg-gray-900">
		<div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
			<div class="flex items-center gap-3 rounded-lg border border-blue-200 bg-blue-50/70 p-4 dark:border-blue-900/40 dark:bg-blue-900/20">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-500/10 text-blue-600 dark:text-blue-300">
					<Server class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-blue-500/70 dark:text-blue-300/80">총 서버실</p>
					<p class="text-lg font-semibold text-gray-900 dark:text-white">{formattedTotal}</p>
				</div>
			</div>
			<div class="flex items-center gap-3 rounded-lg border border-indigo-200 bg-indigo-50/70 p-4 dark:border-indigo-900/40 dark:bg-indigo-900/20">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-indigo-500/10 text-indigo-600 dark:text-indigo-300">
					<Building class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-indigo-500/70 dark:text-indigo-300/80">
						포함 사무실
					</p>
					<p class="text-lg font-semibold text-gray-900 dark:text-white">{filteredOfficeCount.toLocaleString('ko-KR')}개</p>
				</div>
			</div>
			<div class="flex items-center gap-3 rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-200">
					<Calendar class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">마지막 업데이트</p>
					<p class="text-sm font-semibold text-gray-900 dark:text-white">{lastUpdated ?? '데이터 없음'}</p>
					<div class="mt-1 flex items-center gap-2 text-[10px] font-medium uppercase tracking-wide text-gray-400 dark:text-gray-500">
						<span>온도 {monitoringStats.temperature}</span>
						<span>습도 {monitoringStats.humidity}</span>
						<span>출입 {monitoringStats.access}</span>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Search Bar -->
	<div class="border-b border-gray-200 bg-white px-6 py-4 dark:border-gray-700 dark:bg-gray-800">
		<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
			<div class="relative w-full md:max-w-lg">
				<Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400" />
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="서버실명 또는 사무실명으로 검색..."
					class="w-full rounded-lg border border-gray-300 bg-gray-50 py-2 pl-10 pr-4 text-sm text-gray-900 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-900 dark:text-white"
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
						class="rounded-md border border-gray-300 bg-white px-2 py-1 text-xs font-medium text-gray-700 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-200"
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
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-blue-500 {viewMode === 'grid'
							? 'bg-blue-500 text-white shadow-sm'
							: 'hover:text-gray-800 dark:hover:text-gray-200'}"
						on:click={() => setViewMode('grid')}
						aria-pressed={viewMode === 'grid'}
					>
						<LayoutGrid class="h-3.5 w-3.5" />
						<span class="hidden sm:inline">그리드</span>
					</button>
					<button
						type="button"
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-blue-500 {viewMode === 'list'
							? 'bg-blue-500 text-white shadow-sm'
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

	<!-- Server Room Cards Grid -->
	<div class="flex-1 p-6">
		{#if errorMessage}
			<div class="mb-4 flex items-center gap-2 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-600 dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-300">
				<AlertTriangle class="h-4 w-4" />
				<span>{errorMessage}</span>
			</div>
		{/if}
		{#if loading}
			{#if viewMode === 'grid'}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
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
		{:else if filteredServerRooms.length === 0}
			<div class="flex h-64 items-center justify-center">
				<div class="text-center">
					<Server class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
					<p class="text-gray-500 dark:text-gray-400">
						{searchQuery ? '검색 결과가 없습니다.' : '등록된 서버실이 없습니다.'}
					</p>
					{#if !searchQuery}
						<Button
							on:click={openCreateDialog}
							class="mt-4 bg-blue-500 text-white hover:bg-blue-600 {isDesktop ? 'px-3 py-1.5 text-xs' : ''}"
						>
							<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							첫 서버실 추가하기
						</Button>
					{/if}
				</div>
			</div>
		{:else}
			{#if viewMode === 'grid'}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
					{#each filteredServerRooms.slice((page - 1) * limit, (page - 1) * limit + limit) as serverRoom (serverRoom.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white transition-all hover:-translate-y-1 hover:border-blue-300 hover:shadow-lg focus-within:border-blue-400 focus-within:ring-2 focus-within:ring-blue-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-blue-500/60 dark:focus-within:ring-blue-500/20"
							role="button"
							tabindex="0"
							on:click={() => openServerRoomDetail(serverRoom)}
							on:keydown={(event) => handleCardKey(event, serverRoom)}
						>
							<div class="border-b border-gray-200 p-4 dark:border-gray-700">
								<div class="flex items-start justify-between gap-3">
									<div class="flex items-start gap-3">
										<div
											class="{isDesktop
												? 'h-8 w-8'
												: 'h-10 w-10'} flex items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30"
										>
											<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
										</div>
										<div>
											<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
												{serverRoom.name}
											</h3>
											<button
												type="button"
												on:click|stopPropagation={() => navigateToOffice(serverRoom.office_id)}
												class="{isDesktop
													? 'text-[10px]'
													: 'text-xs'} mt-0.5 flex items-center gap-1 text-orange-600 transition-colors hover:text-orange-700 dark:text-orange-400 dark:hover:text-orange-300"
											>
												<Building class="h-2.5 w-2.5" />
												{serverRoom.office_name}
											</button>
											{#if serverRoom.description}
												<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} mt-1 line-clamp-2 text-gray-500 dark:text-gray-400">
													{serverRoom.description}
												</p>
											{/if}
										</div>
									</div>
									<span class="rounded-full bg-blue-100 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-blue-600 dark:bg-blue-900/30 dark:text-blue-300">
										서버실
									</span>
								</div>
							</div>

							<div class="space-y-3 p-4">
								{#if serverRoom.floor_level || serverRoom.room_number}
									<div class="space-y-1 text-xs text-gray-600 dark:text-gray-300">
										{#if serverRoom.floor_level}
											<p>
												<span class="font-medium">층수:</span> {serverRoom.floor_level}
											</p>
										{/if}
										{#if serverRoom.room_number}
											<p>
												<span class="font-medium">호실:</span> {serverRoom.room_number}
											</p>
										{/if}
									</div>
								{/if}

								<div class="flex flex-wrap gap-2 text-xs">
									{#if serverRoom.temperature_monitoring}
										<span class="flex items-center gap-1 rounded-full bg-red-100 px-2 py-1 text-red-700 dark:bg-red-900/30 dark:text-red-300">
											<Thermometer class="h-3 w-3" />
											온도
										</span>
									{/if}
									{#if serverRoom.humidity_monitoring}
										<span class="flex items-center gap-1 rounded-full bg-blue-100 px-2 py-1 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300">
											<Droplets class="h-3 w-3" />
											습도
										</span>
									{/if}
									{#if serverRoom.access_control}
										<span class="flex items-center gap-1 rounded-full bg-green-100 px-2 py-1 text-green-700 dark:bg-green-900/30 dark:text-green-300">
											<Shield class="h-3 w-3" />
											출입통제
										</span>
									{/if}
									{#if !serverRoom.temperature_monitoring && !serverRoom.humidity_monitoring && !serverRoom.access_control}
										<span class="rounded-full bg-gray-100 px-2 py-1 text-[11px] text-gray-500 dark:bg-gray-800 dark:text-gray-400">
											등록된 모니터링 없음
										</span>
									{/if}
								</div>

								<div class="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-300">
									<Calendar class="h-3 w-3 text-gray-400" />
									등록일: {formatDate(serverRoom.created_at)}
								</div>
							</div>

							<div class="border-t border-gray-200 bg-gray-50 px-4 py-3 dark:border-gray-700 dark:bg-gray-900/50">
								<div class="flex items-center justify-between">
									<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-blue-500 dark:text-gray-400 dark:group-hover:text-blue-300">
										상세 보기
										<ChevronRight class="h-4 w-4" />
									</span>
									<div class="flex items-center gap-2">
										<button
											type="button"
											on:click|stopPropagation={() => openEditDialog(serverRoom)}
											class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
											title="수정"
										>
											<Edit class="h-4 w-4" />
										</button>
										<button
											type="button"
											on:click|stopPropagation={() => deleteServerRoom(serverRoom)}
											class="rounded-md p-1 text-red-600 transition hover:bg-red-50 hover:text-red-700 focus:outline-none focus:ring-2 focus:ring-red-200 dark:text-red-300 dark:hover:bg-red-900/30 dark:hover:text-red-200 dark:focus:ring-red-500/40"
											title="삭제"
										>
											<Trash2 class="h-4 w-4" />
										</button>
										<button
											type="button"
											on:click|stopPropagation={() => navigateToOfficeServers(serverRoom.office_id)}
											class="rounded-md p-1 text-gray-600 transition hover:bg-gray-50 hover:text-gray-800 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-300 dark:hover:bg-gray-900/40 dark:hover:text-gray-100 dark:focus:ring-gray-600/40"
											title="사무실 서버실 관리로 이동"
										>
											<ArrowRight class="h-4 w-4" />
										</button>
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="space-y-3">
					{#each filteredServerRooms.slice((page - 1) * limit, (page - 1) * limit + limit) as serverRoom (serverRoom.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white p-4 transition-all hover:-translate-y-[2px] hover:border-blue-300 hover:shadow-md focus-within:border-blue-400 focus-within:ring-2 focus-within:ring-blue-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-blue-500/60 dark:focus-within:ring-blue-500/20"
							role="button"
							tabindex="0"
							on:click={() => openServerRoomDetail(serverRoom)}
							on:keydown={(event) => handleCardKey(event, serverRoom)}
						>
							<div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
								<div class="flex items-start gap-3">
									<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30">
										<Server class="h-5 w-5 text-blue-500" />
									</div>
									<div>
										<h3 class="text-base font-semibold text-gray-900 dark:text-white">
											{serverRoom.name}
										</h3>
										<p class="text-sm text-gray-500 dark:text-gray-400">{serverRoom.office_name}</p>
										{#if serverRoom.description}
											<p class="mt-1 text-xs text-gray-400 dark:text-gray-500 line-clamp-2">{serverRoom.description}</p>
										{/if}
										<div class="mt-2 flex flex-wrap gap-1 text-[11px]">
											{#if serverRoom.temperature_monitoring}
												<span class="rounded-full bg-red-100 px-2 py-0.5 text-red-600 dark:bg-red-900/30 dark:text-red-300">온도</span>
											{/if}
											{#if serverRoom.humidity_monitoring}
												<span class="rounded-full bg-blue-100 px-2 py-0.5 text-blue-600 dark:bg-blue-900/30 dark:text-blue-300">습도</span>
											{/if}
											{#if serverRoom.access_control}
												<span class="rounded-full bg-green-100 px-2 py-0.5 text-green-600 dark:bg-green-900/30 dark:text-green-300">출입통제</span>
											{/if}
										</div>
									</div>
								</div>
								<div class="flex items-center gap-2 md:flex-col md:items-end md:gap-1 text-xs text-gray-500 dark:text-gray-400">
									<span class="flex items-center gap-1">
										<Calendar class="h-3 w-3" />
										{formatDate(serverRoom.created_at)}
									</span>
									<button
										type="button"
										on:click|stopPropagation={() => navigateToOffice(serverRoom.office_id)}
										class="flex items-center gap-1 rounded-full border border-orange-200 px-2 py-0.5 text-[10px] font-semibold text-orange-600 transition hover:border-orange-300 hover:text-orange-500 dark:border-orange-900/40 dark:text-orange-300 dark:hover:border-orange-700/60"
									>
										<Building class="h-3 w-3" />
										사무실 이동
									</button>
								</div>
							</div>
							<div class="mt-4 flex items-center justify-between">
								<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-blue-500 dark:text-gray-400 dark:group-hover:text-blue-300">
									상세 보기
									<ChevronRight class="h-4 w-4" />
								</span>
								<div class="flex items-center gap-2">
									<button
										type="button"
										on:click|stopPropagation={() => openEditDialog(serverRoom)}
										class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
										title="수정"
									>
										<Edit class="h-4 w-4" />
									</button>
									<button
										type="button"
										on:click|stopPropagation={() => deleteServerRoom(serverRoom)}
										class="rounded-md p-1 text-red-600 transition hover:bg-red-50 hover:text-red-700 focus:outline-none focus:ring-2 focus:ring-red-200 dark:text-red-300 dark:hover:bg-red-900/30 dark:hover:text-red-200 dark:focus:ring-red-500/40"
										title="삭제"
									>
										<Trash2 class="h-4 w-4" />
									</button>
									<button
										type="button"
										on:click|stopPropagation={() => navigateToOfficeServers(serverRoom.office_id)}
										class="rounded-md p-1 text-gray-600 transition hover:bg-gray-50 hover:text-gray-800 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-300 dark:hover:bg-gray-900/40 dark:hover:text-gray-100 dark:focus:ring-gray-600/40"
										title="사무실 서버실 관리로 이동"
									>
										<ArrowRight class="h-4 w-4" />
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
							page = Math.max(1, page - 1);
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
							page = Math.min(totalPages, page + 1);
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

<!-- Server Room Form Dialog -->
<ServerRoomFormDialogWithOfficeSelector
	open={showDialog}
	{offices}
	bind:selectedOfficeId
	serverRoom={editingServerRoom}
	onClose={closeDialog}
	onSuccess={handleFormSuccess}
/>
