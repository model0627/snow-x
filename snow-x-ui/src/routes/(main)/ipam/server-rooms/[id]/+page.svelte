<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { serverRoomApi, officeApi } from '$lib/api/office';
	import type { ServerRoom, Office } from '$lib/api/office';
import {
	ArrowLeft,
	Server,
	Plus,
	Edit,
	Trash2,
	Settings,
	Shield,
	Thermometer,
	Droplets,
	Calendar,
	Building
} from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';

	let serverRoom: ServerRoom | null = $state(null);
	let office: Office | null = $state(null);
	let isLoading = $state(true);
	let error = $state('');

	// Rack data (placeholder - will be fetched from API later)
	let racks = $state([]);
	let devices = $state([]);

	// Statistics
let rackCount = $derived(racks.length);
let deviceCount = $derived(devices.length);

	onMount(async () => {
		if (!authStore.token) {
			goto('/account/signin');
			return;
		}

		await loadServerRoomDetails();
	});

	async function loadServerRoomDetails() {
		try {
			isLoading = true;
			error = '';

			// Parse the ID from the URL
			const serverRoomId = $page.params.id;

			// For now, we need to get the office ID from somewhere
			// In a real app, you might get this from the server room data
			// or from the URL structure
			const officeId = $page.url.searchParams.get('office') || '';

			if (!officeId) {
				error = 'Office ID가 필요합니다';
				return;
			}

			// Load server room details
			const [roomData, officeData] = await Promise.all([
				serverRoomApi.getServerRoom(officeId, serverRoomId),
				officeApi.getOffice(officeId)
			]);

			serverRoom = roomData;
			office = officeData;

			// TODO: Load racks and devices for this server room
			// For now, using placeholder data
			racks = [];
			devices = [];
		} catch (err) {
			console.error('Failed to load server room details:', err);
			error = '서버실 정보를 불러오는데 실패했습니다.';
		} finally {
			isLoading = false;
		}
}

	function handleEdit() {
		// TODO: Implement edit functionality
		console.log('Edit server room');
	}

	function handleDelete() {
		// TODO: Implement delete functionality
		if (confirm('정말 이 서버실을 삭제하시겠습니까?')) {
			console.log('Delete server room');
		}
	}

	function handleAddRack() {
		// TODO: Navigate to rack creation page
		console.log('Add new rack');
	}

	function handleManageRacks() {
		// TODO: Navigate to rack management page
		console.log('Manage racks');
	}

function handleManageDevices() {
	// TODO: Navigate to device management page
	console.log('Manage devices');
}

function formatDate(dateString?: string | null): string {
	if (!dateString) return '-';
	const date = new Date(dateString);
	if (Number.isNaN(date.getTime())) {
		return '-';
	}
	return date.toLocaleDateString('ko-KR', {
		year: 'numeric',
		month: 'numeric',
		day: 'numeric'
	});
}
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	{#if isLoading}
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<div class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-blue-600 dark:border-blue-400"></div>
				<p class="mt-4 text-gray-600 dark:text-gray-400">서버실 정보를 불러오는 중...</p>
			</div>
		</div>
	{:else if error}
		<div class="flex min-h-screen items-center justify-center">
			<div class="text-center">
				<p class="text-red-600 dark:text-red-400">{error}</p>
				<button
					onclick={() => goto('/ipam/server-rooms')}
					class="mt-4 rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
				>
					서버실 목록으로
				</button>
			</div>
		</div>
	{:else if serverRoom}
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
							<span>서버실 상세 정보</span>
						</button>
						<div class="flex gap-2">
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

					<!-- Server Room Title -->
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
							<Server class="h-6 w-6 text-blue-600 dark:text-blue-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">{serverRoom.name}</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">{office ? `소속 사무실: ${office.name}` : '서버실 상세 정보'}</p>
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
							<label class="text-sm text-gray-500 dark:text-gray-400">서버실명</label>
							<p class="font-medium text-gray-900 dark:text-gray-100">{serverRoom.name}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">설명</label>
							<p class="text-gray-900 dark:text-gray-100">{serverRoom.description ?? '설명 없음'}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">소속 사무실</label>
							<div class="flex items-center gap-2">
								<Building class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{office?.name ?? '-'}</p>
							</div>
						</div>
					</div>

					<div class="space-y-4">
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">생성일</label>
							<div class="flex items-center gap-2">
								<Calendar class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(serverRoom.created_at)}</p>
							</div>
						</div>
						<div>
							<label class="text-sm text-gray-500 dark:text-gray-400">수정일</label>
							<div class="flex items-center gap-2">
								<Calendar class="h-4 w-4 text-gray-500" />
								<p class="text-gray-900 dark:text-gray-100">{formatDate(serverRoom.updated_at)}</p>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Monitoring Settings -->
			<div class="mb-6 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<h2 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">모니터링 설정</h2>

				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
						<div class="flex items-center gap-3">
							<Thermometer class="h-5 w-5 text-orange-500" />
							<span class="text-gray-900 dark:text-gray-100">온도 모니터링</span>
						</div>
						<span
							class={`rounded-full px-2 py-1 text-xs font-medium ${
								serverRoom.temperature_monitoring
									? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
									: 'bg-gray-100 text-gray-800 dark:bg-gray-600 dark:text-gray-200'
							}`}
						>
							{serverRoom.temperature_monitoring ? '활성' : '비활성'}
						</span>
					</div>

					<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
						<div class="flex items-center gap-3">
							<Droplets class="h-5 w-5 text-blue-500" />
							<span class="text-gray-900 dark:text-gray-100">습도 모니터링</span>
						</div>
						<span
							class={`rounded-full px-2 py-1 text-xs font-medium ${
								serverRoom.humidity_monitoring
									? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
									: 'bg-gray-100 text-gray-800 dark:bg-gray-600 dark:text-gray-200'
							}`}
						>
							{serverRoom.humidity_monitoring ? '활성' : '비활성'}
						</span>
					</div>

					<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
						<div class="flex items-center gap-3">
							<Shield class="h-5 w-5 text-purple-500" />
							<span class="text-gray-900 dark:text-gray-100">출입 통제</span>
						</div>
						<span
							class={`rounded-full px-2 py-1 text-xs font-medium ${
								serverRoom.access_control
									? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
									: 'bg-gray-100 text-gray-800 dark:bg-gray-600 dark:text-gray-200'
							}`}
						>
							{serverRoom.access_control ? '활성' : '비활성'}
						</span>
					</div>
				</div>
			</div>

			<!-- Statistics -->
			<div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-2">
				<!-- Rack Count -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">랙</h3>
						<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
							<Server class="h-5 w-5 text-purple-600 dark:text-purple-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{rackCount}</p>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
						{rackCount ? `${rackCount}개의 랙` : '등록된 랙이 없습니다.'}
					</p>
				</div>

				<!-- Device Count -->
				<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
					<div class="mb-4 flex items-center justify-between">
						<h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">디바이스</h3>
						<div class="rounded-lg bg-green-100 p-2 dark:bg-green-900">
							<Settings class="h-5 w-5 text-green-600 dark:text-green-400" />
						</div>
					</div>
					<p class="text-3xl font-bold text-gray-900 dark:text-gray-100">{deviceCount}</p>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
						{deviceCount ? `${deviceCount}개의 디바이스` : '등록된 디바이스가 없습니다.'}
					</p>
				</div>
			</div>

			<!-- Related Items Section -->
			<div class="rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<div class="mb-6 flex items-center justify-between">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">관련 항목</h2>
				</div>

				<div class="space-y-4">
					<!-- Racks -->
					<div
						class="flex items-center justify-between rounded-lg border border-gray-200 p-4 transition-colors hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-700"
					>
						<div class="flex items-center gap-3">
							<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
								<Server class="h-5 w-5 text-purple-600 dark:text-purple-400" />
							</div>
							<div>
								<p class="font-medium text-gray-900 dark:text-gray-100">랙</p>
                                <p class="text-sm text-gray-500 dark:text-gray-400">
                                    {rackCount ? `${rackCount}개의 랙` : '등록된 랙이 없습니다.'}
                                </p>
							</div>
						</div>
						<button
							onclick={handleManageRacks}
							class="text-blue-600 transition-colors hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
						>
							관리하기 →
						</button>
					</div>

					<!-- Devices -->
					<div
						class="flex items-center justify-between rounded-lg border border-gray-200 p-4 transition-colors hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-700"
					>
						<div class="flex items-center gap-3">
							<div class="rounded-lg bg-green-100 p-2 dark:bg-green-900">
								<Settings class="h-5 w-5 text-green-600 dark:text-green-400" />
							</div>
							<div>
								<p class="font-medium text-gray-900 dark:text-gray-100">디바이스</p>
                                <p class="text-sm text-gray-500 dark:text-gray-400">
                                    {deviceCount ? `${deviceCount}개의 디바이스` : '등록된 디바이스가 없습니다.'}
                                </p>
							</div>
						</div>
						<button
							onclick={handleManageDevices}
							class="text-blue-600 transition-colors hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
						>
							관리하기 →
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
