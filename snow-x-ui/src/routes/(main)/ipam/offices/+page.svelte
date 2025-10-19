<script lang="ts">
	import {
		AlertTriangle,
		BarChart3,
		Building,
		Calendar,
			Clock,
			ChevronRight,
			Edit,
			LayoutGrid,
			MapPin,
			List,
		Plus,
		Search,
		Trash2,
		X
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { officeApi, type Office } from '$lib/api/office';
	import { goto } from '$app/navigation';
		import OfficeFormDialog from '$lib/components/office/OfficeFormDialog.svelte';
		import { onMount } from 'svelte';
		import { browser } from '$app/environment';

	const SEARCH_DEBOUNCE = 150;

		const VIEW_MODE_STORAGE_KEY = 'ipam-offices-view-mode';

		let searchQuery = $state('');
	let loading = $state(false);
	let errorMessage = $state<string | null>(null);
	let offices = $state<Office[]>([]);
	let total = $state(0);
	let page = $state(1);
	let limit = $state(20);
	let viewMode = $state<'grid' | 'list'>('grid');

	// Dialog state
	let showDialog = $state(false);
	let editingOffice = $state<Office | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);
	const totalPages = $derived(Math.max(1, Math.ceil(total / limit)));
	const skeletonItems = $derived(Array.from({ length: viewMode === 'grid' ? (isDesktop ? 6 : 4) : 4 }, (_, index) => index));
	const showingRange = $derived(
		total
			? `${((page - 1) * limit + 1).toLocaleString('ko-KR')} - ${Math.min(total, page * limit).toLocaleString('ko-KR')}`
			: '0 - 0'
	);

	function computeLastUpdated(items: Office[]): string | null {
		if (!items.length) {
			return null;
		}
		const latest = items.reduce((current, office) => {
			const updatedAt = new Date(office.updated_at ?? office.created_at);
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
	const lastUpdated = $derived(computeLastUpdated(offices));

	// 인증 상태는 상위 레이아웃에서 처리하므로 제거

	// Load offices from API
	async function loadOffices() {
		loading = true;
		errorMessage = null;
		try {
			const response = await officeApi.getOffices({
				page,
				limit,
				search: searchQuery || undefined
			});
			offices = response.offices;
			total = response.total;
		} catch (error) {
			console.error('Failed to load offices:', error);
			errorMessage = '사무실 정보를 불러오지 못했습니다.';
		} finally {
			loading = false;
		}
	}

	// Delete office
	async function deleteOffice(office: Office) {
		if (confirm(`"${office.name}" 사무실을 삭제하시겠습니까?`)) {
			try {
				await officeApi.deleteOffice(office.id);
				await loadOffices(); // Reload the list
			} catch (error) {
				console.error('Failed to delete office:', error);
				alert('사무실 삭제에 실패했습니다.');
			}
		}
	}

	// Open create dialog
	function openCreateDialog() {
		editingOffice = null;
		showDialog = true;
	}

	// Open edit dialog
	function openEditDialog(office: Office) {
		editingOffice = office;
		showDialog = true;
	}

	// Close dialog
	function closeDialog() {
		showDialog = false;
		editingOffice = null;
	}

	// Handle successful form submission
	function handleFormSuccess() {
		loadOffices();
	}

		function openOfficeDetail(id: string) {
			goto(`/ipam/offices/${id}`);
		}

		function handleCardKey(event: KeyboardEvent, id: string) {
			if (event.key === 'Enter' || event.key === ' ') {
				event.preventDefault();
				openOfficeDetail(id);
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

	function handleLimitChange(event: Event) {
		const value = Number((event.target as HTMLSelectElement).value);
		if (!Number.isNaN(value) && value !== limit) {
			limit = value;
			page = 1;
			loadOffices();
		}
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
			page = 1; // Reset to first page on search
			loadOffices();
		}, SEARCH_DEBOUNCE);
		return () => {
			if (searchTimeout) clearTimeout(searchTimeout);
		};
	});

	function clearSearch() {
		if (searchQuery) {
			searchQuery = '';
			page = 1;
			loadOffices();
		}
	}

		onMount(() => {
			if (browser) {
				const storedMode = window.localStorage.getItem(VIEW_MODE_STORAGE_KEY);
				if (storedMode === 'grid' || storedMode === 'list') {
					viewMode = storedMode;
				}
			}

			loadOffices();
		});
</script>

<div class="flex min-h-screen flex-1 flex-col bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
		<div class="px-6 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<Building class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-orange-500" />
					<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">사무실 관리</h1>
				</div>
				<Button
					on:click={openCreateDialog}
					class="bg-orange-500 text-white hover:bg-orange-600 {isDesktop ? 'px-3 py-1.5 text-xs' : ''}"
				>
					<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
					새 사무실 추가
				</Button>
			</div>
			<p class="mt-2 {isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400">
				조직의 사무실을 관리합니다.
			</p>
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
					placeholder="사무실명 또는 주소로 검색..."
					class="w-full rounded-lg border border-gray-300 bg-gray-50 py-2 pl-10 pr-4 text-sm text-gray-900 focus:ring-2 focus:ring-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-900 dark:text-white"
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
					표시 구간 {showingRange} / 총 {total.toLocaleString('ko-KR')}개
				</span>
				<label class="flex items-center gap-2">
					<span class="text-xs uppercase tracking-wide text-gray-500 dark:text-gray-400">Page size</span>
					<select
						class="rounded-md border border-gray-300 bg-white px-2 py-1 text-xs font-medium text-gray-700 focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-200"
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
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-orange-500 {viewMode === 'grid'
							? 'bg-orange-500 text-white shadow-sm'
							: 'hover:text-gray-800 dark:hover:text-gray-200'}"
						on:click={() => setViewMode('grid')}
						aria-pressed={viewMode === 'grid'}
					>
						<LayoutGrid class="h-3.5 w-3.5" />
						<span class="hidden sm:inline">그리드</span>
					</button>
					<button
						type="button"
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-orange-500 {viewMode === 'list'
							? 'bg-orange-500 text-white shadow-sm'
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

	<!-- Stats -->
	<div class="bg-gray-50 px-6 py-4 dark:bg-gray-900">
		<div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
			<div class="flex items-center gap-3 rounded-lg border border-orange-200 bg-orange-50/60 p-4 dark:border-orange-900/40 dark:bg-orange-900/20">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-orange-500/10 text-orange-600 dark:text-orange-300">
					<BarChart3 class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-orange-500/80 dark:text-orange-300/80">총 사무실</p>
					<p class="text-lg font-semibold text-gray-900 dark:text-white">{total.toLocaleString('ko-KR')}</p>
				</div>
			</div>
			<div class="flex items-center gap-3 rounded-lg border border-sky-200 bg-sky-50/60 p-4 dark:border-sky-900/40 dark:bg-sky-900/20">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-sky-500/10 text-sky-600 dark:text-sky-300">
					<Clock class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-sky-500/80 dark:text-sky-300/80">마지막 업데이트</p>
					<p class="text-sm font-semibold text-gray-900 dark:text-white">{lastUpdated ?? '데이터 없음'}</p>
				</div>
			</div>
			<div class="flex items-center gap-3 rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-200">
					<Building class="h-5 w-5" />
				</div>
				<div>
					<p class="text-xs font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">현재 페이지</p>
					<p class="text-sm font-semibold text-gray-900 dark:text-white">
						{page} / {totalPages}
					</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Office Cards Grid -->
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
		{:else if offices.length === 0}
			<div class="flex h-64 items-center justify-center">
				<div class="text-center">
					<Building class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
					<p class="text-gray-500 dark:text-gray-400">등록된 사무실이 없습니다.</p>
					{#if searchQuery}
						<p class="mt-1 text-sm text-gray-400 dark:text-gray-500">검색 결과가 없습니다.</p>
					{/if}
				</div>
			</div>
		{:else}
			{#if viewMode === 'grid'}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
					{#each offices as office (office.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white transition-all hover:-translate-y-1 hover:border-orange-300 hover:shadow-lg focus-within:border-orange-400 focus-within:ring-2 focus-within:ring-orange-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-orange-500/60 dark:focus-within:ring-orange-500/20"
							role="button"
							tabindex="0"
							on:click={() => openOfficeDetail(office.id)}
							on:keydown={(event) => handleCardKey(event, office.id)}
						>
							<div class="border-b border-gray-200 p-4 dark:border-gray-700">
								<div class="flex items-start justify-between gap-3">
									<div class="flex items-start gap-3">
										<div
											class="{isDesktop
												? 'h-8 w-8'
												: 'h-10 w-10'} flex items-center justify-center rounded-lg bg-orange-100 dark:bg-orange-900/30"
										>
											<Building class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-orange-500" />
										</div>
										<div class="flex-1">
											<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
												{office.name}
											</h3>
											{#if office.description}
												<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} mt-0.5 line-clamp-2 text-gray-500 dark:text-gray-400">
													{office.description}
												</p>
											{/if}
										</div>
									</div>
									<span
										class="rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide {office.is_active
											? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300'
											: 'bg-rose-100 text-rose-600 dark:bg-rose-900/20 dark:text-rose-300'}"
									>
										{office.is_active ? '활성' : '비활성'}
									</span>
								</div>
							</div>

							<div class="space-y-3 p-4">
								<div class="flex items-start gap-2">
									<MapPin class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mt-0.5 text-gray-400" />
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300 line-clamp-2">
										{office.address}
									</p>
								</div>

								<div class="flex items-center gap-2">
									<Calendar class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
										등록일: {formatDate(office.created_at)}
									</p>
								</div>

								{#if office.contact_person || office.phone || office.email}
									<div class="space-y-1 border-t border-gray-200 pt-2 text-xs dark:border-gray-700">
										{#if office.contact_person}
											<p class="text-gray-600 dark:text-gray-300">
												<span class="font-medium">담당자:</span>
												{office.contact_person}
											</p>
										{/if}
										{#if office.phone}
											<p class="text-gray-600 dark:text-gray-300">
												<span class="font-medium">전화:</span>
												{office.phone}
											</p>
										{/if}
										{#if office.email}
											<p class="text-gray-600 dark:text-gray-300">
												<span class="font-medium">이메일:</span>
												{office.email}
											</p>
										{/if}
									</div>
								{/if}
							</div>

							<div class="border-t border-gray-200 bg-gray-50 px-4 py-3 dark:border-gray-700 dark:bg-gray-900/50">
								<div class="flex items-center justify-between">
									<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-orange-500 dark:text-gray-400 dark:group-hover:text-orange-300">
										상세 보기
										<ChevronRight class="h-4 w-4" />
									</span>
									<div class="flex items-center gap-2">
										<button
											on:click|stopPropagation={() => openEditDialog(office)}
											class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
											title="수정"
										>
											<Edit class="h-4 w-4" />
										</button>
										<button
											on:click|stopPropagation={() => deleteOffice(office)}
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
					{#each offices as office (office.id)}
						<div
							class="group rounded-lg border border-gray-200 bg-white p-4 transition-all hover:-translate-y-[2px] hover:border-orange-300 hover:shadow-md focus-within:border-orange-400 focus-within:ring-2 focus-within:ring-orange-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-orange-500/60 dark:focus-within:ring-orange-500/20"
							role="button"
							tabindex="0"
							on:click={() => openOfficeDetail(office.id)}
							on:keydown={(event) => handleCardKey(event, office.id)}
						>
							<div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
								<div class="flex items-start gap-3">
									<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100 dark:bg-orange-900/30">
										<Building class="h-5 w-5 text-orange-500" />
									</div>
									<div>
										<h3 class="text-base font-semibold text-gray-900 dark:text-white">
											{office.name}
										</h3>
										<p class="text-sm text-gray-500 dark:text-gray-400">{office.address}</p>
										{#if office.description}
											<p class="mt-1 text-xs text-gray-400 dark:text-gray-500 line-clamp-2">{office.description}</p>
										{/if}
									</div>
								</div>
								<div class="flex items-center gap-2 md:flex-col md:items-end md:gap-1">
									<span
										class="rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide {office.is_active
											? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300'
											: 'bg-rose-100 text-rose-600 dark:bg-rose-900/20 dark:text-rose-300'}"
									>
										{office.is_active ? '활성' : '비활성'}
									</span>
									<span class="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400">
										<Calendar class="h-3 w-3" />
										{formatDate(office.created_at)}
									</span>
								</div>
							</div>
							{#if office.contact_person || office.phone || office.email}
								<div class="mt-4 grid gap-2 text-xs text-gray-600 dark:text-gray-300 md:grid-cols-3">
									{#if office.contact_person}
										<div><span class="font-medium">담당자:</span> {office.contact_person}</div>
									{/if}
									{#if office.phone}
										<div><span class="font-medium">전화:</span> {office.phone}</div>
									{/if}
									{#if office.email}
										<div><span class="font-medium">이메일:</span> {office.email}</div>
									{/if}
								</div>
							{/if}
							<div class="mt-4 flex items-center justify-between">
								<span class="flex items-center gap-1 text-sm text-gray-600 transition group-hover:text-orange-500 dark:text-gray-400 dark:group-hover:text-orange-300">
									상세 보기
									<ChevronRight class="h-4 w-4" />
								</span>
								<div class="flex items-center gap-2">
									<button
										on:click|stopPropagation={() => openEditDialog(office)}
										class="rounded-md p-1 text-blue-600 transition hover:bg-blue-50 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-200 dark:text-blue-300 dark:hover:bg-blue-900/30 dark:hover:text-blue-200 dark:focus:ring-blue-500/40"
										title="수정"
									>
										<Edit class="h-4 w-4" />
									</button>
									<button
										on:click|stopPropagation={() => deleteOffice(office)}
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

			<!-- Pagination (if needed) -->
			{#if totalPages > 1}
				<div class="mt-6 flex items-center justify-center space-x-2">
					<button
						disabled={page === 1}
						on:click={() => {
							page--;
							loadOffices();
						}}
						class="px-3 py-2 text-sm text-gray-600 hover:text-gray-900 disabled:opacity-50 dark:text-gray-400 dark:hover:text-white"
					>
						이전
					</button>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{page} / {totalPages}
					</span>
					<button
						disabled={page >= totalPages}
						on:click={() => {
							page++;
							loadOffices();
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

<!-- Office Form Dialog -->
<OfficeFormDialog open={showDialog} office={editingOffice} onClose={closeDialog} onSuccess={handleFormSuccess} />
