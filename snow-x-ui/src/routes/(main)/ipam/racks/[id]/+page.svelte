<svelte:options runes={false} />

<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
	ArrowLeft,
	Server,
	HardDrive,
	Edit,
	Trash2,
	Settings,
	Building,
	Calendar,
	Zap,
	Snowflake,
	MapPin
	} from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { privateApi } from '$lib/api/private';
	import RackGrid from '$lib/components/rack/RackGrid.svelte';
	import { deviceApi, type Device } from '$lib/api/office';

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

	let rack: Rack | null = null;
	let isLoading = true;
	let error = '';

	let devices: Device[] = [];

	type VisualDevice = { id: string; name: string; type: string; height: number; position: number };

	let visualDevices: VisualDevice[] = [];
	let deviceCount = 0;
	let usedUnits = 0;
	let availableUnits = 0;
	let usagePercentage = 0;

	$: updateRackMetrics();

	function updateRackMetrics() {
		const rackHeight = rack?.rack_height ?? 0;
		const list = devices ?? [];

		const mapped: VisualDevice[] = list
			.map((device) => {
				const position = Number(device.rack_position);
				const height = Number(device.rack_size ?? 1);

				if (!Number.isFinite(position)) {
					return null;
				}

				const effectiveHeight = Number.isFinite(height) && height > 0 ? Math.round(height) : 1;
				const basePosition = position <= 0 ? 1 : Math.round(position);
				const clampedPosition = rackHeight > 0 ? Math.min(basePosition, rackHeight) : basePosition;

				return {
					id: device.id,
					name: device.name,
					type: device.device_type ?? 'device',
					height: effectiveHeight,
					position: clampedPosition
				};
			})
			.filter((device): device is VisualDevice => {
				if (!device) return false;
				if (!rackHeight) return device.position > 0;
				return device.position > 0 && device.position <= rackHeight && device.position + device.height - 1 <= rackHeight;
			})
			.sort((a, b) => b.position - a.position);

		visualDevices = mapped;
		usedUnits = mapped.reduce((sum, device) => sum + device.height, 0);
		deviceCount = mapped.length;

		if (rackHeight > 0) {
			availableUnits = Math.max(0, rackHeight - usedUnits);
			usagePercentage = Math.min(100, Math.round((usedUnits / rackHeight) * 100));
		} else {
			availableUnits = 0;
			usagePercentage = 0;
		}
	}

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
				rack_height: Number(rackData.rack_height) || 0,
				power_capacity: rackData.power_capacity ? Number(rackData.power_capacity) : undefined,
				cooling_type: rackData.cooling_type,
				location_x: rackData.location_x,
				location_y: rackData.location_y,
				created_at: rackData.created_at,
				updated_at: rackData.updated_at,
				is_active: true,
				server_room_name: rackData.server_room_name ?? rackData.server_room_id,
				office_name: rackData.office_name ?? '미지정',
				device_count: rackData.device_count ?? 0,
				usage_percentage: rackData.usage_percentage ?? 0,
				used_units: rackData.used_units ?? 0
			};

			try {
				const deviceResponse = await deviceApi.getDevices({
					limit: 200,
					rack_id: rackId
				});
				devices = deviceResponse.devices.filter(
					(device) => device.rack_position !== null && device.rack_position !== undefined
				);
			} catch (deviceError) {
				console.error('Failed to load devices for rack:', deviceError);
				devices = [];
			}
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
		if (rack) {
			goto(`/ipam/device?rack=${rack.id}`);
		} else {
			goto('/ipam/device');
		}
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
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<div
					class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-purple-600 dark:border-purple-400"
				></div>
				<p class="mt-4 text-gray-600 dark:text-gray-400">랙 정보를 불러오는 중...</p>
			</div>
		</div>
	{:else if error}
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<p class="text-red-600 dark:text-red-400">{error}</p>
				<button
					onclick={() => goto('/ipam/racks')}
					class="mt-4 rounded-lg bg-purple-600 px-4 py-2 text-white hover:bg-purple-700"
				>
					랙 목록으로
				</button>
			</div>
		</div>
	{:else if rack}
		<!-- Header -->
		<div class="border-b border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div class="py-4">
					<!-- Breadcrumb and Actions -->
					<div class="mb-4 flex items-center justify-between">
						<button
							onclick={() => window.history.back()}
							class="flex items-center gap-2 text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100"
						>
							<ArrowLeft class="h-4 w-4" />
							<span>랙 상세 정보</span>
						</button>
						<div class="flex gap-2">
							<button
								onclick={handleEdit}
								class="flex items-center gap-2 rounded-lg bg-purple-600 px-4 py-2 text-white transition-colors hover:bg-purple-700"
							>
								<Edit class="h-4 w-4" />
								수정
							</button>
							<button
								onclick={handleDelete}
								class="flex items-center gap-2 rounded-lg bg-red-600 px-4 py-2 text-white transition-colors hover:bg-red-700"
							>
								<Trash2 class="h-4 w-4" />
								삭제
							</button>
						</div>
					</div>

					<!-- Rack Title -->
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-purple-100 p-3 dark:bg-purple-900">
							<Server class="h-6 w-6 text-purple-600 dark:text-purple-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
								{rack.name}
							</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">랙 상세 정보</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Content -->
		<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<!-- Basic Info Section -->
			<div class="mb-6 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<h2 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">기본 정보</h2>

				<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
					<div class="space-y-4">
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">랙명</label>
							<p class="font-medium text-gray-900 dark:text-gray-100">{rack.name}</p>
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
								<Building class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{rack.office_name}</p>
							</div>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">서버실</label>
							<div class="flex items-center gap-2">
								<Server class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{rack.server_room_name}</p>
							</div>
						</div>
					</div>

					<div class="space-y-4">
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">생성일</label>
							<div class="flex items-center gap-2">
								<Calendar class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(rack.created_at)}</p>
							</div>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">수정일</label>
							<div class="flex items-center gap-2">
								<Calendar class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(rack.updated_at)}</p>
							</div>
						</div>
						{#if rack.location_x !== null && rack.location_y !== null}
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">위치 좌표</label>
								<div class="flex items-center gap-2">
									<MapPin class="h-4 w-4 text-gray-500" />
									<p class="text-gray-900 dark:text-gray-100">X: {rack.location_x}, Y: {rack.location_y}</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Rack Specifications -->
			<div class="mb-6 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<h2 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">랙 사양</h2>

				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
						<div class="flex items-center gap-3">
							<HardDrive class="h-5 w-5 text-blue-500" />
							<span class="text-gray-900 dark:text-gray-100">랙 높이</span>
						</div>
						<span class="font-medium text-gray-900 dark:text-gray-100">{rack.rack_height}U</span>
					</div>

					{#if rack.power_capacity}
						<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
							<div class="flex items-center gap-3">
								<Zap class="h-5 w-5 text-yellow-500" />
								<span class="text-gray-900 dark:text-gray-100">전력 용량</span>
							</div>
							<span class="font-medium text-gray-900 dark:text-gray-100">{rack.power_capacity}W</span>
						</div>
					{/if}

					{#if rack.cooling_type}
						<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
							<div class="flex items-center gap-3">
								<Snowflake class="h-5 w-5 text-cyan-500" />
								<span class="text-gray-900 dark:text-gray-100">냉각 방식</span>
							</div>
							<span class="font-medium text-gray-900 dark:text-gray-100">{rack.cooling_type}</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Usage Statistics -->
			<div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-3">
				<!-- Used Units -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용 중</h3>
						<div class="rounded-lg bg-red-100 p-2 dark:bg-red-900">
							<HardDrive class="h-5 w-5 text-red-600 dark:text-red-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{usedUnits}U</p>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">사용됨</p>
				</div>

				<!-- Available Units -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용 가능</h3>
						<div class="rounded-lg bg-green-100 p-2 dark:bg-green-900">
							<HardDrive class="h-5 w-5 text-green-600 dark:text-green-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{availableUnits}U</p>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">남음</p>
				</div>

				<!-- Usage Percentage -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">사용률</h3>
						<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
							<Settings class="h-5 w-5 text-blue-600 dark:text-blue-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{usagePercentage}%</p>
					<div class="mt-2 h-2 w-full rounded-full bg-gray-200 dark:bg-gray-700">
						<div
							class="h-2 rounded-full bg-blue-500 transition-all duration-300"
							style="width: {usagePercentage}%"
						></div>
					</div>
				</div>
			</div>

			<!-- Rack Visualization Section -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-6 flex items-center justify-between">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">랙 시각화</h2>
					<div class="flex gap-2">
						<span class="text-sm text-gray-500 dark:text-gray-400">{deviceCount}개의 디바이스</span>
						<button
							onclick={handleManageDevices}
							class="rounded-md bg-purple-100 px-3 py-1 text-sm text-purple-700 transition-colors hover:bg-purple-200 dark:bg-purple-900 dark:text-purple-300 dark:hover:bg-purple-800"
						>
							디바이스 관리
						</button>
					</div>
				</div>

		{#if rack}
			<RackGrid rackHeight={rack.rack_height} devices={visualDevices} />
		{/if}

				<div class="mt-6 border-t border-gray-200 pt-6 dark:border-gray-700">
					<div class="flex items-center justify-between">
						<div class="text-sm text-gray-500 dark:text-gray-400">
							빈 슬롯을 클릭하면 새 디바이스를 추가할 수 있습니다.
						</div>
						<button
							onclick={handleManageDevices}
							class="font-medium text-purple-600 transition-colors hover:text-purple-700 dark:text-purple-400 dark:hover:text-purple-300"
						>
							디바이스 관리 →
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
