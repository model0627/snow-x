<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { privateApi } from '$lib/api/private';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { onMount } from 'svelte';

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
	}

	interface ServerRoom {
		id: string;
		office_id: string;
		name: string;
		description?: string;
		floor_level?: string;
		room_number?: string;
		temperature_monitoring: boolean;
		humidity_monitoring: boolean;
		access_control: boolean;
		created_at: string;
		updated_at: string;
	}

	interface ServerRoomListResponse {
		server_rooms: ServerRoom[];
		total_count: number;
		page: number;
		limit: number;
		total_pages: number;
	}

	interface CreateRackRequest {
		server_room_id?: string;
		name: string;
		description?: string;
		rack_height: number;
		power_capacity?: number;
		cooling_type?: string;
		location_x?: number;
		location_y?: number;
	}

	interface Props {
		open: boolean;
		rack?: Rack | null;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, rack = null, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let serverRoomsLoading = $state(false);
	let serverRooms = $state<ServerRoom[]>([]);
	let selectedServerRoomId = $state('');
	let formData = $state({
		name: '',
		description: '',
		rack_height: 42,
		power_capacity: undefined as number | undefined,
		cooling_type: '',
		location_x: undefined as number | undefined,
		location_y: undefined as number | undefined
	});

	const isDesktop = $derived(desktopStore.isDesktop);
	const isEdit = $derived(rack !== null);
	const title = $derived(isEdit ? '랙 수정' : '새 랙 추가');

	// Load server rooms
	async function loadServerRooms() {
		serverRoomsLoading = true;
		try {
			// 10A 사무실 ID
			const officeId = '299a5075-1460-4bc9-9fe1-3d227f897dcd';
			const response = await privateApi.get(`v0/ipam/office/${officeId}/server-room`).json<ServerRoomListResponse>();
			serverRooms = response.server_rooms;

			// 기본값으로 첫 번째 서버실 선택
			if (serverRooms.length > 0 && !selectedServerRoomId) {
				selectedServerRoomId = serverRooms[0].id;
			}
		} catch (error) {
			console.error('Failed to load server rooms:', error);
			serverRooms = [];
		} finally {
			serverRoomsLoading = false;
		}
	}

	// Reset form when dialog opens/closes or rack changes
	$effect(() => {
		if (open) {
			// 서버실 목록 로드
			loadServerRooms();

			if (rack) {
				// Edit mode - populate form with rack data
				formData = {
					name: rack.name,
					description: rack.description || '',
					rack_height: rack.rack_height,
					power_capacity: rack.power_capacity,
					cooling_type: rack.cooling_type || '',
					location_x: rack.location_x,
					location_y: rack.location_y
				};
				selectedServerRoomId = rack.server_room_id;
			} else {
				// Create mode - reset form
				formData = {
					name: '',
					description: '',
					rack_height: 42,
					power_capacity: undefined,
					cooling_type: '',
					location_x: undefined,
					location_y: undefined
				};
				selectedServerRoomId = '';
			}
		}
	});

	async function handleSubmit() {
		if (!formData.name.trim() || formData.rack_height <= 0) {
			alert('랙명과 랙 높이는 필수입니다.');
			return;
		}

		if (!selectedServerRoomId) {
			alert('서버실을 선택해주세요.');
			return;
		}

		loading = true;
		try {
			if (isEdit && rack) {
				// 수정 API는 아직 구현되지 않음
				alert('랙 수정 기능은 아직 구현되지 않았습니다.');
			} else {
				const createData: CreateRackRequest = {
					server_room_id: selectedServerRoomId,
					name: formData.name,
					rack_height: formData.rack_height,
					description: formData.description || undefined,
					power_capacity: formData.power_capacity,
					cooling_type: formData.cooling_type || undefined,
					location_x: formData.location_x,
					location_y: formData.location_y
				};

				await privateApi.post(`v0/ipam/racks`, { json: createData });
			}

			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to save rack:', error);
			alert(`랙 ${isEdit ? '수정' : '등록'}에 실패했습니다.`);
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="w-full max-w-lg border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader class="border-b border-gray-200 pb-4 dark:border-gray-700">
			<DialogTitle class="{isDesktop ? 'text-sm' : 'text-lg'} font-semibold text-gray-900 dark:text-white"
				>{title}</DialogTitle
			>
			<p class="{isDesktop ? 'text-xs' : 'text-sm'} mt-1 text-gray-500 dark:text-gray-400">
				랙 정보를 {isEdit ? '수정' : '등록'}하세요
			</p>
		</DialogHeader>

		<form on:submit|preventDefault={handleSubmit} class="space-y-5 pt-4">
			<div>
				<label
					for="server_room"
					class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					서버실 <span class="text-red-500">*</span>
				</label>
				{#if serverRoomsLoading}
					<div
						class="w-full px-3 py-2.5 {isDesktop
							? 'text-xs'
							: 'text-sm'} rounded-lg border border-gray-300 bg-gray-50 text-gray-500 dark:border-gray-600 dark:bg-gray-700"
					>
						서버실을 불러오는 중...
					</div>
				{:else}
					<select
						id="server_room"
						bind:value={selectedServerRoomId}
						required
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 transition-colors focus:border-purple-500 focus:ring-2
							focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					>
						<option value="">서버실을 선택하세요</option>
						{#each serverRooms as serverRoom (serverRoom.id)}
							<option value={serverRoom.id}>
								{serverRoom.name}
								{#if serverRoom.room_number}
									({serverRoom.room_number})
								{/if}
								{#if serverRoom.floor_level}
									- {serverRoom.floor_level}
								{/if}
							</option>
						{/each}
					</select>
				{/if}
			</div>

			<div>
				<label
					for="name"
					class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					랙명 <span class="text-red-500">*</span>
				</label>
				<input
					id="name"
					type="text"
					bind:value={formData.name}
					required
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2
						focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="랙명을 입력하세요"
				/>
			</div>

			<div>
				<label
					for="description"
					class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					설명
				</label>
				<textarea
					id="description"
					bind:value={formData.description}
					rows="3"
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} resize-none rounded-lg border border-gray-300
						bg-white text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500
						focus:ring-2 focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="랙 설명을 입력하세요"
				></textarea>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div>
					<label
						for="rack_height"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						랙 높이 (U) <span class="text-red-500">*</span>
					</label>
					<input
						id="rack_height"
						type="number"
						bind:value={formData.rack_height}
						required
						min="1"
						max="100"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2
							focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="42"
					/>
				</div>

				<div>
					<label
						for="power_capacity"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						전력 용량 (W)
					</label>
					<input
						id="power_capacity"
						type="number"
						bind:value={formData.power_capacity}
						min="0"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2
							focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="2000"
					/>
				</div>
			</div>

			<div>
				<label
					for="cooling_type"
					class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					냉각 방식
				</label>
				<select
					id="cooling_type"
					bind:value={formData.cooling_type}
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
						text-gray-900 transition-colors focus:border-purple-500 focus:ring-2
						focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
				>
					<option value="">냉각 방식 선택</option>
					<option value="공랭">공랭</option>
					<option value="수랭">수랭</option>
					<option value="혼합">혼합</option>
				</select>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div>
					<label
						for="location_x"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						위치 X (m)
					</label>
					<input
						id="location_x"
						type="number"
						bind:value={formData.location_x}
						step="0.1"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2
							focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="0.0"
					/>
				</div>

				<div>
					<label
						for="location_y"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						위치 Y (m)
					</label>
					<input
						id="location_y"
						type="number"
						bind:value={formData.location_y}
						step="0.1"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2
							focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="0.0"
					/>
				</div>
			</div>

			<div class="flex justify-end space-x-3 border-t border-gray-200 pt-6 dark:border-gray-700">
				<Button
					type="button"
					variant="outline"
					onclick={onClose}
					disabled={loading}
					class="{isDesktop ? 'px-4 py-2 text-xs' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
				>
					취소
				</Button>
				<Button
					type="submit"
					disabled={loading || !formData.name.trim() || formData.rack_height <= 0 || !selectedServerRoomId}
					class="bg-purple-500 text-white hover:bg-purple-600 {isDesktop
						? 'px-4 py-2 text-xs'
						: 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? '처리 중...' : isEdit ? '수정' : '등록'}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>
