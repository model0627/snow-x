<script lang="ts">
	import {
		Server, Building, Plus, Edit, Trash2, Calendar, Search,
		Thermometer, Droplets, Shield, MoreVertical, ArrowRight, Eye
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { serverRoomApi, officeApi, type ServerRoom, type Office } from '$lib/api/office';
	import ServerRoomFormDialogWithOfficeSelector from '$lib/components/office/ServerRoomFormDialogWithOfficeSelector.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let searchQuery = $state('');
	let loading = $state(false);
	let offices = $state<Office[]>([]);
	let serverRooms = $state<ServerRoom[]>([]);
	let total = $state(0);
	let page = $state(1);
	let limit = $state(20);

	// Dialog state
	let showDialog = $state(false);
	let editingServerRoom = $state<ServerRoom | null>(null);
	let selectedOfficeId = $state<string>('');

	const isDesktop = $derived(desktopStore.isDesktop);

	// Load all offices and their server rooms
	async function loadData() {
		loading = true;
		try {
			// First get all offices
			const officesResponse = await officeApi.getOffices({
				page: 1,
				limit: 100 // Get all offices
			});
			offices = officesResponse.offices;

			// Then get server rooms for each office
			const allServerRooms: (ServerRoom & { office_name: string })[] = [];

			for (const office of offices) {
				try {
					const serverRoomsResponse = await serverRoomApi.getServerRooms(office.id, {
						page: 1,
						limit: 100 // Get all server rooms for this office
					});

					// Add office name to each server room
					const serverRoomsWithOffice = serverRoomsResponse.server_rooms.map(room => ({
						...room,
						office_name: office.name
					}));

					allServerRooms.push(...serverRoomsWithOffice);
				} catch (error) {
					console.error(`Failed to load server rooms for office ${office.name}:`, error);
				}
			}

			// Filter by search query if provided
			if (searchQuery.trim()) {
				const filtered = allServerRooms.filter(room =>
					room.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
					room.office_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
					(room.description && room.description.toLowerCase().includes(searchQuery.toLowerCase()))
				);
				serverRooms = filtered;
				total = filtered.length;
			} else {
				serverRooms = allServerRooms;
				total = allServerRooms.length;
			}
		} catch (error) {
			console.error('Failed to load data:', error);
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
			loadData();
		}, 300);
	});

	onMount(() => {
		loadData();
	});
</script>

<div class="flex-1 min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="px-6 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
					<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">전체 서버실</h1>
				</div>
				<Button
					onclick={openCreateDialog}
					class="bg-blue-500 hover:bg-blue-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
				>
					<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
					새 서버실 추가
				</Button>
			</div>
			<p class="mt-2 {isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400">모든 사무실의 서버실을 한 번에 확인할 수 있습니다.</p>
		</div>
	</div>

	<!-- Search Bar -->
	<div class="px-6 py-4 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="relative max-w-md">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="서버실명 또는 사무실명으로 검색..."
				class="w-full pl-10 pr-4 py-2 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
					bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-white
					focus:outline-none focus:ring-2 focus:ring-blue-500"
			/>
		</div>
	</div>

	<!-- Server Room Cards Grid -->
	<div class="flex-1 p-6">
		{#if loading}
			<div class="flex items-center justify-center h-64">
				<div class="text-gray-500 dark:text-gray-400">로딩 중...</div>
			</div>
		{:else if serverRooms.length === 0}
			<div class="flex items-center justify-center h-64">
				<div class="text-center">
					<Server class="h-12 w-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
					<p class="text-gray-500 dark:text-gray-400">
						{searchQuery ? '검색 결과가 없습니다.' : '등록된 서버실이 없습니다.'}
					</p>
					{#if !searchQuery}
						<Button
							onclick={openCreateDialog}
							class="mt-4 bg-blue-500 hover:bg-blue-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
						>
							<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							첫 서버실 추가하기
						</Button>
					{/if}
				</div>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each serverRooms as serverRoom (serverRoom.id)}
					<div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:shadow-lg transition-shadow">
						<!-- Card Header -->
						<div class="p-4 border-b border-gray-200 dark:border-gray-700">
							<div class="flex items-start justify-between">
								<div class="flex items-start gap-3">
									<div class="{isDesktop ? 'w-8 h-8' : 'w-10 h-10'} bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center">
										<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
									</div>
									<div class="flex-1">
										<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">{serverRoom.name}</h3>
										<button
											onclick={() => navigateToOffice(serverRoom.office_id)}
											class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-orange-600 dark:text-orange-400 hover:text-orange-700 dark:hover:text-orange-300 mt-0.5 flex items-center gap-1 transition-colors"
										>
											<Building class="h-2.5 w-2.5" />
											{serverRoom.office_name}
										</button>
										{#if serverRoom.description}
											<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-gray-500 dark:text-gray-400 mt-1">{serverRoom.description}</p>
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
						<div class="p-4 space-y-3">
							{#if serverRoom.floor_level || serverRoom.room_number}
								<div class="space-y-1">
									{#if serverRoom.floor_level}
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
											<span class="font-medium">층수:</span> {serverRoom.floor_level}
										</p>
									{/if}
									{#if serverRoom.room_number}
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">
											<span class="font-medium">호실:</span> {serverRoom.room_number}
										</p>
									{/if}
								</div>
							{/if}

							<!-- Monitoring Features -->
							<div class="flex flex-wrap gap-2">
								{#if serverRoom.temperature_monitoring}
									<div class="flex items-center gap-1 px-2 py-1 bg-red-100 dark:bg-red-900/30 rounded-full">
										<Thermometer class="{isDesktop ? 'h-2.5 w-2.5' : 'h-3 w-3'} text-red-500" />
										<span class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-red-700 dark:text-red-400">온도</span>
									</div>
								{/if}
								{#if serverRoom.humidity_monitoring}
									<div class="flex items-center gap-1 px-2 py-1 bg-blue-100 dark:bg-blue-900/30 rounded-full">
										<Droplets class="{isDesktop ? 'h-2.5 w-2.5' : 'h-3 w-3'} text-blue-500" />
										<span class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-blue-700 dark:text-blue-400">습도</span>
									</div>
								{/if}
								{#if serverRoom.access_control}
									<div class="flex items-center gap-1 px-2 py-1 bg-green-100 dark:bg-green-900/30 rounded-full">
										<Shield class="{isDesktop ? 'h-2.5 w-2.5' : 'h-3 w-3'} text-green-500" />
										<span class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-green-700 dark:text-green-400">출입통제</span>
									</div>
								{/if}
							</div>

							<div class="flex items-center gap-2">
								<Calendar class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-300">등록일: {formatDate(serverRoom.created_at)}</p>
							</div>
						</div>

						<!-- Card Footer -->
						<div class="px-4 py-3 bg-gray-50 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-700">
							<div class="flex items-center justify-between">
								<div class="space-y-1">
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">랙: 0개</p>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">디바이스: 0개</p>
								</div>
								<div class="flex items-center gap-2">
									<button
										onclick={() => goto(`/ipam/server-rooms/${serverRoom.id}?office=${serverRoom.office_id}`)}
										class="{isDesktop ? 'text-xs' : 'text-sm'} text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 transition-colors"
										title="상세 보기"
									>
										<Eye class="h-3 w-3" />
									</button>
									<button
										onclick={() => openEditDialog(serverRoom)}
										class="{isDesktop ? 'text-xs' : 'text-sm'} text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
										title="수정"
									>
										<Edit class="h-3 w-3" />
									</button>
									<button
										class="{isDesktop ? 'text-xs' : 'text-sm'} text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 transition-colors"
										onclick={() => deleteServerRoom(serverRoom)}
										title="삭제"
									>
										<Trash2 class="h-3 w-3" />
									</button>
									<button
										onclick={() => navigateToOfficeServers(serverRoom.office_id)}
										class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 transition-colors flex items-center gap-1"
										title="사무실 서버실 관리로 이동"
									>
										<span>사무실</span>
										<ArrowRight class="h-3 w-3" />
									</button>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>

			<!-- Summary -->
			{#if total > 0}
				<div class="flex items-center justify-center mt-6">
					<div class="text-center">
						<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400">
							총 {total}개의 서버실 ({offices.length}개 사무실)
						</p>
					</div>
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