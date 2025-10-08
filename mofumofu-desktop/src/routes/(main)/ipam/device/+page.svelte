<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Search, Plus, HardDrive, Eye, Edit, Trash2, Server, Wifi, Network, Shield, Database, UserPlus } from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import DeviceFormDialog from '$lib/components/ipam/DeviceFormDialog.svelte';
	import IpAssignDialog from '$lib/components/ipam/IpAssignDialog.svelte';
	import { deviceApi, rackApi, type Device, type Rack, type IpAddress } from '$lib/api/office';
	import { sendDeviceAddedNotification, ensureNotificationPermission } from '$lib/utils/notification';
	import { browser } from '$app/environment';

	interface DeviceDisplay extends Device {
		type_icon: typeof Server;
		type_label: string;
		assigned_ips?: IpAddress[];
	}

	interface DeviceStats {
		total: number;
		active: number;
		inactive: number;
		maintenance: number;
	}

	let devices = $state<DeviceDisplay[]>([]);
	let racks = $state<Rack[]>([]);
	let stats = $state<DeviceStats>({
		total: 0,
		active: 0,
		inactive: 0,
		maintenance: 0
	});
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let showAddDialog = $state(false);
	let showIpAssignDialog = $state(false);
	let editingDevice = $state<Device | null>(null);
	let assigningDevice = $state<DeviceDisplay | null>(null);
	let currentPage = $state(1);
	let itemsPerPage = $state(50);
	let totalItems = $state(0);

	// Filters
	let filterDeviceType = $state('');
	let filterStatus = $state('');

	const isDesktop = $derived(desktopStore.isDesktop);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}

		// Tauri 데스크톱 앱에서만 알림 권한 요청
		if (browser && (window as any).__TAURI__) {
			await ensureNotificationPermission();
		}

		await Promise.all([loadDevices(), loadRacks()]);
	});

	function getDeviceTypeIcon(type: string): typeof Server {
		switch (type.toLowerCase()) {
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
		switch (type.toLowerCase()) {
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
				return type;
		}
	}

	async function loadRacks() {
		try {
			const response = await rackApi.getRacks({ page: 1, limit: 100 });
			racks = response.racks;
		} catch (err) {
			console.error('Failed to load racks:', err);
		}
	}

	async function loadDevices() {
		try {
			isLoading = true;
			error = '';

			console.log('Loading devices...');
			const response = await deviceApi.getDevices({
				page: currentPage,
				limit: itemsPerPage,
				search: searchQuery || undefined,
				device_type: filterDeviceType || undefined,
				status: filterStatus || undefined
			});
			console.log('API Response:', response);

			totalItems = response.total;

			// Transform API response to display format
			const devicesWithMetadata = response.devices.map((device) => ({
				...device,
				type_icon: getDeviceTypeIcon(device.device_type),
				type_label: getDeviceTypeLabel(device.device_type)
			}));

			// Load assigned IPs for each device in parallel
			const devicesWithIps = await Promise.all(
				devicesWithMetadata.map(async (device) => {
					try {
						const assignedIps = await deviceApi.getAssignedIpAddresses(device.id);
						return { ...device, assigned_ips: assignedIps };
					} catch (err) {
						console.error(`Failed to load IPs for device ${device.id}:`, err);
						return { ...device, assigned_ips: [] };
					}
				})
			);

			devices = devicesWithIps;

			// Calculate stats
			stats = {
				total: response.total,
				active: devices.filter((d) => d.status === 'active').length,
				inactive: devices.filter((d) => d.status === 'inactive').length,
				maintenance: devices.filter((d) => d.status === 'maintenance').length
			};

			console.log('Transformed devices:', devices);
			console.log('Stats:', stats);
		} catch (err) {
			console.error('Failed to load devices:', err);
			error = '디바이스 목록을 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
	}

	function handleAddDevice() {
		editingDevice = null;
		showAddDialog = true;
	}

	async function handleAddSuccess(newDevice?: Device) {
		console.log('🎉 Device added successfully:', newDevice);

		// Always reload to get the latest data
		await loadDevices();

		// Tauri 데스크톱 앱에서만 알림 전송
		if (browser && (window as any).__TAURI__) {
			console.log('🖥️ Running in Tauri, checking notification...');
			if (newDevice) {
				console.log('📱 Sending notification for device:', newDevice.name);
				try {
					await sendDeviceAddedNotification({
						deviceId: newDevice.id,
						deviceName: newDevice.name,
						deviceType: getDeviceTypeLabel(newDevice.device_type)
					});
					console.log('✅ Notification sent successfully');
				} catch (error) {
					console.error('❌ Failed to send notification:', error);
				}
			} else {
				console.warn('⚠️ No device data received from dialog');
			}
		} else {
			console.log('🌐 Not running in Tauri or not in browser');
		}
	}

	function handleViewDevice(device: DeviceDisplay) {
		console.log('View device:', device);
		// TODO: Navigate to device detail page
	}

	async function handleEditDevice(device: DeviceDisplay) {
		try {
			// Fetch full device data from API
			const fullDevice = await deviceApi.getDevice(device.id);
			editingDevice = fullDevice;
			showAddDialog = true;
		} catch (err) {
			console.error('Failed to load device for editing:', err);
			alert('디바이스 정보를 불러오는데 실패했습니다.');
		}
	}

	async function handleDeleteDevice(device: DeviceDisplay) {
		if (confirm(`정말 ${device.name} 디바이스를 삭제하시겠습니까?`)) {
			try {
				await deviceApi.deleteDevice(device.id);
				// Reload the list after successful deletion
				await loadDevices();
			} catch (err) {
				console.error('Failed to delete device:', err);
				alert('디바이스 삭제에 실패했습니다.');
			}
		}
	}

	function handleDialogClose() {
		showAddDialog = false;
		editingDevice = null;
	}

	function handleAssignIp(device: DeviceDisplay) {
		assigningDevice = device;
		showIpAssignDialog = true;
	}

	function handleIpAssignDialogClose() {
		showIpAssignDialog = false;
		assigningDevice = null;
	}

	async function handleIpAssignSuccess() {
		await loadDevices();
	}

	async function handleUnassignIp(device: DeviceDisplay, ip: IpAddress) {
		try {
			await deviceApi.unassignIpAddress(device.id, ip.id);
			await loadDevices();
		} catch (error) {
			console.error('Failed to unassign IP address:', error);
		}
	}

	// Filter devices based on search query
	const filteredDevices = $derived.by(() => {
		if (!searchQuery.trim()) return devices;

		const query = searchQuery.toLowerCase();
		return devices.filter(
			(device) =>
				device.name.toLowerCase().includes(query) ||
				device.device_type.toLowerCase().includes(query) ||
				device.manufacturer?.toLowerCase().includes(query) ||
				device.model?.toLowerCase().includes(query) ||
				device.serial_number?.toLowerCase().includes(query) ||
				device.description?.toLowerCase().includes(query)
		);
	});

	function getStatusColor(status: string): string {
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

	function getStatusLabel(status: string): string {
		switch (status) {
			case 'active':
				return '활성';
			case 'inactive':
				return '비활성';
			case 'maintenance':
				return '점검';
			default:
				return status;
		}
	}

	function formatDate(dateString?: string): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleDateString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}

	function getRackName(rackId?: string): string {
		if (!rackId) return '-';
		const rack = racks.find((r) => r.id === rackId);
		return rack ? rack.name : '-';
	}

	const totalPages = $derived(Math.ceil(totalItems / itemsPerPage));

	function handlePageChange(page: number) {
		currentPage = page;
		loadDevices();
	}

	function handleSearch() {
		currentPage = 1;
		loadDevices();
	}

	function handleFilterChange() {
		currentPage = 1;
		loadDevices();
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="py-6">
				<!-- Title and Add Button -->
				<div class="mb-6 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
							<HardDrive class="h-6 w-6 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">디바이스 관리</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">서버, 네트워크 장비 등을 관리합니다.</p>
						</div>
					</div>
					<button
						onclick={handleAddDevice}
						class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
					>
						<Plus class="h-4 w-4" />
						디바이스 추가
					</button>
				</div>

				<!-- Search and Filters -->
				<div class="flex flex-col gap-4 md:flex-row">
					<div class="relative max-w-md flex-1">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<Search class="h-4 w-4 text-gray-400" />
						</div>
						<input
							type="text"
							bind:value={searchQuery}
							onkeydown={(e) => e.key === 'Enter' && handleSearch()}
							placeholder="디바이스명, 모델, 시리얼 번호로 검색..."
							class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10
								text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
								focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white
								{isDesktop ? 'text-sm' : 'text-base'}"
						/>
					</div>
					<div class="flex gap-2">
						<select
							bind:value={filterDeviceType}
							onchange={handleFilterChange}
							class="rounded-lg border border-gray-300 bg-white px-3 py-2.5 text-gray-900 dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop
								? 'text-sm'
								: 'text-base'}"
						>
							<option value="">모든 타입</option>
							<option value="server">서버</option>
							<option value="switch">스위치</option>
							<option value="router">라우터</option>
							<option value="firewall">방화벽</option>
							<option value="storage">스토리지</option>
						</select>
						<select
							bind:value={filterStatus}
							onchange={handleFilterChange}
							class="rounded-lg border border-gray-300 bg-white px-3 py-2.5 text-gray-900 dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop
								? 'text-sm'
								: 'text-base'}"
						>
							<option value="">모든 상태</option>
							<option value="active">활성</option>
							<option value="inactive">비활성</option>
							<option value="maintenance">점검</option>
						</select>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Status Counts -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="mb-6 flex items-center gap-6 text-sm">
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-gray-400"></div>
				<span class="text-gray-600 dark:text-gray-400">총 {stats.total}개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-green-500"></div>
				<span class="text-gray-600 dark:text-gray-400">활성 {stats.active}개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-gray-500"></div>
				<span class="text-gray-600 dark:text-gray-400">비활성 {stats.inactive}개</span>
			</div>
			<div class="flex items-center gap-2">
				<div class="h-3 w-3 rounded-full bg-orange-500"></div>
				<span class="text-gray-600 dark:text-gray-400">점검 {stats.maintenance}개</span>
			</div>
		</div>

		<!-- Devices Table -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">디바이스 목록</h2>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					{filteredDevices.length}개 디바이스
				</p>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<div
							class="mx-auto h-8 w-8 animate-spin rounded-full border-b-2 border-blue-600 dark:border-blue-400"
						></div>
						<p class="mt-2 text-gray-500 dark:text-gray-400 {isDesktop ? 'text-sm' : 'text-base'}">로딩 중...</p>
					</div>
				</div>
			{:else if error}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<p class="mb-4 text-red-600 dark:text-red-400">{error}</p>
						<button
							onclick={() => loadDevices()}
							class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
						>
							다시 시도
						</button>
					</div>
				</div>
			{:else if filteredDevices.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<HardDrive class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
						<p class="mb-4 text-gray-500 dark:text-gray-400">
							{searchQuery ? '검색 결과가 없습니다.' : '등록된 디바이스가 없습니다.'}
						</p>
						{#if !searchQuery}
							<button
								onclick={handleAddDevice}
								class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
							>
								첫 번째 디바이스 추가
							</button>
						{/if}
					</div>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-700">
							<tr>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									디바이스명
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									타입
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									모델
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									시리얼 번호
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									상태
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									할당된 IP
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									담당자
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									랙 위치
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									데이터 출처
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									등기일
								</th>
								<th
									class="px-6 py-3 text-right text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									작업
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each filteredDevices as device (device.id)}
								<tr class="transition-colors hover:bg-gray-50 dark:hover:bg-gray-700">
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="text-sm font-medium text-gray-900 dark:text-gray-100">
											{device.name}
										</div>
										{#if device.description}
											<div class="text-xs text-gray-500 dark:text-gray-400">
												{device.description}
											</div>
										{/if}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center">
											<div
												class="mr-2 flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900"
											>
												<svelte:component this={device.type_icon} class="h-4 w-4 text-blue-600 dark:text-blue-400" />
											</div>
											<span class="text-sm text-gray-900 dark:text-gray-100">{device.type_label}</span>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="text-sm text-gray-900 dark:text-gray-100">
											{device.manufacturer || '-'}
										</div>
										{#if device.model}
											<div class="text-xs text-gray-500 dark:text-gray-400">
												{device.model}
											</div>
										{/if}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
										{device.serial_number || '-'}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<span class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(device.status)}">
											{getStatusLabel(device.status)}
										</span>
									</td>
									<td class="px-6 py-4 text-sm">
										<div class="flex flex-col gap-1">
											{#if device.assigned_ips && device.assigned_ips.length > 0}
												<div class="mb-1 flex flex-wrap gap-1">
													{#each device.assigned_ips as ip}
														<span
															class="inline-flex items-center gap-1 rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200"
														>
															{ip.ip_address}
															<button
																onclick={() => handleUnassignIp(device, ip)}
																class="transition-colors hover:text-red-600 dark:hover:text-red-400"
																title="할당 해제"
															>
																×
															</button>
														</span>
													{/each}
												</div>
											{/if}
											<button
												onclick={() => handleAssignIp(device)}
												class="text-left text-blue-600 transition-colors hover:text-blue-800 hover:underline dark:text-blue-400 dark:hover:text-blue-300"
											>
												+ IP 할당
											</button>
										</div>
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400"> 담당자 없음 </td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
										{#if device.rack_id}
											{getRackName(device.rack_id)}
											{#if device.rack_position}
												(U{device.rack_position})
											{/if}
										{:else}
											-
										{/if}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										{#if device.source_type === 'api_sync'}
											<span class="inline-flex items-center gap-1 rounded-full bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200">
												<Database class="h-3 w-3" />
												API 동기화
											</span>
										{:else}
											<span class="inline-flex items-center gap-1 rounded-full bg-gray-100 px-2 py-1 text-xs font-medium text-gray-800 dark:bg-gray-700 dark:text-gray-200">
												<UserPlus class="h-3 w-3" />
												수동 입력
											</span>
										{/if}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">
										{formatDate(device.purchase_date)}
									</td>
									<td class="px-6 py-4 text-right text-sm font-medium whitespace-nowrap">
										<div class="flex items-center justify-end gap-2">
											<button
												onclick={() => handleViewDevice(device)}
												class="text-gray-600 transition-colors hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
												title="상세보기"
											>
												<Eye class="h-4 w-4" />
											</button>
											<button
												onclick={() => handleEditDevice(device)}
												class="text-gray-600 transition-colors hover:text-yellow-600 dark:text-gray-400 dark:hover:text-yellow-400"
												title="수정"
											>
												<Edit class="h-4 w-4" />
											</button>
											<button
												onclick={() => handleDeleteDevice(device)}
												class="text-gray-600 transition-colors hover:text-red-600 dark:text-gray-400 dark:hover:text-red-400"
												title="삭제"
											>
												<Trash2 class="h-4 w-4" />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="mt-6 flex items-center justify-between">
				<div class="text-sm text-gray-700 dark:text-gray-300">
					총 {totalItems}개 중 {(currentPage - 1) * itemsPerPage + 1} - {Math.min(
						currentPage * itemsPerPage,
						totalItems
					)}개 표시
				</div>
				<div class="flex gap-2">
					<button
						onclick={() => handlePageChange(currentPage - 1)}
						disabled={currentPage === 1}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						이전
					</button>

					{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
						{#if page === 1 || page === totalPages || (page >= currentPage - 2 && page <= currentPage + 2)}
							<button
								onclick={() => handlePageChange(page)}
								class="rounded-lg border px-3 py-2 {page === currentPage
									? 'border-blue-600 bg-blue-600 text-white'
									: 'border-gray-300 bg-white text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'}"
							>
								{page}
							</button>
						{:else if page === currentPage - 3 || page === currentPage + 3}
							<span class="px-3 py-2">...</span>
						{/if}
					{/each}

					<button
						onclick={() => handlePageChange(currentPage + 1)}
						disabled={currentPage === totalPages}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						다음
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Device Add/Edit Dialog -->
<DeviceFormDialog
	open={showAddDialog}
	onClose={handleDialogClose}
	onSuccess={handleAddSuccess}
	editData={editingDevice}
/>

<!-- IP Assign Dialog -->
{#if assigningDevice}
	<IpAssignDialog
		open={showIpAssignDialog}
		deviceId={assigningDevice.id}
		deviceName={assigningDevice.name}
		onClose={handleIpAssignDialogClose}
		onSuccess={handleIpAssignSuccess}
	/>
{/if}
