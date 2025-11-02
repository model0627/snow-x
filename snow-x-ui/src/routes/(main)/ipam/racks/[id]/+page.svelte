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
		MapPin,
		Loader2
	} from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { privateApi } from '$lib/api/private';
import RackView, { type RackViewDevice } from '$lib/components/rack/RackView.svelte';
import DeviceFormDialog from '$lib/components/ipam/DeviceFormDialog.svelte';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
import { Button } from '$lib/components/ui/button';
	import { deviceApi, type Device, type RackDeviceSummary } from '$lib/api/office';

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

interface DeviceDialogPrefill {
	rack_id?: string;
	rack_position?: number;
	rack_size?: number;
	device_type?: string;
	status?: string;
	name?: string;
}

const STATUS_LABEL_MAP: Record<string, string> = {
	active: 'A',
	maintenance: 'F',
	inactive: 'I'
	};

	const TYPE_LABEL_MAP: Record<string, string> = {
		server: 'S',
		storage: 'S',
		switch: 'P',
		router: 'P',
		firewall: 'F'
	};

	let rack: Rack | null = null;
	let isLoading = true;
	let error = '';

let devices: RackDeviceSummary[] = [];
	let rackViewDevices: RackViewDevice[] = [];
	let deviceCount = 0;
	let usedUnits = 0;
	let availableUnits = 0;
	let usagePercentage = 0;
	let totalRackUnits = 0;
	const deviceDetailCache: Map<string, Device> = new Map();
	let deviceDialogOpen = false;
	let deviceDialogMode: 'create' | 'edit' = 'create';
	let deviceDialogPrefill: DeviceDialogPrefill | null = null;
	let deviceDialogEditData: Device | null = null;
	let deviceDialogLoading = false;
	let pendingDeleteDevice: Device | null = null;
	let deleteConfirmOpen = false;
	let isDeletingDevice = false;

	function deriveLabels(device: RackDeviceSummary): string[] {
		const labels = new Set<string>();
		const status = device.status?.toLowerCase();
		const statusLabel = status ? STATUS_LABEL_MAP[status] : undefined;
		if (statusLabel) labels.add(statusLabel);
		const type = device.device_type?.toLowerCase();
		const typeLabel = type ? TYPE_LABEL_MAP[type] : undefined;
		if (typeLabel) labels.add(typeLabel);
		if (!labels.size) labels.add('I');
		return Array.from(labels);
}

function computeRackMetrics() {
	if (!rack) {
		rackViewDevices = [];
		deviceCount = 0;
		usedUnits = 0;
		availableUnits = 0;
		usagePercentage = 0;
		totalRackUnits = 0;
		console.debug('[rack] computeRackMetrics skipped: no rack data yet');
		return;
	}

	const rackHeight = rack.rack_height ?? 0;
	console.log('[rack] computeRackMetrics input', {
		rackHeight,
		deviceCount: devices.length,
		devices
	});
	const maxDeviceExtent = devices.reduce((max, device) => {
		const startRaw = parseFloat(String(device.rack_position ?? ''));
		const sizeRaw = parseFloat(String(device.rack_size ?? '1'));
		if (!Number.isFinite(startRaw) || startRaw <= 0) {
			return max;
		}
		const size = Number.isFinite(sizeRaw) && sizeRaw > 0 ? Math.round(sizeRaw) : 1;
		const end = Math.round(startRaw) + size - 1;
		return Math.max(max, end);
	}, 0);
	const baseHeight = rackHeight > 0 ? rackHeight : 48;
	const defaultHeight = Math.max(48, baseHeight);
	const results: RackViewDevice[] = [];
	let calculatedUsed = 0;
	const occupiedSlots = new Set<number>();
	let fallbackCursor = 1;
	let dynamicMaxExtent = Math.max(defaultHeight, maxDeviceExtent);

	for (const device of devices ?? []) {
			const rawPosition = parseFloat(String(device.rack_position ?? ''));
			const rawSize = parseFloat(String(device.rack_size ?? '1'));
			const size = Math.max(1, Number.isFinite(rawSize) && rawSize > 0 ? Math.round(rawSize) : 1);

			let start: number;

			if (Number.isFinite(rawPosition) && rawPosition > 0) {
				start = Math.max(1, Math.round(rawPosition));
			} else {
				let candidate = Math.max(1, fallbackCursor);
				let placed = false;

				while (!placed) {
					let conflict = false;
					for (let offset = 0; offset < size; offset += 1) {
						if (occupiedSlots.has(candidate + offset)) {
							conflict = true;
							break;
						}
					}

					if (!conflict) {
						start = candidate;
						fallbackCursor = candidate + size;
						placed = true;
					} else {
						candidate += 1;
					}
				}
			}

			calculatedUsed += size;
			dynamicMaxExtent = Math.max(dynamicMaxExtent, start + size - 1);

			for (let offset = 0; offset < size; offset += 1) {
				occupiedSlots.add(start + offset);
			}

			const labels = deriveLabels(device);
			results.push({
				id: device.id,
				name: device.name,
				start,
				size,
				frontLabels: labels,
				rearLabels: labels
			});
		}

		rackViewDevices = results.sort((a, b) => b.start - a.start);
		deviceCount = devices.length || rack.device_count || 0;
		totalRackUnits = Math.max(dynamicMaxExtent, defaultHeight);

		const usedByLayout = rackViewDevices.reduce((sum, device) => sum + device.size, 0);
		if (usedByLayout > 0) {
			usedUnits = usedByLayout;
		} else if (calculatedUsed > 0) {
			usedUnits = calculatedUsed;
		} else if (rack.used_units) {
			usedUnits = rack.used_units;
		} else {
			usedUnits = 0;
		}

		const baseUnits = Math.max(rackHeight, totalRackUnits);
		availableUnits = Math.max(0, baseUnits - usedUnits);
		if (baseUnits > 0) {
			const computed = rack.usage_percentage ?? Math.round((usedUnits / baseUnits) * 100);
			usagePercentage = Math.min(100, Math.max(0, computed));
		} else {
		usagePercentage = rack.usage_percentage ?? 0;
	}

	console.debug('[rack] metrics calculated', {
		rackHeight,
		totalRackUnits,
		maxDeviceExtent,
		deviceCount,
		usedUnits,
		availableUnits,
		usagePercentage,
		devices,
		rackViewDevices
	});
	console.log('[rack] computeRackMetrics result', {
		totalRackUnits,
		deviceCount,
		usedUnits,
		availableUnits,
		usagePercentage,
		rackViewDevices
	});
}

	$: computeRackMetrics();

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
			const rackId = $page.params.id;

			const rackData = await privateApi.get(`v0/ipam/racks/${rackId}`).json();
			const rackDevicesRaw = (rackData.devices ?? []) as RackDeviceSummary[];
			console.log('[rack] detail response received', {
				rackId,
				rawDeviceCount: rackDevicesRaw.length,
				rackData
			});
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
				device_count: rackData.device_count ?? rackDevicesRaw.length ?? 0,
				usage_percentage: rackData.usage_percentage ?? 0,
				used_units: rackData.used_units ?? 0
			};

			let mappedDevices = rackDevicesRaw.map((device) => ({
					...device,
					rack_size: Number.isFinite(device.rack_size) && device.rack_size > 0 ? Math.round(device.rack_size) : 1,
					rack_position:
						device.rack_position !== null && device.rack_position !== undefined
							? Number(device.rack_position)
							: undefined
				}));

			if (!mappedDevices.length) {
				console.log('[rack] no embedded devices detected, falling back to deviceApi', {
					rackId
				});
					let fallbackDevices: Device[] = [];
					let currentPage = 1;
					const pageSize = 500;
					let remaining = true;

					try {
						while (remaining) {
							const deviceResponse = await deviceApi.getDevices({
								page: currentPage,
								limit: pageSize,
								rack_id: rackId
							});

							fallbackDevices = fallbackDevices.concat(deviceResponse.devices);
							remaining = fallbackDevices.length < deviceResponse.total;
							currentPage += 1;
						}
					} catch (deviceError) {
						console.error('Failed to load devices for rack (fallback):', deviceError);
						fallbackDevices = [];
					}

					fallbackDevices.forEach((device) => {
						deviceDetailCache.set(device.id, device);
					});

					console.log('[rack] fallback device response', {
						total: fallbackDevices.length,
						pagesFetched: currentPage - 1,
						sample: fallbackDevices.slice(0, 3)
					});

					mappedDevices = fallbackDevices
						.filter((device) => device.rack_id === rackId)
						.map((device) => ({
							id: device.id,
							name: device.name,
							device_type: device.device_type,
							status: device.status,
							manufacturer: device.manufacturer,
							model: device.model,
							serial_number: device.serial_number,
							rack_position: device.rack_position ?? undefined,
							rack_size:
								Number.isFinite(device.rack_size) && device.rack_size > 0
									? Math.round(device.rack_size)
									: 1
						}));
				console.log('[rack] fallback device fetch complete', {
					totalFetched: fallbackDevices.length,
					mappedDevices: mappedDevices.length
				});
				}

			devices = mappedDevices;
			console.log('[rack] devices prepared for visualization', {
				count: devices.length,
				devices
			});

			computeRackMetrics();
		} catch (err) {
			console.error('Failed to load rack details:', err);
			error = '랙 정보를 불러오는데 실패했습니다.';
			devices = [];
		} finally {
			isLoading = false;
		}
	}

	function openCreateDialogForUnit(unit: number) {
		if (!rack) return;
		deviceDialogMode = 'create';
		deviceDialogPrefill = {
			rack_id: rack.id,
			rack_position: unit,
			rack_size: 1,
			status: 'active'
		};
		deviceDialogEditData = null;
		deviceDialogOpen = true;
	}

	async function ensureDeviceDetail(deviceId: string): Promise<Device | null> {
		if (!deviceId) return null;
		if (deviceDetailCache.has(deviceId)) {
			return deviceDetailCache.get(deviceId)!;
		}
		try {
			const detail = await deviceApi.getDevice(deviceId);
			deviceDetailCache.set(deviceId, detail);
			return detail;
		} catch (error) {
			console.error('Failed to fetch device detail:', error);
			alert('디바이스 정보를 불러오지 못했습니다.');
			return null;
		}
	}

	function handleSlotSelect(event: CustomEvent<{ unit: number; column: 'front' | 'rear' }>) {
		openCreateDialogForUnit(event.detail.unit);
	}

	async function handleDeviceSelect(event: CustomEvent<{ device: RackViewDevice; column: 'front' | 'rear' }>) {
		const deviceId = event.detail.device.id;
		if (!deviceId) return;
		deviceDialogMode = 'edit';
		deviceDialogLoading = true;
		const detail = await ensureDeviceDetail(deviceId);
		deviceDialogLoading = false;
		if (!detail) return;
		deviceDialogEditData = detail;
		deviceDialogPrefill = null;
		deviceDialogOpen = true;
	}

	function handleDeviceDialogClose() {
		deviceDialogOpen = false;
		deviceDialogPrefill = null;
		deviceDialogEditData = null;
		deviceDialogMode = 'create';
	}

	async function handleDeviceDialogSuccess(result?: Device) {
		if (result) {
			deviceDetailCache.set(result.id, result);
		}
		await loadRackDetails();
	}

	function handleDeviceDeleteRequest(event: CustomEvent<Device>) {
		pendingDeleteDevice = event.detail;
		deleteConfirmOpen = true;
		deviceDialogOpen = false;
	}

	function handleDeleteCancel() {
		deleteConfirmOpen = false;
		if (pendingDeleteDevice) {
			deviceDialogMode = 'edit';
			deviceDialogEditData = pendingDeleteDevice;
			deviceDialogPrefill = null;
			deviceDialogOpen = true;
		}
		pendingDeleteDevice = null;
	}

	async function confirmDeviceDelete() {
		if (!pendingDeleteDevice) return;
		isDeletingDevice = true;
		try {
			await deviceApi.deleteDevice(pendingDeleteDevice.id);
			deviceDetailCache.delete(pendingDeleteDevice.id);
			deleteConfirmOpen = false;
			deviceDialogOpen = false;
			deviceDialogMode = 'create';
			deviceDialogEditData = null;
			const deletedDeviceId = pendingDeleteDevice.id;
			pendingDeleteDevice = null;
			await loadRackDetails();
			console.log('[rack] device deleted', { deletedDeviceId });
		} catch (error) {
			console.error('Failed to delete device:', error);
			alert('디바이스 삭제에 실패했습니다.');
		} finally {
			isDeletingDevice = false;
		}
	}

	function handleEdit() {
		console.log('Edit rack');
	}

	function handleDelete() {
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
					{#if totalRackUnits > 0}
						<p class="text-sm text-gray-500 dark:text-gray-400">
							{usedUnits.toLocaleString()}U / {totalRackUnits.toLocaleString()}U
						</p>
					{/if}
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
		<RackView
			rackHeight={rack.rack_height ?? 48}
			devices={rackViewDevices}
			on:slot-select={handleSlotSelect}
			on:device-select={handleDeviceSelect}
		/>
	{/if}

		<div class="mt-6 border-t border-gray-200 pt-6 dark:border-gray-700">
			<div class="flex items-center justify-between">
				<div class="text-sm text-gray-500 dark:text-gray-400">
					상세 디바이스 정보는 아래 관리 페이지에서 수정할 수 있습니다.
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

{#if deviceDialogLoading}
	<div class="fixed inset-0 z-[60] flex items-center justify-center bg-slate-900/30">
		<Loader2 class="h-6 w-6 animate-spin text-white" />
	</div>
{/if}

<DeviceFormDialog
	open={deviceDialogOpen}
	onClose={handleDeviceDialogClose}
	onSuccess={handleDeviceDialogSuccess}
	editData={deviceDialogMode === 'edit' ? deviceDialogEditData : null}
	prefill={deviceDialogMode === 'create' ? deviceDialogPrefill : null}
	enableDelete
	on:request-delete={handleDeviceDeleteRequest}
/>

<Dialog open={deleteConfirmOpen} onOpenChange={(value) => !value && handleDeleteCancel()}>
	<DialogContent class="max-w-md">
		<DialogHeader>
			<DialogTitle>디바이스 삭제</DialogTitle>
		</DialogHeader>
		<p class="text-sm text-gray-600 dark:text-gray-300">
			정말로
			<strong class="font-semibold text-gray-900 dark:text-gray-100">{pendingDeleteDevice?.name}</strong>
			디바이스를 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.
		</p>
		<DialogFooter class="mt-6">
			<Button type="button" variant="outline" onclick={handleDeleteCancel} disabled={isDeletingDevice}>
				취소
			</Button>
			<Button
				type="button"
				variant="destructive"
				onclick={confirmDeviceDelete}
				disabled={isDeletingDevice}
			>
				{#if isDeletingDevice}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					삭제 중...
				{:else}
					삭제
				{/if}
			</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
