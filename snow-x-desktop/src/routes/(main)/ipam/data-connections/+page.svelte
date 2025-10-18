<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Plug, RefreshCw, Trash2, Pencil, TestTube, Activity, CheckCircle, XCircle, Clock, Settings, MoreVertical, History } from 'lucide-svelte';
	import { externalApiService, type ExternalApiConnection, type CreateConnectionRequest, type ApiStats, type ConnectionTargetType, type FieldMapping } from '$lib/api/external-api';
	import { toast, Toaster } from 'svelte-sonner';
	import FieldMappingDialog from '$lib/components/ipam/FieldMappingDialog.svelte';
	import SyncHistoryDialog from '$lib/components/ipam/SyncHistoryDialog.svelte';

	interface ConnectionFormData {
		name: string;
		base_url: string;
		description: string;
		target_type: ConnectionTargetType;
		sync_interval: number;
		is_active: boolean;
		auto_sync: boolean;
		headers: { key: string; value: string }[];
	}

	const targetTypeLabels: Record<ConnectionTargetType, string> = {
		device: '디바이스',
		device_library: '라이브러리',
		contact: '담당자'
	};

	let connections = $state<ExternalApiConnection[]>([]);
	let stats = $state<ApiStats | null>(null);
	let isLoading = $state(false);
	let showCreateDialog = $state(false);
	let showEditDialog = $state(false);
	let showFieldMappingDialog = $state(false);
	let showSyncHistoryDialog = $state(false);
	let syncHistoryConnection = $state<ExternalApiConnection | null>(null);
	let openDropdownId = $state<number | null>(null);
	let dropdownPosition = $state<{ top: number; left: number } | null>(null);
	let selectedConnection = $state<ExternalApiConnection | null>(null);
	let mappingConnection = $state<ExternalApiConnection | null>(null);
	let testingConnection = $state<number | null>(null);
	let syncingConnection = $state<number | null>(null);

	let formData = $state<ConnectionFormData>({
		name: '',
		base_url: '',
		description: '',
		target_type: 'device',
		sync_interval: 3600,
		is_active: true,
		auto_sync: false,
		headers: []
	});

	let syncIntervalMinutes = $state(60);

	async function loadConnections() {
		isLoading = true;
		try {
			connections = await externalApiService.listConnections();
		} catch (error) {
			console.error('Failed to load connections:', error);
		} finally {
			isLoading = false;
		}
	}

	async function loadStats() {
		try {
			stats = await externalApiService.getStats();
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	function openCreateDialog() {
		formData = {
			name: '',
			base_url: '',
			description: '',
			target_type: 'device',
			sync_interval: 3600,
			is_active: true,
			auto_sync: false,
			headers: []
		};
		syncIntervalMinutes = 60;
		showCreateDialog = true;
	}

	function openEditDialog(connection: ExternalApiConnection) {
		selectedConnection = connection;
		formData = {
			name: connection.name,
			base_url: connection.base_url,
			description: connection.description || '',
			target_type: connection.target_type,
			sync_interval: connection.sync_interval,
			is_active: connection.is_active,
			auto_sync: connection.auto_sync,
			headers: connection.headers ? Object.entries(connection.headers).map(([key, value]) => ({ key, value })) : []
		};
		syncIntervalMinutes = connection.sync_interval / 60;
		showEditDialog = true;
	}

	async function handleCreate(event: SubmitEvent) {
		event.preventDefault();
		try {
			const request: CreateConnectionRequest = {
				name: formData.name,
				base_url: formData.base_url,
				description: formData.description,
				target_type: formData.target_type,
				sync_interval: formData.sync_interval,
				is_active: formData.is_active,
				auto_sync: formData.auto_sync,
				headers: formData.headers.reduce((acc, h) => {
					if (h.key && h.value) acc[h.key] = h.value;
					return acc;
				}, {} as Record<string, string>)
			};

			await externalApiService.createConnection(request);
			showCreateDialog = false;
			toast.success('API 연결이 추가되었습니다.');
			await loadConnections();
			await loadStats();
		} catch (error) {
			console.error('Failed to create connection:', error);
			toast.error('연결 생성에 실패했습니다.');
		}
	}

	async function handleUpdate(event: SubmitEvent) {
		event.preventDefault();
		if (!selectedConnection) return;

		try {
			const request: Partial<CreateConnectionRequest> = {
				name: formData.name,
				base_url: formData.base_url,
				description: formData.description,
				target_type: formData.target_type,
				sync_interval: formData.sync_interval,
				is_active: formData.is_active,
				auto_sync: formData.auto_sync,
				headers: formData.headers.reduce((acc, h) => {
					if (h.key && h.value) acc[h.key] = h.value;
					return acc;
				}, {} as Record<string, string>)
			};

			console.log('[UPDATE] Sending request:', request);
			console.log('[UPDATE] formData.target_type:', formData.target_type);

			await externalApiService.updateConnection(selectedConnection.id, request);
			showEditDialog = false;
			selectedConnection = null;
			toast.success('API 연결이 수정되었습니다.');
			await loadConnections();
			await loadStats();
		} catch (error) {
			console.error('Failed to update connection:', error);
			toast.error('연결 수정에 실패했습니다.');
		}
	}

	async function handleDelete(id: number, name: string) {
		if (!confirm(`"${name}" 연결을 삭제하시겠습니까?`)) return;

		try {
			await externalApiService.deleteConnection(id);
			toast.success('API 연결이 삭제되었습니다.');
			await loadConnections();
			await loadStats();
		} catch (error) {
			console.error('Failed to delete connection:', error);
			toast.error('연결 삭제에 실패했습니다.');
		}
	}

	async function handleTest(connection: ExternalApiConnection) {
		testingConnection = connection.id;
		try {
			const result = await externalApiService.testConnection({
				base_url: connection.base_url,
				headers: connection.headers,
				auth_config: connection.auth_config
			});
			toast.success(`테스트 성공! 응답 시간: ${result.response_time_ms}ms`, {
				description: `상태: ${result.status}`
			});
		} catch (error) {
			console.error('Failed to test connection:', error);
			toast.error('연결 테스트에 실패했습니다.');
		} finally {
			testingConnection = null;
		}
	}

	async function handleSync(connection: ExternalApiConnection) {
		syncingConnection = connection.id;
		try {
			const result = await externalApiService.syncConnection(connection.id);
			toast.success('동기화가 시작되었습니다.', {
				description: `Task ID: ${result.task_id}`
			});
			await loadConnections();
			await loadStats();
		} catch (error) {
			console.error('Failed to sync connection:', error);
			toast.error('동기화 시작에 실패했습니다.');
		} finally {
			syncingConnection = null;
		}
	}

	async function handleSyncAll() {
		if (!confirm('모든 활성 연결을 동기화하시겠습니까?')) return;

		try {
			const result = await externalApiService.syncAllConnections();
			toast.success('전체 동기화가 시작되었습니다.', {
				description: `Task ID: ${result.task_id}`
			});
			await loadConnections();
			await loadStats();
		} catch (error) {
			console.error('Failed to sync all:', error);
			toast.error('전체 동기화 시작에 실패했습니다.');
		}
	}

	function openFieldMapping(connection: ExternalApiConnection) {
		mappingConnection = connection;
		showFieldMappingDialog = true;
		openDropdownId = null;
	}

	function openSyncHistory(connection: ExternalApiConnection) {
		syncHistoryConnection = connection;
		showSyncHistoryDialog = true;
		openDropdownId = null;
	}

	function toggleDropdown(connectionId: number, event: MouseEvent) {
		if (openDropdownId === connectionId) {
			openDropdownId = null;
			dropdownPosition = null;
		} else {
			const button = event.currentTarget as HTMLElement;
			const rect = button.getBoundingClientRect();
			dropdownPosition = {
				top: rect.bottom + 4,
				left: rect.right - 192 // 192px = w-48
			};
			openDropdownId = connectionId;
		}
	}

	// 드롭다운 외부 클릭 시 닫기
	$effect(() => {
		if (openDropdownId !== null) {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown-menu') && !target.closest('.dropdown-trigger')) {
					openDropdownId = null;
					dropdownPosition = null;
				}
			};
			document.addEventListener('click', handleClickOutside);
			return () => document.removeEventListener('click', handleClickOutside);
		}
	});

	async function handleFieldMappingSave(mapping: FieldMapping) {
		if (!mappingConnection) return;

		try {
			await externalApiService.updateConnection(mappingConnection.id, {
				field_mapping: mapping
			});
			toast.success('필드 매핑이 저장되었습니다.');
			await loadConnections();
		} catch (error) {
			console.error('Failed to save field mapping:', error);
			toast.error('필드 매핑 저장에 실패했습니다.');
		}
	}

	function addHeader() {
		formData.headers = [...formData.headers, { key: '', value: '' }];
	}

	function removeHeader(index: number) {
		formData.headers = formData.headers.filter((_, i) => i !== index);
	}

	function formatDateTime(dateString: string): string {
		return new Date(dateString).toLocaleString('ko-KR');
	}

	onMount(() => {
		loadConnections();
		loadStats();
	});
</script>

<Toaster position="top-right" />

<div class="min-h-screen bg-gray-50 p-6 dark:bg-gray-900">
	<div class="mx-auto max-w-7xl">
		<!-- Header -->
		<div class="mb-6 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">데이터 연결</h1>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					외부 API와 데이터를 동기화하여 디바이스, 라이브러리, 담당자 정보를 자동으로 가져옵니다
				</p>
			</div>
			<div class="flex gap-2">
				<button
					onclick={handleSyncAll}
					class="flex items-center gap-2 rounded-lg border border-orange-600 px-4 py-2 font-medium text-orange-600 transition hover:bg-orange-50 dark:hover:bg-orange-950/20"
				>
					<RefreshCw class="h-5 w-5" />
					전체 동기화
				</button>
				<button
					onclick={openCreateDialog}
					class="flex items-center gap-2 rounded-lg bg-orange-600 px-4 py-2 font-medium text-white transition hover:bg-orange-700"
				>
					<Plus class="h-5 w-5" />
					연결 추가
				</button>
			</div>
		</div>

		<!-- Stats -->
		{#if stats}
			<div class="mb-6 grid gap-4 md:grid-cols-4">
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
							<Plug class="h-5 w-5 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<p class="text-sm text-gray-500 dark:text-gray-400">전체 연결</p>
							<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{stats.total_connections}</p>
						</div>
					</div>
				</div>
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-green-100 p-2 dark:bg-green-900">
							<CheckCircle class="h-5 w-5 text-green-600 dark:text-green-400" />
						</div>
						<div>
							<p class="text-sm text-gray-500 dark:text-gray-400">활성 연결</p>
							<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{stats.active_connections}</p>
						</div>
					</div>
				</div>
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
							<Activity class="h-5 w-5 text-purple-600 dark:text-purple-400" />
						</div>
						<div>
							<p class="text-sm text-gray-500 dark:text-gray-400">오늘 동기화</p>
							<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{stats.today_syncs}</p>
						</div>
					</div>
				</div>
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-red-100 p-2 dark:bg-red-900">
							<XCircle class="h-5 w-5 text-red-600 dark:text-red-400" />
						</div>
						<div>
							<p class="text-sm text-gray-500 dark:text-gray-400">오늘 실패</p>
							<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{stats.today_failed_syncs}</p>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Connections List -->
		<div class="min-h-[1600px] overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100">
					<Plug class="h-5 w-5" />
					API 연결 목록
				</h2>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="h-8 w-8 animate-spin rounded-full border-4 border-orange-600 border-t-transparent"></div>
				</div>
			{:else if connections.length === 0}
				<div class="py-12 text-center">
					<p class="text-gray-500 dark:text-gray-400">등록된 연결이 없습니다</p>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-900">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">연결 이름</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">연결 대상</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">API URL</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">동기화 주기</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">마지막 동기화</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">상태</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">작업</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each connections as connection}
								<tr class="hover:bg-gray-50 dark:hover:bg-gray-700">
									<td class="whitespace-nowrap px-4 py-4">
										<div class="text-sm font-medium text-gray-900 dark:text-gray-100">{connection.name}</div>
										{#if connection.description}
											<div class="text-sm text-gray-500 dark:text-gray-400">{connection.description}</div>
										{/if}
									</td>
									<td class="whitespace-nowrap px-4 py-4 text-sm text-gray-900 dark:text-gray-100">
										<span class="rounded-full bg-orange-100 px-3 py-1 text-xs font-medium text-orange-800 dark:bg-orange-900 dark:text-orange-200">
											{targetTypeLabels[connection.target_type]}
										</span>
									</td>
									<td class="whitespace-nowrap px-4 py-4 text-sm text-gray-900 dark:text-gray-100">
										<code class="rounded bg-gray-100 px-2 py-1 text-xs dark:bg-gray-700">{connection.base_url}</code>
									</td>
									<td class="whitespace-nowrap px-4 py-4 text-sm text-gray-900 dark:text-gray-100">
										<div class="flex items-center gap-1">
											<Clock class="h-4 w-4 text-gray-400" />
											{connection.sync_interval / 60}분
										</div>
									</td>
									<td class="px-6 py-8 text-sm text-gray-900 dark:text-gray-100">
										{#if connection.last_sync_at}
											<div class="whitespace-nowrap">{formatDateTime(connection.last_sync_at)}</div>
											<div class="mt-1 text-xs text-gray-500">동기화 {connection.sync_count}회</div>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-4 py-4">
										<div class="flex flex-col gap-1">
											{#if connection.is_active}
												<span class="rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-800 dark:bg-green-900 dark:text-green-200">
													활성
												</span>
											{:else}
												<span class="rounded-full bg-gray-100 px-2 py-1 text-xs font-medium text-gray-800 dark:bg-gray-700 dark:text-gray-200">
													비활성
												</span>
											{/if}
											{#if connection.auto_sync}
												<span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200">
													자동동기화
												</span>
											{/if}
										</div>
									</td>
									<td class="whitespace-nowrap px-4 py-4 text-sm font-medium">
										<button
											onclick={(e) => toggleDropdown(connection.id, e)}
											class="dropdown-trigger rounded-lg p-1.5 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700"
											title="작업"
										>
											<MoreVertical class="h-5 w-5" />
										</button>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Dropdown Menu Portal -->
{#if openDropdownId !== null && dropdownPosition}
	{#each connections as connection}
		{#if connection.id === openDropdownId}
			<div
				class="dropdown-menu fixed z-50 w-48 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-gray-800 dark:ring-gray-700"
				style="top: {dropdownPosition.top}px; left: {dropdownPosition.left}px;"
			>
				<div class="py-1">
					<button
						onclick={() => { handleTest(connection); openDropdownId = null; }}
						disabled={testingConnection === connection.id}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 disabled:opacity-50 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						<TestTube class="h-4 w-4" />
						연결 테스트
					</button>
					<button
						onclick={() => { handleSync(connection); openDropdownId = null; }}
						disabled={syncingConnection === connection.id || !connection.is_active}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 disabled:opacity-50 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						<RefreshCw class="h-4 w-4" />
						동기화 실행
					</button>
					<button
						onclick={() => openSyncHistory(connection)}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						<History class="h-4 w-4" />
						동기화 히스토리
					</button>
					<button
						onclick={() => openFieldMapping(connection)}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						<Settings class="h-4 w-4" />
						필드 매핑 설정
					</button>
					<button
						onclick={() => openEditDialog(connection)}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						<Pencil class="h-4 w-4" />
						수정
					</button>
					<div class="my-1 border-t dark:border-gray-700"></div>
					<button
						onclick={() => { handleDelete(connection.id, connection.name); openDropdownId = null; }}
						class="flex w-full items-center gap-2 px-4 py-2 text-left text-sm text-red-600 hover:bg-red-50 dark:text-red-400 dark:hover:bg-red-900/20"
					>
						<Trash2 class="h-4 w-4" />
						삭제
					</button>
				</div>
			</div>
		{/if}
	{/each}
{/if}

<!-- Create Dialog -->
{#if showCreateDialog}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={(e) => {
			if (e.target === e.currentTarget) showCreateDialog = false;
		}}
	>
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-gray-100">API 연결 추가</h2>
			<form onsubmit={handleCreate}>
				<div class="grid gap-4">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">연결 이름 *</label>
						<input
							type="text"
							bind:value={formData.name}
							required
							placeholder="예: 담당자 동기화"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">연결 대상 *</label>
						<select
							bind:value={formData.target_type}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						>
							<option value="device">디바이스</option>
							<option value="device_library">라이브러리</option>
							<option value="contact">담당자</option>
						</select>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">API URL *</label>
						<input
							type="url"
							bind:value={formData.base_url}
							required
							placeholder="https://api.example.com/v1/employees"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">설명</label>
						<textarea
							bind:value={formData.description}
							rows="2"
							placeholder="API 연결에 대한 설명"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						></textarea>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">동기화 주기 (분)</label>
						<input
							type="number"
							bind:value={syncIntervalMinutes}
							min="1"
							step="1"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							oninput={(e) => {
								const target = e.target as HTMLInputElement;
								syncIntervalMinutes = parseInt(target.value);
								formData.sync_interval = parseInt(target.value) * 60;
							}}
						/>
					</div>
					<div>
						<label class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">HTTP 헤더</label>
						{#each formData.headers as header, i}
							<div class="mb-2 flex gap-2">
								<input
									type="text"
									bind:value={header.key}
									placeholder="헤더 이름 (예: Authorization)"
									class="flex-1 rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
								/>
								<input
									type="text"
									bind:value={header.value}
									placeholder="값 (예: Bearer token123)"
									class="flex-1 rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
								/>
								<button
									type="button"
									onclick={() => removeHeader(i)}
									class="rounded-lg bg-red-600 px-3 py-2 text-white hover:bg-red-700"
								>
									<Trash2 class="h-4 w-4" />
								</button>
							</div>
						{/each}
						<button
							type="button"
							onclick={addHeader}
							class="mt-2 flex items-center gap-2 rounded-lg border border-gray-300 px-3 py-2 text-sm hover:bg-gray-50 dark:border-gray-600 dark:hover:bg-gray-700"
						>
							<Plus class="h-4 w-4" />
							헤더 추가
						</button>
					</div>
					<div class="flex gap-4">
						<label class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={formData.is_active}
								class="h-4 w-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
							/>
							<span class="text-sm text-gray-700 dark:text-gray-300">활성화</span>
						</label>
						<label class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={formData.auto_sync}
								class="h-4 w-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
							/>
							<span class="text-sm text-gray-700 dark:text-gray-300">자동 동기화</span>
						</label>
					</div>
				</div>
				<div class="mt-6 flex justify-end gap-2">
					<button
						type="button"
						onclick={() => (showCreateDialog = false)}
						class="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
					>
						추가
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Edit Dialog -->
{#if showEditDialog}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={(e) => {
			if (e.target === e.currentTarget) {
				showEditDialog = false;
				selectedConnection = null;
			}
		}}
	>
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-gray-100">API 연결 수정</h2>
			<form onsubmit={handleUpdate}>
				<div class="grid gap-4">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">연결 이름 *</label>
						<input
							type="text"
							bind:value={formData.name}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">연결 대상 *</label>
						<select
							bind:value={formData.target_type}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						>
							<option value="device">디바이스</option>
							<option value="device_library">라이브러리</option>
							<option value="contact">담당자</option>
						</select>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">API URL *</label>
						<input
							type="url"
							bind:value={formData.base_url}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">설명</label>
						<textarea
							bind:value={formData.description}
							rows="2"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						></textarea>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">동기화 주기 (분)</label>
						<input
							type="number"
							bind:value={syncIntervalMinutes}
							min="1"
							step="1"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							oninput={(e) => {
								const target = e.target as HTMLInputElement;
								syncIntervalMinutes = parseInt(target.value);
								formData.sync_interval = parseInt(target.value) * 60;
							}}
						/>
					</div>
					<div>
						<label class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">HTTP 헤더</label>
						{#each formData.headers as header, i}
							<div class="mb-2 flex gap-2">
								<input
									type="text"
									bind:value={header.key}
									placeholder="헤더 이름"
									class="flex-1 rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
								/>
								<input
									type="text"
									bind:value={header.value}
									placeholder="값"
									class="flex-1 rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
								/>
								<button
									type="button"
									onclick={() => removeHeader(i)}
									class="rounded-lg bg-red-600 px-3 py-2 text-white hover:bg-red-700"
								>
									<Trash2 class="h-4 w-4" />
								</button>
							</div>
						{/each}
						<button
							type="button"
							onclick={addHeader}
							class="mt-2 flex items-center gap-2 rounded-lg border border-gray-300 px-3 py-2 text-sm hover:bg-gray-50 dark:border-gray-600 dark:hover:bg-gray-700"
						>
							<Plus class="h-4 w-4" />
							헤더 추가
						</button>
					</div>
					<div class="flex gap-4">
						<label class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={formData.is_active}
								class="h-4 w-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
							/>
							<span class="text-sm text-gray-700 dark:text-gray-300">활성화</span>
						</label>
						<label class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={formData.auto_sync}
								class="h-4 w-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
							/>
							<span class="text-sm text-gray-700 dark:text-gray-300">자동 동기화</span>
						</label>
					</div>
				</div>
				<div class="mt-6 flex justify-end gap-2">
					<button
						type="button"
						onclick={() => {
							showEditDialog = false;
							selectedConnection = null;
						}}
						class="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
					>
						저장
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Field Mapping Dialog -->
{#if showFieldMappingDialog && mappingConnection}
	<FieldMappingDialog
		bind:show={showFieldMappingDialog}
		targetType={mappingConnection.target_type}
		apiUrl={mappingConnection.base_url}
		headers={mappingConnection.headers}
		currentMapping={mappingConnection.field_mapping}
		onClose={() => {
			showFieldMappingDialog = false;
			mappingConnection = null;
		}}
		onSave={handleFieldMappingSave}
	/>
{/if}

<!-- Sync History Dialog -->
{#if showSyncHistoryDialog && syncHistoryConnection}
	<SyncHistoryDialog
		bind:open={showSyncHistoryDialog}
		connectionId={syncHistoryConnection.id}
		connectionName={syncHistoryConnection.name}
	/>
{/if}
