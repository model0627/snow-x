<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		ArrowLeft,
		HardDrive,
		Server,
		Network,
		Wifi,
		Shield,
		Database,
		Edit,
		Trash2,
		Plus,
		BookOpen,
		ExternalLink,
		X
	} from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import DeviceFormDialog from '$lib/components/ipam/DeviceFormDialog.svelte';
	import IpAssignDialog from '$lib/components/ipam/IpAssignDialog.svelte';
	import ContactAssignDialog from '$lib/components/ipam/ContactAssignDialog.svelte';
	import {
		deviceApi,
		rackApi,
		deviceLibraryApi,
		type Device,
		type Rack,
		type IpAddress,
		type DeviceContact,
		type DeviceLibrary
	} from '$lib/api/office';

	type DeviceTypeIcon = typeof Server;

	let device = $state<Device | null>(null);
let assignedIps = $state<IpAddress[]>([]);
let assignedContacts = $state<DeviceContact[]>([]);
let racks = $state<Rack[]>([]);
let linkedLibrary = $state<DeviceLibrary | null>(null);
let isLoading = $state(true);
let error = $state('');
let showEditDialog = $state(false);
let showIpAssignDialog = $state(false);
let showContactAssignDialog = $state(false);
let showLibraryLinkDialog = $state(false);
let availableLibraries = $state<DeviceLibrary[]>([]);
let selectedLibraryIdForLink = $state('');
let isLinkingLibrary = $state(false);
let showRackLinkDialog = $state(false);
let availableRacksForLink = $state<Rack[]>([]);
let selectedRackIdForLink = $state('');
let selectedRackPositionForLink = $state('');
let isLinkingRack = $state(false);
	const deviceId = $derived($page.params.id);

	const deviceTypeIcon = $derived.by(() => (device ? getDeviceTypeIcon(device.device_type) : HardDrive));
	const deviceTypeLabel = $derived.by(() => (device ? getDeviceTypeLabel(device.device_type) : ''));
	const statusLabel = $derived.by(() => (device ? getStatusLabel(device.status) : ''));
	const statusBadgeClass = $derived.by(() =>
		device ? getStatusBadgeClass(device.status) : 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200'
	);
	const sourceLabel = $derived.by(() => (device ? getSourceLabel(device.source_type) : ''));
	const sourceBadgeClass = $derived.by(() =>
		device ? getSourceBadgeClass(device.source_type) : 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200'
	);
	const rackInfo = $derived.by(() => {
		if (!device || !device.rack_id) return null;
		return racks.find((rack) => rack.id === device.rack_id) ?? null;
	});
	const sortedAssignedIps = $derived.by(() =>
		[...assignedIps].sort((a, b) => ipToNumber(a.ip_address) - ipToNumber(b.ip_address))
	);
const sortedAssignedContacts = $derived.by(() =>
	[...assignedContacts].sort((a, b) => a.contact_name.localeCompare(b.contact_name, 'ko'))
);
const isLibraryLinked = $derived(Boolean(linkedLibrary));
const linkableLibraries = $derived.by(() =>
	availableLibraries.filter((library) => !library.device_id || library.device_id === device?.id)
);
const librariesLinkedElsewhere = $derived.by(() =>
	availableLibraries.filter((library) => library.device_id && library.device_id !== device?.id)
);
const linkableRacks = $derived.by(() =>
	availableRacksForLink.map((rackItem) => ({
		id: rackItem.id,
		name: rackItem.name,
		height: rackItem.rack_height ?? 0
	}))
);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}

		await loadDeviceDetails();
	});

	async function loadDeviceDetails() {
		const id = deviceId;
		if (!id) {
			error = '유효하지 않은 디바이스 ID입니다.';
			isLoading = false;
			return;
		}

		try {
			isLoading = true;
			error = '';

			const deviceData = await deviceApi.getDevice(id);
			device = deviceData;

			await Promise.all([loadAssignedIps(id), loadAssignedContacts(id), loadRacks(), loadLinkedLibrary(id)]);
		} catch (err) {
			console.error('Failed to load device details:', err);
			error = '디바이스 정보를 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
	}

async function loadAssignedIps(id: string) {
	try {
		assignedIps = await deviceApi.getAssignedIpAddresses(id);
	} catch (err) {
		console.error('Failed to load assigned IP addresses:', err);
		assignedIps = [];
	}
}

async function loadAssignedContacts(id: string) {
	try {
		const response = await deviceApi.getAssignedContacts(id);
		assignedContacts = response.mappings;
	} catch (err) {
		console.error('Failed to load assigned contacts:', err);
		assignedContacts = [];
	}
}

async function loadLinkedLibrary(id: string) {
	try {
		const response = await deviceLibraryApi.getDeviceLibraries({ page: 1, limit: 1000 });
		const found = response.libraries.find((library) => library.device_id === id) ?? null;
		linkedLibrary = found;
		availableLibraries = response.libraries;
	} catch (err) {
		console.error('Failed to load linked library:', err);
		linkedLibrary = null;
		availableLibraries = [];
	}
}

async function loadRacks() {
	try {
		const response = await rackApi.getRacks({ page: 1, limit: 1000 });
		racks = response.racks;
		availableRacksForLink = response.racks;
	} catch (err) {
		console.error('Failed to load racks:', err);
		racks = [];
		availableRacksForLink = [];
	}
}

async function handleUnlinkLibrary() {
	if (!linkedLibrary || !device) return;
	if (!confirm('이 디바이스와 라이브러리 연결을 해제할까요?')) return;

	try {
		await deviceLibraryApi.updateDeviceLibrary(linkedLibrary.id, {
			device_id: null,
			device_name: null,
			remove_device_link: true
		});
		linkedLibrary = null;
	} catch (err) {
		console.error('Failed to unlink library:', err);
		alert('라이브러리 연결 해제에 실패했습니다.');
	}
}

function handleNavigateToLibrary() {
	goto('/ipam/responsible');
}

function handleOpenRackLinkDialog() {
	if (device) {
		selectedRackIdForLink = device.rack_id || '';
		selectedRackPositionForLink = device.rack_position?.toString() || '';
	}
	showRackLinkDialog = true;
}

async function handleRackLinkSubmit(event: SubmitEvent) {
	event.preventDefault();
	if (!device) return;

	const payload: Record<string, any> = {};

	if (selectedRackIdForLink) {
		payload.rack_id = selectedRackIdForLink;
		if (!selectedRackPositionForLink.trim()) {
			alert('랙 위치(U)를 입력해주세요.');
			return;
		}
		const parsedPosition = Number(selectedRackPositionForLink);
		if (!Number.isFinite(parsedPosition) || parsedPosition <= 0) {
			alert('유효한 랙 위치(U)를 입력해주세요.');
			return;
		}
		payload.rack_position = parsedPosition;
	} else {
		payload.rack_id = null;
		payload.rack_position = null;
	}

	isLinkingRack = true;
	try {
		await deviceApi.updateDevice(device.id, payload);
		await loadDeviceDetails();
		showRackLinkDialog = false;
		selectedRackIdForLink = '';
		selectedRackPositionForLink = '';
	} catch (err) {
		console.error('Failed to link rack:', err);
		alert('랙 연결에 실패했습니다.');
	} finally {
		isLinkingRack = false;
	}
}

async function handleLibraryLinkSubmit(event: SubmitEvent) {
	event.preventDefault();
	if (!device || !selectedLibraryIdForLink) return;

	const targetLibrary = availableLibraries.find((library) => library.id === selectedLibraryIdForLink);
	if (!targetLibrary) {
		alert('선택한 라이브러리를 찾을 수 없습니다.');
		return;
	}

	isLinkingLibrary = true;
	try {
		await deviceLibraryApi.updateDeviceLibrary(targetLibrary.id, {
			device_id: device.id,
			device_name: device.name
		});
		linkedLibrary = targetLibrary;
		showLibraryLinkDialog = false;
		selectedLibraryIdForLink = '';
		await loadLinkedLibrary(device.id);
	} catch (err) {
		console.error('Failed to link library:', err);
		alert('라이브러리 연결에 실패했습니다.');
	} finally {
		isLinkingLibrary = false;
	}
}

	function handleEdit() {
		showEditDialog = true;
	}

	function handleEditDialogClose() {
		showEditDialog = false;
	}

async function handleEditSuccess(updatedDevice?: Device) {
	if (updatedDevice) {
		device = updatedDevice;
		if (updatedDevice.id) {
			await Promise.all([
				loadAssignedIps(updatedDevice.id),
				loadAssignedContacts(updatedDevice.id)
			]);
		}
			if (updatedDevice.rack_id && !racks.some((rack) => rack.id === updatedDevice.rack_id)) {
				await loadRacks();
			}
		} else {
			await loadDeviceDetails();
		}
	}

	function handleAssignIp() {
		showIpAssignDialog = true;
	}

	function handleIpAssignDialogClose() {
		showIpAssignDialog = false;
	}

async function handleIpAssignSuccess() {
	if (device) {
		await loadAssignedIps(device.id);
	}
	showIpAssignDialog = false;
}

async function handleUnassignIp(ip: IpAddress) {
	if (!device) return;

	try {
		await deviceApi.unassignIpAddress(device.id, ip.id);
		await loadAssignedIps(device.id);
	} catch (err) {
		console.error('Failed to unassign IP address:', err);
		alert('IP 주소 할당 해제에 실패했습니다.');
	}
}

function handleAssignContact() {
	showContactAssignDialog = true;
}

function handleContactAssignDialogClose() {
	showContactAssignDialog = false;
}

async function handleContactAssignSuccess() {
	if (device) {
		await loadAssignedContacts(device.id);
	}
	showContactAssignDialog = false;
}

async function handleUnassignContact(contact: DeviceContact) {
	if (!device) return;
	if (!confirm(`${contact.contact_name} 담당자 연결을 해제하시겠습니까?`)) return;

	try {
		await deviceApi.unassignContact(device.id, contact.contact_id);
		await loadAssignedContacts(device.id);
	} catch (err) {
		console.error('Failed to unassign contact:', err);
		alert('담당자 연결 해제에 실패했습니다.');
	}
}

async function handleDelete() {
	if (!device) return;
	if (!confirm(`정말 "${device.name}" 디바이스를 삭제하시겠습니까?`)) return;

	try {
		await deviceApi.deleteDevice(device.id);
		goto('/ipam/device');
	} catch (err) {
		console.error('Failed to delete device:', err);
		alert('디바이스 삭제에 실패했습니다.');
	}
}

	function getDeviceTypeIcon(type: string): DeviceTypeIcon {
		switch (type?.toLowerCase()) {
			case 'server':
				return Server;
			case 'switch':
				return Network;
			case 'router':
				return Wifi;
			case 'firewall':
				return Shield;
			case 'storage':
				return Database;
			default:
				return HardDrive;
		}
	}

	function getDeviceTypeLabel(type: string): string {
		switch (type?.toLowerCase()) {
			case 'server':
				return '서버';
			case 'switch':
				return '스위치';
			case 'router':
				return '라우터';
			case 'firewall':
				return '방화벽';
			case 'storage':
				return '스토리지';
			default:
				return type || '-';
		}
	}

	function getStatusLabel(status: string): string {
		switch (status) {
			case 'active':
				return '활성';
			case 'inactive':
				return '비활성';
			case 'maintenance':
				return '점검';
			default:
				return status || '-';
		}
	}

	function getStatusBadgeClass(status: string): string {
		switch (status) {
			case 'active':
				return 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200';
			case 'inactive':
				return 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200';
			case 'maintenance':
				return 'bg-orange-100 dark:bg-orange-900 text-orange-800 dark:text-orange-200';
			default:
				return 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200';
		}
	}

	function getSourceLabel(source: string): string {
		switch (source) {
			case 'api_sync':
				return 'API 동기화';
			default:
				return '수동 입력';
		}
	}

	function getSourceBadgeClass(source: string): string {
		switch (source) {
			case 'api_sync':
				return 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200';
			default:
				return 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200';
		}
	}

	function formatDate(dateString?: string | null): string {
		if (!dateString) return '-';
		const date = new Date(dateString);
		if (Number.isNaN(date.getTime())) return '-';
		return date.toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}

	function formatDateTime(dateString?: string | null): string {
		if (!dateString) return '-';
		const date = new Date(dateString);
		if (Number.isNaN(date.getTime())) return '-';
		return date.toLocaleString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatPower(power?: number | null): string {
		if (power === null || power === undefined) return '-';
		return `${power} W`;
	}

	function formatRackPosition(position?: number | null): string {
		if (!position) return '-';
		return `U${position}`;
	}

	function ipToNumber(ip?: string): number {
		if (!ip) return Number.MAX_SAFE_INTEGER;
		const parts = ip.split('.').map(Number);
		if (parts.length !== 4 || parts.some((part) => Number.isNaN(part))) {
			return Number.MAX_SAFE_INTEGER;
		}
		return (parts[0] << 24) + (parts[1] << 16) + (parts[2] << 8) + parts[3];
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	{#if isLoading}
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<div class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-blue-600 dark:border-blue-400"></div>
				<p class="mt-4 text-gray-600 dark:text-gray-400">디바이스 정보를 불러오는 중...</p>
			</div>
		</div>
	{:else if error}
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<p class="text-red-600 dark:text-red-400">{error}</p>
				<button
					onclick={() => goto('/ipam/device')}
					class="mt-4 rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
				>
					디바이스 목록으로
				</button>
			</div>
		</div>
	{:else if device}
		<!-- Header -->
		<div class="border-b border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div class="py-4">
					<div class="mb-4 flex items-center justify-between">
						<button
							onclick={() => goto('/ipam/device')}
							class="flex items-center gap-2 text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100"
						>
							<ArrowLeft class="h-4 w-4" />
							<span>디바이스 목록으로</span>
						</button>
						<div class="flex gap-2">
							<button
								onclick={handleAssignIp}
								class="flex items-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-white transition-colors hover:bg-green-700"
							>
								<Plus class="h-4 w-4" />
								IP 할당
							</button>
							<button
								onclick={handleEdit}
								class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
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

					<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
						<div class="flex items-center gap-3">
							<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
								<svelte:component this={deviceTypeIcon} class="h-6 w-6 text-blue-600 dark:text-blue-400" />
							</div>
							<div>
								<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">{device.name}</h1>
								<div class="mt-1 flex flex-wrap items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
									<span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200">
										{deviceTypeLabel}
									</span>
									<span class="rounded-full px-2 py-1 text-xs font-medium {statusBadgeClass}">
										{statusLabel}
									</span>
									{#if device.serial_number}
										<span>시리얼: {device.serial_number}</span>
									{/if}
								</div>
							</div>
						</div>
						<div class="text-sm text-gray-500 dark:text-gray-400">
							<div>등록일: {formatDate(device.created_at)}</div>
							<div>최근 업데이트: {formatDate(device.updated_at)}</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Content -->
		<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2 space-y-6">
					<!-- Basic Information -->
					<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
						<h2 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">기본 정보</h2>
						<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">디바이스 타입</label>
								<div class="mt-1 flex items-center gap-2 text-gray-900 dark:text-gray-100">
									<svelte:component this={deviceTypeIcon} class="h-4 w-4 text-blue-600 dark:text-blue-400" />
									<span>{deviceTypeLabel}</span>
								</div>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">상태</label>
								<div class="mt-1">
									<span class="rounded-full px-2 py-1 text-xs font-medium {statusBadgeClass}">{statusLabel}</span>
								</div>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">제조사</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{device.manufacturer || '-'}</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">모델명</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{device.model || '-'}</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">구매일</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{formatDate(device.purchase_date)}</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">보증 만료일</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{formatDate(device.warranty_end)}</p>
							</div>
						</div>

						{#if device.description}
							<div class="mt-6">
								<label class="text-sm text-gray-500 dark:text-gray-400">설명</label>
								<p class="mt-2 whitespace-pre-line rounded-lg border border-gray-100 bg-gray-50 p-4 text-sm text-gray-700 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-300">
									{device.description}
								</p>
							</div>
						{/if}
					</div>

				<!-- Deployment Information -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">배치 정보</h2>
						<button
							onclick={handleOpenRackLinkDialog}
							class="rounded-md border border-blue-500 px-3 py-1.5 text-sm text-blue-600 transition-colors hover:bg-blue-500 hover:text-white dark:border-blue-400 dark:text-blue-300 dark:hover:bg-blue-400 dark:hover:text-white"
						>
							{device?.rack_id ? '랙 변경' : '랙 연결'}
						</button>
					</div>
						<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">랙</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">
									{#if rackInfo}
										{rackInfo.name}
										{#if device.rack_position}
											<span class="text-sm text-gray-500 dark:text-gray-400"> ( {formatRackPosition(device.rack_position)} )</span>
										{/if}
									{:else}
										-
									{/if}
								</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">랙 ID</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{device.rack_id || '-'}</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">랙 유닛 수</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">
									{device.rack_size ? `${device.rack_size}U` : '-'}
								</p>
							</div>
							<div>
								<label class="text-sm text-gray-500 dark:text-gray-400">소비 전력</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{formatPower(device.power_consumption)}</p>
							</div>
						</div>
					</div>

					<!-- Assigned IP Addresses -->
					<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">할당된 IP 주소</h2>
							<button
								onclick={handleAssignIp}
								class="flex items-center gap-2 rounded-lg bg-green-600 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-green-700"
							>
								<Plus class="h-4 w-4" />
								새 IP 할당
							</button>
						</div>

						{#if sortedAssignedIps.length > 0}
							<div class="flex flex-wrap gap-2">
								{#each sortedAssignedIps as ip (ip.id)}
									<span
										class="inline-flex items-center gap-2 rounded-full bg-blue-100 px-3 py-1 text-sm font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200"
									>
										{ip.ip_address || '-'}
										<button
											onclick={() => handleUnassignIp(ip)}
											class="rounded-full bg-blue-200 px-2 text-xs text-blue-800 transition-colors hover:bg-blue-300 dark:bg-blue-800 dark:text-blue-100 dark:hover:bg-blue-700"
											title="할당 해제"
										>
											×
										</button>
									</span>
								{/each}
							</div>
						{:else}
							<p class="text-sm text-gray-500 dark:text-gray-400">
								현재 할당된 IP 주소가 없습니다. 상단의 IP 할당 버튼을 눌러 IP를 연결하세요.
							</p>
						{/if}
					</div>

					<!-- Assigned Contacts -->
					<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">연결된 담당자</h2>
							<button
								onclick={handleAssignContact}
								class="flex items-center gap-2 rounded-lg bg-purple-600 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-purple-700"
							>
								<Plus class="h-4 w-4" />
								담당자 연결
							</button>
						</div>

						{#if sortedAssignedContacts.length > 0}
							<div class="space-y-2">
								{#each sortedAssignedContacts as contact (contact.id)}
									<div class="flex items-center justify-between rounded-lg border border-gray-200 p-3 transition hover:border-purple-500 hover:bg-purple-50 dark:border-gray-700 dark:hover:border-purple-400 dark:hover:bg-purple-900/20">
										<div>
											<p class="font-medium text-gray-900 dark:text-gray-100">{contact.contact_name}</p>
											<div class="mt-1 flex flex-wrap items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
												{#if contact.role}
													<span class="rounded bg-purple-100 px-2 py-0.5 text-xs font-semibold uppercase tracking-wide text-purple-800 dark:bg-purple-800 dark:text-purple-100">
														{contact.role}
													</span>
												{/if}
												<span class="text-gray-400 dark:text-gray-500">
													연결일 {new Date(contact.created_at).toLocaleString('ko-KR')}
												</span>
											</div>
										</div>
										<button
											onclick={() => handleUnassignContact(contact)}
											class="rounded-lg border border-red-500 px-3 py-1.5 text-sm font-medium text-red-600 transition hover:bg-red-500 hover:text-white"
										>
											연결 해제
										</button>
									</div>
								{/each}
							</div>
						{:else}
							<p class="text-sm text-gray-500 dark:text-gray-400">
								연결된 담당자가 없습니다. 상단 버튼을 눌러 담당자를 지정하세요.
							</p>
						{/if}
					</div>
				</div>

				<div class="space-y-6">
					<!-- Metadata -->
					<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
						<h2 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">메타데이터</h2>
						<div class="space-y-4 text-sm">
							<div>
								<label class="text-gray-500 dark:text-gray-400">디바이스 ID</label>
								<p class="mt-1 break-all font-mono text-gray-900 dark:text-gray-100">{device.id}</p>
							</div>
							<div>
								<label class="text-gray-500 dark:text-gray-400">데이터 출처</label>
								<div class="mt-1">
									<span class="rounded-full px-2 py-1 text-xs font-medium {sourceBadgeClass}">{sourceLabel}</span>
								</div>
							</div>
							<div>
								<label class="text-gray-500 dark:text-gray-400">생성자</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{device.created_by}</p>
							</div>
							<div>
								<label class="text-gray-500 dark:text-gray-400">생성일</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{formatDateTime(device.created_at)}</p>
							</div>
							<div>
								<label class="text-gray-500 dark:text-gray-400">업데이트</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">{formatDateTime(device.updated_at)}</p>
							</div>
							<div>
								<label class="text-gray-500 dark:text-gray-400">외부 연동 ID</label>
								<p class="mt-1 text-gray-900 dark:text-gray-100">
									{device.external_api_connection_id ? `#${device.external_api_connection_id}` : '-'}
								</p>
							</div>
						</div>
					</div>

					<!-- Linked Library -->
					<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
						<div class="mb-4 flex items-center justify-between">
							<div class="flex items-center gap-2">
								<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
									<BookOpen class="h-5 w-5 text-purple-600 dark:text-purple-300" />
								</div>
								<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">라이브러리 템플릿</h2>
							</div>
							<div class="flex items-center gap-2">
								<button
									onclick={handleNavigateToLibrary}
									class="flex items-center gap-1 rounded-lg border border-gray-300 px-3 py-1.5 text-sm text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
								>
									<ExternalLink class="h-4 w-4" />
									라이브러리 목록
								</button>
								{#if isLibraryLinked}
									<button
										onclick={handleUnlinkLibrary}
										class="flex items-center gap-1 rounded-lg border border-red-500 px-3 py-1.5 text-sm text-red-600 transition-colors hover:bg-red-500 hover:text-white dark:border-red-400 dark:text-red-300 dark:hover:bg-red-500 dark:hover:text-white"
									>
										Unlink
									</button>
								{:else}
									<button
										onclick={() => (showLibraryLinkDialog = true)}
										class="flex items-center gap-1 rounded-lg border border-purple-500 px-3 py-1.5 text-sm text-purple-600 transition-colors hover:bg-purple-500 hover:text-white dark:border-purple-400 dark:text-purple-300 dark:hover:bg-purple-500 dark:hover:text-white"
									>
										템플릿 연결
									</button>
								{/if}
							</div>
						</div>

						{#if linkedLibrary}
							<div class="space-y-3">
								<div>
									<p class="text-sm font-medium text-gray-500 dark:text-gray-400">라이브러리 이름</p>
									<p class="text-base font-semibold text-gray-900 dark:text-gray-100">{linkedLibrary.name}</p>
								</div>
								{#if linkedLibrary.description}
									<div>
										<p class="text-sm font-medium text-gray-500 dark:text-gray-400">설명</p>
										<p class="text-sm text-gray-700 dark:text-gray-300">{linkedLibrary.description}</p>
									</div>
								{/if}
								<div class="grid grid-cols-1 gap-3 text-sm text-gray-700 dark:text-gray-200 sm:grid-cols-2">
									<div>
										<p class="text-xs uppercase text-gray-500 dark:text-gray-400">장비 유형</p>
										<p>{linkedLibrary.device_type}</p>
									</div>
									<div>
										<p class="text-xs uppercase text-gray-500 dark:text-gray-400">제조사</p>
										<p>{linkedLibrary.manufacturer || '-'}</p>
									</div>
									<div>
										<p class="text-xs uppercase text-gray-500 dark:text-gray-400">모델</p>
										<p>{linkedLibrary.model || '-'}</p>
									</div>
									<div>
										<p class="text-xs uppercase text-gray-500 dark:text-gray-400">기본 랙 크기</p>
										<p>{linkedLibrary.default_rack_size ? `${linkedLibrary.default_rack_size}U` : '-'}</p>
									</div>
								</div>
							</div>
						{:else}
							<div class="rounded-lg border border-dashed border-gray-300 p-4 text-sm text-gray-500 dark:border-gray-700 dark:text-gray-400">
								연결된 라이브러리가 없습니다. 라이브러리 페이지에서 템플릿을 연결할 수 있습니다.
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<DeviceFormDialog
	open={showEditDialog}
	onClose={handleEditDialogClose}
	onSuccess={handleEditSuccess}
	editData={device}
/>

{#if device}
	<IpAssignDialog
		open={showIpAssignDialog}
		deviceId={device.id}
		deviceName={device.name}
		onClose={handleIpAssignDialogClose}
		onSuccess={handleIpAssignSuccess}
	/>
	<ContactAssignDialog
		open={showContactAssignDialog}
		deviceId={device.id}
		deviceName={device.name}
		onClose={handleContactAssignDialogClose}
		onSuccess={handleContactAssignSuccess}
	/>
{/if}

{#if showLibraryLinkDialog && device}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
		<div class="w-full max-w-xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-900">
			<div class="mb-4 flex items-center justify-between">
				<div class="flex items-center gap-2">
					<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
						<BookOpen class="h-5 w-5 text-purple-600 dark:text-purple-300" />
					</div>
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">라이브러리 템플릿 연결</h3>
						<p class="text-sm text-gray-500 dark:text-gray-400">디바이스에 대응되는 템플릿을 선택하세요.</p>
					</div>
				</div>
				<button
					onclick={() => {
						showLibraryLinkDialog = false;
						selectedLibraryIdForLink = '';
					}}
					class="rounded-full p-1 text-gray-500 transition-colors hover:bg-gray-200 dark:text-gray-400 dark:hover:bg-gray-700"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<form onsubmit={handleLibraryLinkSubmit} class="space-y-4">
				<div>
					<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">라이브러리 선택</label>
					<select
						bind:value={selectedLibraryIdForLink}
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-purple-500 focus:outline-none focus:ring-2 focus:ring-purple-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100"
						required
					>
						<option value="" disabled>라이브러리를 선택하세요</option>
						{#if linkableLibraries.length > 0}
							{#each linkableLibraries as library}
								<option value={library.id}>
									{library.name}
									{library.device_type ? ` (${library.device_type})` : ''}
									{library.device_id === device?.id ? ' - 현재 연결됨' : ''}
								</option>
							{/each}
						{:else}
							<option value="" disabled>연결 가능한 라이브러리가 없습니다</option>
						{/if}
						{#if librariesLinkedElsewhere.length > 0}
							<optgroup label="다른 디바이스에 연결된 라이브러리">
								{#each librariesLinkedElsewhere as library}
									<option value={library.id} disabled>
										{library.name}
										{library.device_type ? ` (${library.device_type})` : ''}
										- {library.device_name || '다른 디바이스'} 연결됨
									</option>
								{/each}
							</optgroup>
						{/if}
					</select>
				</div>

				<div class="flex justify-end gap-2">
					<button
						type="button"
						onclick={() => {
							showLibraryLinkDialog = false;
							selectedLibraryIdForLink = '';
						}}
						class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-purple-600 px-4 py-2 text-white transition-colors hover:bg-purple-700 disabled:opacity-50"
						disabled={isLinkingLibrary}
					>
						{isLinkingLibrary ? '연결 중...' : '연결하기'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

{#if showRackLinkDialog && device}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
		<div class="w-full max-w-xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-900">
			<div class="mb-4 flex items-center justify-between">
				<div class="flex items-center gap-2">
					<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
						<HardDrive class="h-5 w-5 text-blue-600 dark:text-blue-300" />
					</div>
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">랙 연결</h3>
						<p class="text-sm text-gray-500 dark:text-gray-400">디바이스를 배치할 랙과 U 위치를 선택하세요.</p>
					</div>
				</div>
				<button
					onclick={() => {
						showRackLinkDialog = false;
						selectedRackIdForLink = '';
						selectedRackPositionForLink = '';
					}}
					class="rounded-full p-1 text-gray-500 transition-colors hover:bg-gray-200 dark:text-gray-400 dark:hover:bg-gray-700"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<form onsubmit={handleRackLinkSubmit} class="space-y-4">
				<div>
					<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">랙 선택</label>
					<select
						bind:value={selectedRackIdForLink}
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100"
					>
						<option value="">연결 해제</option>
						{#each linkableRacks as rackItem}
							<option value={rackItem.id}>
								{rackItem.name}
								{rackItem.height ? ` (${rackItem.height}U)` : ''}
							</option>
						{/each}
					</select>
				</div>

				{#if selectedRackIdForLink}
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">랙 위치 (U)</label>
						<input
							type="number"
							min="1"
							bind:value={selectedRackPositionForLink}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100"
							placeholder="예: 10"
						/>
						<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">상단에서부터 U 위치를 입력하세요.</p>
					</div>
				{/if}

				<div class="flex justify-end gap-2">
					<button
						type="button"
						onclick={() => {
							showRackLinkDialog = false;
							selectedRackIdForLink = '';
							selectedRackPositionForLink = '';
						}}
						class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:opacity-50"
						disabled={isLinkingRack}
					>
						{isLinkingRack ? '저장 중...' : '저장'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
