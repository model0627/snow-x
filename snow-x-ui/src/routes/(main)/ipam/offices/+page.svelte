<script lang="ts">
	import { Building, MapPin, Calendar, Eye, Edit, Trash2, Plus, Search, MoreVertical } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { officeApi, type Office } from '$lib/api/office';
	import { goto } from '$app/navigation';
	import OfficeFormDialog from '$lib/components/office/OfficeFormDialog.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let searchQuery = $state('');
	let loading = $state(false);
	let offices = $state<Office[]>([]);
	let total = $state(0);
	let page = $state(1);
	let limit = $state(20);

	// Dialog state
	let showDialog = $state(false);
	let editingOffice = $state<Office | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);

	// 인증 상태는 상위 레이아웃에서 처리하므로 제거

	// Load offices from API
	async function loadOffices() {
		loading = true;
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
		}, 300);
	});

	onMount(() => {
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
					onclick={openCreateDialog}
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
		<div class="relative max-w-md">
			<Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400" />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="사무실명 또는 주소로 검색..."
				class="w-full py-2 pr-4 pl-10 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-gray-50
					text-gray-900 focus:ring-2 focus:ring-orange-500 focus:outline-none
					dark:border-gray-600 dark:bg-gray-900 dark:text-white"
			/>
		</div>
	</div>

	<!-- Office Cards Grid -->
	<div class="flex-1 p-6">
		{#if loading}
			<div class="flex h-64 items-center justify-center">
				<div class="text-gray-500 dark:text-gray-400">로딩 중...</div>
			</div>
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
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
				{#each offices as office (office.id)}
					<div
						class="rounded-lg border border-gray-200 bg-white transition-shadow hover:shadow-lg dark:border-gray-700 dark:bg-gray-800"
					>
						<!-- Card Header -->
						<div class="border-b border-gray-200 p-4 dark:border-gray-700">
							<div class="flex items-start justify-between">
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
											<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} mt-0.5 text-gray-500 dark:text-gray-400">
												{office.description}
											</p>
										{/if}
									</div>
								</div>
								<div class="relative">
									<button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
										<MoreVertical class="h-4 w-4" />
									</button>
								</div>
							</div>
						</div>

						<!-- Card Body -->
						<div class="space-y-3 p-4">
							<div class="flex items-start gap-2">
								<MapPin class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mt-0.5 text-gray-400" />
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">{office.address}</p>
							</div>

							<div class="flex items-center gap-2">
								<Calendar class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
									등록일: {formatDate(office.created_at)}
								</p>
							</div>

							{#if office.contact_person || office.phone || office.email}
								<div class="space-y-1 border-t border-gray-200 pt-2 dark:border-gray-700">
									{#if office.contact_person}
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
											<span class="font-medium">담당자:</span>
											{office.contact_person}
										</p>
									{/if}
									{#if office.phone}
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
											<span class="font-medium">전화:</span>
											{office.phone}
										</p>
									{/if}
									{#if office.email}
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
											<span class="font-medium">이메일:</span>
											{office.email}
										</p>
									{/if}
								</div>
							{/if}
						</div>

						<!-- Card Footer -->
						<div class="border-t border-gray-200 bg-gray-50 px-4 py-3 dark:border-gray-700 dark:bg-gray-900/50">
							<div class="flex items-center justify-between">
								<button
									onclick={() => goto(`/ipam/offices/${office.id}`)}
									class="{isDesktop
										? 'text-xs'
										: 'text-sm'} text-gray-600 transition-colors hover:text-orange-500 dark:text-gray-400"
								>
									상세 보기
								</button>
								<div class="flex items-center gap-2">
									<button
										onclick={() => openEditDialog(office)}
										class="{isDesktop
											? 'text-xs'
											: 'text-sm'} text-blue-600 transition-colors hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
										title="수정"
									>
										<Edit class="h-3 w-3" />
									</button>
									<button
										class="{isDesktop
											? 'text-xs'
											: 'text-sm'} text-red-600 transition-colors hover:text-red-700 dark:text-red-400 dark:hover:text-red-300"
										onclick={() => deleteOffice(office)}
										title="삭제"
									>
										<Trash2 class="h-3 w-3" />
									</button>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>

			<!-- Pagination (if needed) -->
			{#if total > limit}
				<div class="mt-6 flex items-center justify-center space-x-2">
					<button
						disabled={page === 1}
						onclick={() => {
							page--;
							loadOffices();
						}}
						class="px-3 py-2 text-sm text-gray-600 hover:text-gray-900 disabled:opacity-50 dark:text-gray-400 dark:hover:text-white"
					>
						이전
					</button>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{page} / {Math.ceil(total / limit)}
					</span>
					<button
						disabled={page >= Math.ceil(total / limit)}
						onclick={() => {
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
