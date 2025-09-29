<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		Server, ArrowLeft, Plus, Edit, Trash2, Calendar,
		Thermometer, Droplets, Shield, Building, MoreVertical, Eye
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { serverRoomApi, type ServerRoom, officeApi, type Office } from '$lib/api/office';
	import ServerRoomFormDialog from '$lib/components/office/ServerRoomFormDialog.svelte';
	import { onMount } from 'svelte';

	// Get office ID from route params
	const officeId = $page.params.id;

	let office = $state<Office | null>(null);
	let serverRooms = $state<ServerRoom[]>([]);
	let loading = $state(true);
	let serverRoomsLoading = $state(false);
	let total = $state(0);
	let page_num = $state(1);
	let limit = $state(20);

	// Dialog state
	let showDialog = $state(false);
	let editingServerRoom = $state<ServerRoom | null>(null);

	const isDesktop = $derived(desktopStore.isDesktop);

	// Load office details
	async function loadOffice() {
		try {
			office = await officeApi.getOffice(officeId);
		} catch (error) {
			console.error('Failed to load office:', error);
			goto('/ipam/offices');
		}
	}

	// Load server rooms from API
	async function loadServerRooms() {
		serverRoomsLoading = true;
		try {
			const response = await serverRoomApi.getServerRooms(officeId, {
				page: page_num,
				limit
			});
			serverRooms = response.server_rooms;
			total = response.total;
		} catch (error) {
			console.error('Failed to load server rooms:', error);
		} finally {
			serverRoomsLoading = false;
		}
	}

	// Delete server room
	async function deleteServerRoom(serverRoom: ServerRoom) {
		if (confirm(`"${serverRoom.name}" 서버실을 삭제하시겠습니까?`)) {
			try {
				await serverRoomApi.deleteServerRoom(officeId, serverRoom.id);
				await loadServerRooms(); // Reload the list
			} catch (error) {
				console.error('Failed to delete server room:', error);
				alert('서버실 삭제에 실패했습니다.');
			}
		}
	}

	// Open create dialog
	function openCreateDialog() {
		editingServerRoom = null;
		showDialog = true;
	}

	// Open edit dialog
	function openEditDialog(serverRoom: ServerRoom) {
		editingServerRoom = serverRoom;
		showDialog = true;
	}

	// Close dialog
	function closeDialog() {
		showDialog = false;
		editingServerRoom = null;
	}

	// Handle successful form submission
	function handleFormSuccess() {
		loadServerRooms();
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

	onMount(async () => {
		loading = true;
		await Promise.all([loadOffice(), loadServerRooms()]);
		loading = false;
	});
</script>

{#if loading}
	<div class="flex-1 flex items-center justify-center min-h-screen">
		<div class="text-gray-500 dark:text-gray-400">로딩 중...</div>
	</div>
{:else if office}
	<div class="flex-1 min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
		<!-- Header -->
		<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
			<div class="px-6 py-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-3">
						<button
							onclick={() => goto(`/ipam/offices/${officeId}`)}
							class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
							title="사무실로 돌아가기"
						>
							<ArrowLeft class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-gray-600 dark:text-gray-400" />
						</button>
						<Building class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-orange-500" />
						<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">{office.name}</span>
						<span class="text-gray-300 dark:text-gray-600">/</span>
						<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
						<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">서버실 관리</h1>
					</div>
					<Button
						onclick={openCreateDialog}
						class="bg-blue-500 hover:bg-blue-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
					>
						<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
						새 서버실 추가
					</Button>
				</div>
				<p class="mt-2 {isDesktop ? 'text-xs' : 'text-sm'} text-gray-600 dark:text-gray-400">{office.name}의 서버실을 관리합니다.</p>
			</div>
		</div>

		<!-- Server Room Cards Grid -->
		<div class="flex-1 p-6">
			{#if serverRoomsLoading}
				<div class="flex items-center justify-center h-64">
					<div class="text-gray-500 dark:text-gray-400">로딩 중...</div>
				</div>
			{:else if serverRooms.length === 0}
				<div class="flex items-center justify-center h-64">
					<div class="text-center">
						<Server class="h-12 w-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
						<p class="text-gray-500 dark:text-gray-400">등록된 서버실이 없습니다.</p>
						<Button
							onclick={openCreateDialog}
							class="mt-4 bg-blue-500 hover:bg-blue-600 text-white {isDesktop ? 'text-xs px-3 py-1.5' : ''}"
						>
							<Plus class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							첫 서버실 추가하기
						</Button>
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
											{#if serverRoom.description}
												<p class="{isDesktop ? 'text-[10px]' : 'text-xs'} text-gray-500 dark:text-gray-400 mt-0.5">{serverRoom.description}</p>
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
											onclick={() => goto(`/ipam/server-rooms/${serverRoom.id}?office=${officeId}`)}
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
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>

				<!-- Pagination -->
				{#if total > limit}
					<div class="flex items-center justify-center mt-6 space-x-2">
						<button
							disabled={page_num === 1}
							onclick={() => { page_num--; loadServerRooms(); }}
							class="px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white disabled:opacity-50"
						>
							이전
						</button>
						<span class="text-sm text-gray-600 dark:text-gray-400">
							{page_num} / {Math.ceil(total / limit)}
						</span>
						<button
							disabled={page_num >= Math.ceil(total / limit)}
							onclick={() => { page_num++; loadServerRooms(); }}
							class="px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white disabled:opacity-50"
						>
							다음
						</button>
					</div>
				{/if}
			{/if}
		</div>
	</div>

	<!-- Server Room Form Dialog -->
	<ServerRoomFormDialog
		open={showDialog}
		{officeId}
		serverRoom={editingServerRoom}
		onClose={closeDialog}
		onSuccess={handleFormSuccess}
	/>
{/if}