<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import {
		serverRoomApi,
		type ServerRoom,
		type CreateServerRoomRequest,
		type UpdateServerRoomRequest
	} from '$lib/api/office';
	import { desktopStore } from '$lib/stores/desktop.svelte';

	interface Props {
		open: boolean;
		officeId: string;
		serverRoom?: ServerRoom | null;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, officeId, serverRoom = null, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let formData = $state({
		name: '',
		description: '',
		floor_level: '',
		room_number: '',
		temperature_monitoring: false,
		humidity_monitoring: false,
		access_control: false
	});

	const isDesktop = $derived(desktopStore.isDesktop);
	const isEdit = $derived(serverRoom !== null);
	const title = $derived(isEdit ? '서버실 수정' : '새 서버실 추가');

	// Reset form when dialog opens/closes or server room changes
	$effect(() => {
		if (open) {
			if (serverRoom) {
				// Edit mode - populate form with server room data
				formData = {
					name: serverRoom.name,
					description: serverRoom.description || '',
					floor_level: serverRoom.floor_level || '',
					room_number: serverRoom.room_number || '',
					temperature_monitoring: serverRoom.temperature_monitoring,
					humidity_monitoring: serverRoom.humidity_monitoring,
					access_control: serverRoom.access_control
				};
			} else {
				// Create mode - reset form
				formData = {
					name: '',
					description: '',
					floor_level: '',
					room_number: '',
					temperature_monitoring: false,
					humidity_monitoring: false,
					access_control: false
				};
			}
		}
	});

	async function handleSubmit() {
		if (!formData.name.trim()) {
			alert('서버실명은 필수입니다.');
			return;
		}

		loading = true;
		try {
			if (isEdit && serverRoom) {
				const updateData: UpdateServerRoomRequest = {};
				if (formData.name !== serverRoom.name) updateData.name = formData.name;
				if (formData.description !== (serverRoom.description || ''))
					updateData.description = formData.description || undefined;
				if (formData.floor_level !== (serverRoom.floor_level || ''))
					updateData.floor_level = formData.floor_level || undefined;
				if (formData.room_number !== (serverRoom.room_number || ''))
					updateData.room_number = formData.room_number || undefined;
				if (formData.temperature_monitoring !== serverRoom.temperature_monitoring)
					updateData.temperature_monitoring = formData.temperature_monitoring;
				if (formData.humidity_monitoring !== serverRoom.humidity_monitoring)
					updateData.humidity_monitoring = formData.humidity_monitoring;
				if (formData.access_control !== serverRoom.access_control) updateData.access_control = formData.access_control;

				if (Object.keys(updateData).length > 0) {
					await serverRoomApi.updateServerRoom(officeId, serverRoom.id, updateData);
				}
			} else {
				const createData: CreateServerRoomRequest = {
					name: formData.name,
					description: formData.description || undefined,
					floor_level: formData.floor_level || undefined,
					room_number: formData.room_number || undefined,
					temperature_monitoring: formData.temperature_monitoring,
					humidity_monitoring: formData.humidity_monitoring,
					access_control: formData.access_control
				};
				await serverRoomApi.createServerRoom(officeId, createData);
			}

			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to save server room:', error);
			alert(`서버실 ${isEdit ? '수정' : '등록'}에 실패했습니다.`);
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
				서버실 정보를 {isEdit ? '수정' : '등록'}하세요
			</p>
		</DialogHeader>

		<form on:submit|preventDefault={handleSubmit} class="space-y-5 pt-4">
			<div>
				<label
					for="name"
					class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					서버실명 <span class="text-red-500">*</span>
				</label>
				<input
					id="name"
					type="text"
					bind:value={formData.name}
					required
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="서버실명을 입력하세요"
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
						bg-white text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500
						focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="서버실 설명을 입력하세요"
				></textarea>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div>
					<label
						for="floor_level"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						층수
					</label>
					<input
						id="floor_level"
						type="text"
						bind:value={formData.floor_level}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: B1, 1F, 2F"
					/>
				</div>

				<div>
					<label
						for="room_number"
						class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						호실
					</label>
					<input
						id="room_number"
						type="text"
						bind:value={formData.room_number}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 101, A동"
					/>
				</div>
			</div>

			<div>
				<label class="block {isDesktop ? 'text-xs' : 'text-sm'} mb-3 font-medium text-gray-900 dark:text-white">
					모니터링 기능
				</label>
				<div class="space-y-3">
					<label class="flex cursor-pointer items-center space-x-3">
						<input
							type="checkbox"
							bind:checked={formData.temperature_monitoring}
							class="h-4 w-4 rounded border-gray-300 bg-gray-100 text-red-600 focus:ring-2 focus:ring-red-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-red-600"
						/>
						<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-900 dark:text-white">온도 모니터링</span>
					</label>

					<label class="flex cursor-pointer items-center space-x-3">
						<input
							type="checkbox"
							bind:checked={formData.humidity_monitoring}
							class="h-4 w-4 rounded border-gray-300 bg-gray-100 text-blue-600 focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600"
						/>
						<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-900 dark:text-white">습도 모니터링</span>
					</label>

					<label class="flex cursor-pointer items-center space-x-3">
						<input
							type="checkbox"
							bind:checked={formData.access_control}
							class="h-4 w-4 rounded border-gray-300 bg-gray-100 text-green-600 focus:ring-2 focus:ring-green-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-green-600"
						/>
						<span class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-900 dark:text-white">출입 통제</span>
					</label>
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
					disabled={loading || !formData.name.trim()}
					class="bg-blue-500 text-white hover:bg-blue-600 {isDesktop
						? 'px-4 py-2 text-xs'
						: 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? '처리 중...' : isEdit ? '수정' : '등록'}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>
