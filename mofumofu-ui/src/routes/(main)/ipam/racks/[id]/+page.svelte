<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		ArrowLeft, Server, HardDrive, Edit, Trash2, Settings,
		Thermometer, Building, Calendar, Zap, Snowflake, MapPin, Eye
	} from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { privateApi } from '$lib/api/private';
	import RackVisualizer from '$lib/components/rack/RackVisualizer.svelte';

	// Mock rack interface - will be replaced with actual API types
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
		is_active: boolean;
		server_room_name: string;
		office_name: string;
		device_count: number;
		usage_percentage: number;
		used_units: number;
	}

	let rack: Rack | null = $state(null);
	let isLoading = $state(true);
	let error = $state('');

	// Mock devices data (placeholder - will be fetched from API later)
	let devices = $state([
		{
			id: 'dev1',
			name: 'DB Server 01',
			type: 'server',
			height: 2,
			position: 40
		},
		{
			id: 'dev2',
			name: 'Web Server 01',
			type: 'server',
			height: 1,
			position: 38
		},
		{
			id: 'dev3',
			name: 'Switch Core 01',
			type: 'switch',
			height: 1,
			position: 36
		},
		{
			id: 'dev4',
			name: 'Storage Array 01',
			type: 'storage',
			height: 4,
			position: 32
		}
	]);

	// Statistics
	let deviceCount = $derived(devices.length);
	let usedUnits = $derived(devices.reduce((sum, device) => sum + device.height, 0));
	let availableUnits = $derived(rack ? rack.rack_height - usedUnits : 0);
	let usagePercentage = $derived(rack ? Math.round((usedUnits / rack.rack_height) * 100) : 0);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}

		await loadRackDetails();
	});

	async function loadRackDetails() {
		try {
			isLoading = true;
			error = '';

			// Parse the ID from the URL
			const rackId = $page.params.id;

			// Load rack details from API
			const rackData = await privateApi.get(`v0/ipam/racks/${rackId}`).json();

			// Map API response to our interface
			rack = {
				id: rackData.id,
				server_room_id: rackData.server_room_id,
				name: rackData.name,
				description: rackData.description,
				rack_height: rackData.rack_height,
				power_capacity: rackData.power_capacity,
				cooling_type: rackData.cooling_type,
				location_x: rackData.location_x,
				location_y: rackData.location_y,
				created_at: rackData.created_at,
				updated_at: rackData.updated_at,
				is_active: true,
				server_room_name: '10A 메인 서버룸', // TODO: Get from server room API
				office_name: '10A 사무실', // TODO: Get from office API
				device_count: 0,
				usage_percentage: 0,
				used_units: 0
			};

			// TODO: Load devices for this rack from API
			// devices = await deviceApi.getDevicesByRack(rackId);
		} catch (err) {
			console.error('Failed to load rack details:', err);
			error = '랙 정보를 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
	}

	function handleEdit() {
		// TODO: Implement edit functionality
		console.log('Edit rack');
	}

	function handleDelete() {
		// TODO: Implement delete functionality
		if (confirm('정말 이 랙을 삭제하시겠습니까?')) {
			console.log('Delete rack');
		}
	}

	function handleManageDevices() {
		// TODO: Navigate to device management page
		console.log('Manage devices');
	}

	function handleSlotClick(position: number) {
		console.log('Clicked empty slot at position:', position);
		// TODO: Show add device dialog for this position
	}

	function handleDeviceClick(device: any) {
		console.log('Clicked device:', device);
		// TODO: Show device details or edit dialog
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	{#if isLoading}
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600 dark:border-purple-400 mx-auto"></div>
				<p class="mt-4 text-gray-600 dark:text-gray-400">랙 정보를 불러오는 중...</p>
			</div>
		</div>
	{:else if error}
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center">
				<p class="text-red-600 dark:text-red-400">{error}</p>
				<button
					onclick={() => goto('/ipam/racks')}
					class="mt-4 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700"
				>
					랙 목록으로
				</button>
			</div>
		</div>
	{:else if rack}
		<!-- Header -->
		<div class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
				<div class="py-4">
					<!-- Breadcrumb and Actions -->
					<div class="flex items-center justify-between mb-4">
						<button
							onclick={() => window.history.back()}
							class="flex items-center gap-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100 transition-colors"
						>
							<ArrowLeft class="w-4 h-4" />
							<span>랙 상세 정보</span>
						</button>
						<div class="flex gap-2">
							<button
								onclick={handleEdit}
								class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors flex items-center gap-2"
							>
								<Edit class="w-4 h-4" />
								수정
							</button>
							<button
								onclick={handleDelete}
								class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors flex items-center gap-2"
							>
								<Trash2 class="w-4 h-4" />
								삭제
							</button>
						</div>
					</div>

					<!-- Rack Title -->
					<div class="flex items-center gap-3">
						<div class="p-3 bg-purple-100 dark:bg-purple-900 rounded-lg">
							<Server class="w-6 h-6 text-purple-600 dark:text-purple-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
								{rack.name}
							</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								랙 상세 정보
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Content -->
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<!-- Basic Info Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 mb-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">기본 정보</h2>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-4">
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">랙명</label>
							<p class="text-gray-900 dark:text-gray-100 font-medium">{rack.name}</p>
						</div>
						{#if rack.description}
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">설명</label>
								<p class="text-gray-900 dark:text-gray-100">{rack.description}</p>
							</div>
						{/if}
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">소속 사무실</label>
							<div class="flex items-center gap-2">
								<Building class="w-4 h-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{rack.office_name}</p>
							</div>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">서버실</label>
							<div class="flex items-center gap-2">
								<Server class="w-4 h-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{rack.server_room_name}</p>
							</div>
						</div>
					</div>

					<div class="space-y-4">
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">생성일</label>
							<div class="flex items-center gap-2">
								<Calendar class="w-4 h-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(rack.created_at)}</p>
							</div>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">수정일</label>
							<div class="flex items-center gap-2">
								<Calendar class="w-4 h-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(rack.updated_at)}</p>
							</div>
						</div>
						{#if rack.location_x !== null && rack.location_y !== null}
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">위치 좌표</label>
								<div class="flex items-center gap-2">
									<MapPin class="w-4 h-4 text-gray-500" />
									<p class="text-gray-900 dark:text-gray-100">X: {rack.location_x}, Y: {rack.location_y}</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Rack Specifications -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 mb-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">랙 사양</h2>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div class="flex items-center gap-3">
							<HardDrive class="w-5 h-5 text-blue-500" />
							<span class="text-gray-900 dark:text-gray-100">랙 높이</span>
						</div>
						<span class="font-medium text-gray-900 dark:text-gray-100">{rack.rack_height}U</span>
					</div>

					{#if rack.power_capacity}
						<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
							<div class="flex items-center gap-3">
								<Zap class="w-5 h-5 text-yellow-500" />
								<span class="text-gray-900 dark:text-gray-100">전력 용량</span>
							</div>
							<span class="font-medium text-gray-900 dark:text-gray-100">{rack.power_capacity}W</span>
						</div>
					{/if}

					{#if rack.cooling_type}
						<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
							<div class="flex items-center gap-3">
								<Snowflake class="w-5 h-5 text-cyan-500" />
								<span class="text-gray-900 dark:text-gray-100">냉각 방식</span>
							</div>
							<span class="font-medium text-gray-900 dark:text-gray-100">{rack.cooling_type}</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Usage Statistics -->
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
				<!-- Used Units -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
					<div class="flex items-center justify-between mb-4">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용 중</h3>
						<div class="p-2 bg-red-100 dark:bg-red-900 rounded-lg">
							<HardDrive class="w-5 h-5 text-red-600 dark:text-red-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{usedUnits}U</p>
					<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">사용됨</p>
				</div>

				<!-- Available Units -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
					<div class="flex items-center justify-between mb-4">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용 가능</h3>
						<div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
							<HardDrive class="w-5 h-5 text-green-600 dark:text-green-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{availableUnits}U</p>
					<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">남음</p>
				</div>

				<!-- Usage Percentage -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
					<div class="flex items-center justify-between mb-4">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용률</h3>
						<div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
							<Settings class="w-5 h-5 text-blue-600 dark:text-blue-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{usagePercentage}%</p>
					<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mt-2">
						<div
							class="h-2 rounded-full bg-blue-500 transition-all duration-300"
							style="width: {usagePercentage}%"
						></div>
					</div>
				</div>
			</div>

			<!-- Rack Visualization Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
				<div class="flex items-center justify-between mb-6">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">랙 시각화</h2>
					<div class="flex gap-2">
						<span class="text-sm text-gray-500 dark:text-gray-400">{deviceCount}개의 디바이스</span>
						<button
							onclick={handleManageDevices}
							class="px-3 py-1 bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-300 rounded-md text-sm hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
						>
							디바이스 관리
						</button>
					</div>
				</div>

				{#if rack}
					<RackVisualizer
						rackHeight={rack.rack_height}
						devices={devices}
						onSlotClick={handleSlotClick}
						onDeviceClick={handleDeviceClick}
						showLabels={true}
					/>
				{/if}

				<div class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
					<div class="flex items-center justify-between">
						<div class="text-sm text-gray-500 dark:text-gray-400">
							빈 슬롯을 클릭하면 새 디바이스를 추가할 수 있습니다.
						</div>
						<button
							onclick={handleManageDevices}
							class="text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 transition-colors font-medium"
						>
							디바이스 관리 →
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>