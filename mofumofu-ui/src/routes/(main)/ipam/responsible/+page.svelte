<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { BookOpen, Plus, Search, Pencil, Trash2, X, AlertCircle, Cpu } from '@lucide/svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import {
		deviceLibraryApi,
		deviceApi,
		type DeviceLibrary,
		type CreateDeviceLibraryRequest,
		type UpdateDeviceLibraryRequest,
		type CreateDeviceRequest
	} from '$lib/api/office';

	// State
	let libraries = $state<DeviceLibrary[]>([]);
	let devices = $state<any[]>([]);
	let isLoading = $state(false);
	let searchQuery = $state('');
	let currentPage = $state(1);
	let totalCount = $state(0);
	let pageSize = $state(20);

	// Dialog state
	let showCreateDialog = $state(false);
	let showEditDialog = $state(false);
	let showDeleteDialog = $state(false);
	let showCreateDeviceDialog = $state(false);
	let selectedLibrary = $state<DeviceLibrary | null>(null);

	// Form state
	let formData = $state<CreateDeviceLibraryRequest>({
		name: '',
		description: '',
		device_type: '',
		manufacturer: '',
		model: '',
		default_rack_size: undefined,
		default_power_consumption: undefined
	});

	let formErrors = $state<Record<string, string>>({});

	// Device form state
	let deviceFormData = $state<CreateDeviceRequest>({
		name: '',
		description: '',
		device_type: '',
		manufacturer: '',
		model: '',
		serial_number: '',
		rack_size: 1,
		power_consumption: undefined,
		status: 'available'
	});

	const isDesktop = $derived(desktopStore.isDesktop);

	// Load libraries on mount
	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}
		await Promise.all([loadLibraries(), loadDevices()]);
	});

	// Load libraries from API
	async function loadLibraries() {
		isLoading = true;
		try {
			const response = await deviceLibraryApi.getDeviceLibraries({
				page: currentPage,
				limit: pageSize,
				search: searchQuery || undefined
			});
			libraries = response.libraries;
			totalCount = response.total;
		} catch (error) {
			console.error('Failed to load libraries:', error);
			alert('라이브러리 목록을 불러오는데 실패했습니다.');
		} finally {
			isLoading = false;
		}
	}

	// Load devices from API
	async function loadDevices() {
		try {
			const response = await deviceApi.getDevices({ limit: 1000 });
			devices = response.devices;
		} catch (error) {
			console.error('Failed to load devices:', error);
		}
	}

	// Search handler
	let searchTimeout: ReturnType<typeof setTimeout>;
	function handleSearch() {
		clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			currentPage = 1;
			loadLibraries();
		}, 300);
	}

	// Open create dialog
	function openCreateDialog() {
		formData = {
			name: '',
			description: '',
			device_type: '',
			manufacturer: '',
			model: '',
			default_rack_size: undefined,
			default_power_consumption: undefined
		};
		formErrors = {};
		showCreateDialog = true;
	}

	// Open edit dialog
	function openEditDialog(library: DeviceLibrary) {
		selectedLibrary = library;
		formData = {
			name: library.name,
			description: library.description || '',
			device_type: library.device_type,
			manufacturer: library.manufacturer || '',
			model: library.model || '',
			default_rack_size: library.default_rack_size || undefined,
			default_power_consumption: library.default_power_consumption || undefined,
			device_id: library.device_id || undefined,
			device_name: library.device_name || ''
		};
		formErrors = {};
		showEditDialog = true;
	}

	// Open delete dialog
	function openDeleteDialog(library: DeviceLibrary) {
		selectedLibrary = library;
		showDeleteDialog = true;
	}

	// Validate form
	function validateForm(): boolean {
		const errors: Record<string, string> = {};

		if (!formData.name.trim()) {
			errors.name = '이름을 입력해주세요';
		}

		if (!formData.device_type.trim()) {
			errors.device_type = '장비 유형을 입력해주세요';
		}

		formErrors = errors;
		return Object.keys(errors).length === 0;
	}

	// Create library
	async function handleCreate() {
		if (!validateForm()) return;

		isLoading = true;
		try {
			await deviceLibraryApi.createDeviceLibrary(formData);
			showCreateDialog = false;
			await loadLibraries();
		} catch (error) {
			console.error('Failed to create library:', error);
			alert('라이브러리 생성에 실패했습니다.');
		} finally {
			isLoading = false;
		}
	}

	// Update library
	async function handleUpdate() {
		if (!selectedLibrary || !validateForm()) return;

		isLoading = true;
		try {
			await deviceLibraryApi.updateDeviceLibrary(selectedLibrary.id, formData);
			showEditDialog = false;
			await loadLibraries();
		} catch (error) {
			console.error('Failed to update library:', error);
			alert('라이브러리 수정에 실패했습니다.');
		} finally {
			isLoading = false;
		}
	}

	// Delete library
	async function handleDelete() {
		if (!selectedLibrary) return;

		isLoading = true;
		try {
			await deviceLibraryApi.deleteDeviceLibrary(selectedLibrary.id);
			showDeleteDialog = false;
			selectedLibrary = null;
			await loadLibraries();
		} catch (error) {
			console.error('Failed to delete library:', error);
			alert('라이브러리 삭제에 실패했습니다.');
		} finally {
			isLoading = false;
		}
	}

	// Open create device from library dialog
	function openCreateDeviceDialog(library: DeviceLibrary) {
		selectedLibrary = library;
		deviceFormData = {
			name: '',
			description: library.description || '',
			device_type: library.device_type,
			manufacturer: library.manufacturer || '',
			model: library.model || '',
			serial_number: '',
			rack_size: library.default_rack_size || 1,
			power_consumption: library.default_power_consumption || undefined,
			status: 'available'
		};
		formErrors = {};
		showCreateDeviceDialog = true;
	}

	// Create device from library
	async function handleCreateDevice() {
		if (!deviceFormData.name.trim()) {
			formErrors = { name: '장비 이름을 입력해주세요' };
			return;
		}

		isLoading = true;
		try {
			await deviceApi.createDevice(deviceFormData);
			showCreateDeviceDialog = false;
			alert('디바이스가 성공적으로 생성되었습니다!');
		} catch (error) {
			console.error('Failed to create device:', error);
			alert('디바이스 생성에 실패했습니다.');
		} finally {
			isLoading = false;
		}
	}

	// Pagination
	const totalPages = $derived(Math.ceil(totalCount / pageSize));

	function goToPage(page: number) {
		currentPage = page;
		loadLibraries();
	}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="py-6">
				<div class="mb-6 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-purple-100 p-3 dark:bg-purple-900">
							<BookOpen class="h-6 w-6 text-purple-600 dark:text-purple-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">📚 장비 라이브러리</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								장비 템플릿을 관리하고 빠르게 장비를 생성할 수 있습니다.
							</p>
						</div>
					</div>
					<button
						onclick={openCreateDialog}
						class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600"
					>
						<Plus class="h-5 w-5" />
						라이브러리 추가
					</button>
				</div>

				<!-- Search -->
				<div class="relative">
					<Search class="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
					<input
						type="text"
						bind:value={searchQuery}
						oninput={handleSearch}
						placeholder="이름, 제조사, 모델로 검색..."
						class="w-full rounded-lg border border-gray-300 py-2 pl-10 pr-4 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Main Content -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<!-- Stats -->
		<div class="mb-6 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
					<BookOpen class="h-5 w-5 text-purple-600 dark:text-purple-400" />
				</div>
				<div>
					<p class="text-sm text-gray-500 dark:text-gray-400">전체 라이브러리</p>
					<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{totalCount}</p>
				</div>
			</div>
		</div>

		<!-- Library List -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100">
					<BookOpen class="h-5 w-5" />
					라이브러리 목록
				</h2>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<div
							class="mx-auto h-8 w-8 animate-spin rounded-full border-b-2 border-blue-600 dark:border-blue-400"
						></div>
						<p class="mt-2 text-gray-500 dark:text-gray-400">로딩 중...</p>
					</div>
				</div>
			{:else if libraries.length === 0}
				<div class="flex items-center justify-center py-12">
					<div class="text-center">
						<BookOpen class="mx-auto mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
						<p class="text-gray-500 dark:text-gray-400">
							{searchQuery ? '검색 결과가 없습니다.' : '등록된 라이브러리가 없습니다.'}
						</p>
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
									이름
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									장비 유형
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									제조사
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									모델
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									랙 크기
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									전력 소비
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									연결된 디바이스
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-400"
								>
									작업
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each libraries as library (library.id)}
								<tr class="transition-colors hover:bg-gray-50 dark:hover:bg-gray-700">
									<td class="px-6 py-4">
										<div class="text-sm font-medium text-gray-900 dark:text-gray-100">
											{library.name}
										</div>
										{#if library.description}
											<div class="text-xs text-gray-500 dark:text-gray-400">
												{library.description}
											</div>
										{/if}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										<span
											class="rounded-full bg-purple-100 px-2 py-1 text-xs font-medium text-purple-800 dark:bg-purple-900 dark:text-purple-200"
										>
											{library.device_type}
										</span>
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										{library.manufacturer || '-'}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										{library.model || '-'}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										{library.default_rack_size ? `${library.default_rack_size}U` : '-'}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										{library.default_power_consumption ? `${library.default_power_consumption}W` : '-'}
									</td>
									<td class="px-6 py-4 text-sm whitespace-nowrap text-gray-900 dark:text-gray-100">
										{#if library.device_name}
											<span
												class="rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-800 dark:bg-green-900 dark:text-green-200"
											>
												{library.device_name}
											</span>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-2">
											<button
												onclick={() => openCreateDeviceDialog(library)}
												class="rounded-lg p-2 text-green-600 transition-colors hover:bg-green-50 dark:text-green-400 dark:hover:bg-green-900"
												title="디바이스 생성"
											>
												<Cpu class="h-4 w-4" />
											</button>
											<button
												onclick={() => openEditDialog(library)}
												class="rounded-lg p-2 text-blue-600 transition-colors hover:bg-blue-50 dark:text-blue-400 dark:hover:bg-blue-900"
												title="수정"
											>
												<Pencil class="h-4 w-4" />
											</button>
											<button
												onclick={() => openDeleteDialog(library)}
												class="rounded-lg p-2 text-red-600 transition-colors hover:bg-red-50 dark:text-red-400 dark:hover:bg-red-900"
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

				<!-- Pagination -->
				{#if totalPages > 1}
					<div class="flex items-center justify-between border-t border-gray-200 px-6 py-4 dark:border-gray-700">
						<div class="text-sm text-gray-500 dark:text-gray-400">
							전체 {totalCount}개 중 {(currentPage - 1) * pageSize + 1}-{Math.min(
								currentPage * pageSize,
								totalCount
							)}개 표시
						</div>
						<div class="flex gap-2">
							<button
								onclick={() => goToPage(currentPage - 1)}
								disabled={currentPage === 1}
								class="rounded-lg border border-gray-300 px-3 py-1 text-sm transition-colors hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:hover:bg-gray-700"
							>
								이전
							</button>
							<button
								onclick={() => goToPage(currentPage + 1)}
								disabled={currentPage >= totalPages}
								class="rounded-lg border border-gray-300 px-3 py-1 text-sm transition-colors hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:hover:bg-gray-700"
							>
								다음
							</button>
						</div>
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>

<!-- Create/Edit Dialog -->
{#if showCreateDialog || showEditDialog}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 dark:bg-gray-800">
			<div class="mb-4 flex items-center justify-between">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					{showCreateDialog ? '라이브러리 추가' : '라이브러리 수정'}
				</h3>
				<button
					onclick={() => {
						showCreateDialog = false;
						showEditDialog = false;
					}}
					class="rounded-lg p-1 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-700"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<form
				onsubmit={(e) => {
					e.preventDefault();
					showCreateDialog ? handleCreate() : handleUpdate();
				}}
				class="space-y-4"
			>
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							이름 <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={formData.name}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="라이브러리 이름"
						/>
						{#if formErrors.name}
							<p class="mt-1 text-sm text-red-600">{formErrors.name}</p>
						{/if}
					</div>

					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							설명
						</label>
						<textarea
							bind:value={formData.description}
							rows="2"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="설명"
						></textarea>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							장비 유형 <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={formData.device_type}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="예: Server, Switch, Router"
						/>
						{#if formErrors.device_type}
							<p class="mt-1 text-sm text-red-600">{formErrors.device_type}</p>
						{/if}
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							제조사
						</label>
						<input
							type="text"
							bind:value={formData.manufacturer}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="예: Dell, HP, Cisco"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							모델
						</label>
						<input
							type="text"
							bind:value={formData.model}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="모델명"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							기본 랙 크기 (U)
						</label>
						<input
							type="number"
							bind:value={formData.default_rack_size}
							min="1"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="예: 1, 2, 4"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							기본 전력 소비 (W)
						</label>
						<input
							type="number"
							bind:value={formData.default_power_consumption}
							min="0"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="예: 500, 1000"
						/>
					</div>

					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							연결 디바이스 (선택사항)
						</label>
						<select
							bind:value={formData.device_id}
							onchange={(e) => {
								const selectedDeviceId = (e.target as HTMLSelectElement).value;
								if (selectedDeviceId) {
									const selectedDevice = devices.find((d) => d.id === selectedDeviceId);
									if (selectedDevice) {
										formData.device_name = selectedDevice.name;
									}
								} else {
									formData.device_name = '';
								}
							}}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						>
							<option value="">선택 안 함</option>
							{#each devices as device}
								<option value={device.id}>
									{device.name}
									{device.device_type ? `(${device.device_type})` : ''}
								</option>
							{/each}
						</select>
						<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
							이 라이브러리를 특정 디바이스에 연결할 수 있습니다
						</p>
					</div>
				</div>

				<div class="flex justify-end gap-2 pt-4">
					<button
						type="button"
						onclick={() => {
							showCreateDialog = false;
							showEditDialog = false;
						}}
						class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						disabled={isLoading}
						class="rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:opacity-50 dark:bg-blue-500 dark:hover:bg-blue-600"
					>
						{isLoading ? '처리 중...' : showCreateDialog ? '추가' : '수정'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Delete Confirmation Dialog -->
{#if showDeleteDialog && selectedLibrary}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
		<div class="w-full max-w-md rounded-lg bg-white p-6 dark:bg-gray-800">
			<div class="mb-4 flex items-center gap-3">
				<div class="rounded-lg bg-red-100 p-2 dark:bg-red-900">
					<AlertCircle class="h-6 w-6 text-red-600 dark:text-red-400" />
				</div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">라이브러리 삭제</h3>
			</div>

			<p class="mb-4 text-gray-600 dark:text-gray-400">
				정말로 <strong class="text-gray-900 dark:text-gray-100">{selectedLibrary.name}</strong>
				라이브러리를 삭제하시겠습니까?
			</p>

			<div class="flex justify-end gap-2">
				<button
					onclick={() => {
						showDeleteDialog = false;
						selectedLibrary = null;
					}}
					class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
				>
					취소
				</button>
				<button
					onclick={handleDelete}
					disabled={isLoading}
					class="rounded-lg bg-red-600 px-4 py-2 text-white transition-colors hover:bg-red-700 disabled:opacity-50"
				>
					{isLoading ? '삭제 중...' : '삭제'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Create Device from Library Dialog -->
{#if showCreateDeviceDialog && selectedLibrary}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 dark:bg-gray-800">
			<div class="mb-4 flex items-center justify-between">
				<div>
					<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
						라이브러리로부터 디바이스 생성
					</h3>
					<p class="text-sm text-gray-500 dark:text-gray-400">
						{selectedLibrary.name} 템플릿 사용
					</p>
				</div>
				<button
					onclick={() => {
						showCreateDeviceDialog = false;
					}}
					class="rounded-lg p-1 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-700"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleCreateDevice();
				}}
				class="space-y-4"
			>
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							장비 이름 <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={deviceFormData.name}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="예: Server-001"
						/>
						{#if formErrors.name}
							<p class="mt-1 text-sm text-red-600">{formErrors.name}</p>
						{/if}
					</div>

					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							설명
						</label>
						<textarea
							bind:value={deviceFormData.description}
							rows="2"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="설명"
						></textarea>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							장비 유형
						</label>
						<input
							type="text"
							bind:value={deviceFormData.device_type}
							readonly
							class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 dark:border-gray-600 dark:bg-gray-600 dark:text-gray-100"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							제조사
						</label>
						<input
							type="text"
							bind:value={deviceFormData.manufacturer}
							readonly
							class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 dark:border-gray-600 dark:bg-gray-600 dark:text-gray-100"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							모델
						</label>
						<input
							type="text"
							bind:value={deviceFormData.model}
							readonly
							class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 dark:border-gray-600 dark:bg-gray-600 dark:text-gray-100"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							시리얼 번호
						</label>
						<input
							type="text"
							bind:value={deviceFormData.serial_number}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							placeholder="시리얼 번호"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							랙 크기 (U)
						</label>
						<input
							type="number"
							bind:value={deviceFormData.rack_size}
							min="1"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							전력 소비 (W)
						</label>
						<input
							type="number"
							bind:value={deviceFormData.power_consumption}
							min="0"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>

					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
							상태
						</label>
						<select
							bind:value={deviceFormData.status}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						>
							<option value="available">사용 가능</option>
							<option value="in_use">사용 중</option>
							<option value="maintenance">유지보수</option>
							<option value="retired">폐기</option>
						</select>
					</div>
				</div>

				<div class="flex justify-end gap-2 pt-4">
					<button
						type="button"
						onclick={() => {
							showCreateDeviceDialog = false;
						}}
						class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						disabled={isLoading}
						class="rounded-lg bg-green-600 px-4 py-2 text-white transition-colors hover:bg-green-700 disabled:opacity-50"
					>
						{isLoading ? '생성 중...' : '디바이스 생성'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
