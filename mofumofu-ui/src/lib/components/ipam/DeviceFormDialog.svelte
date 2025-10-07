<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { HardDrive } from '@lucide/svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { deviceApi, rackApi, type Device, type CreateDeviceRequest, type Rack } from '$lib/api/office';
	import Select from 'svelte-select';

	interface DeviceFormData {
		rack_id: string;
		name: string;
		device_type: string;
		description: string;
		manufacturer: string;
		model: string;
		serial_number: string;
		rack_position: string;
		rack_size: string;
		power_consumption: string;
		status: string;
		purchase_date: string;
		warranty_end: string;
	}

	interface Props {
		open: boolean;
		onClose: () => void;
		onSuccess: (newDevice?: Device) => void;
		editData?: Device | null;
	}

	let { open, onClose, onSuccess, editData = null }: Props = $props();

	let loading = $state(false);
	let racks = $state<Rack[]>([]);
	let selectedRack = $state<{value: string, label: string} | null>(null);
	let selectedDeviceType = $state<{value: string, label: string} | null>(null);
	let selectedStatus = $state<{value: string, label: string} | null>(null);

	let formData = $state<DeviceFormData>({
		rack_id: '',
		name: '',
		device_type: '',
		description: '',
		manufacturer: '',
		model: '',
		serial_number: '',
		rack_position: '',
		rack_size: '1',
		power_consumption: '',
		status: 'active',
		purchase_date: '',
		warranty_end: ''
	});

	const isDesktop = $derived(desktopStore.isDesktop);

	const deviceTypes = [
		{ value: 'server', label: '서버' },
		{ value: 'switch', label: '스위치' },
		{ value: 'router', label: '라우터' },
		{ value: 'firewall', label: '방화벽' },
		{ value: 'storage', label: '스토리지' }
	];

	const statusOptions = [
		{ value: 'active', label: '활성' },
		{ value: 'inactive', label: '비활성' },
		{ value: 'maintenance', label: '점검' }
	];

	// Load racks when dialog opens
	$effect(() => {
		if (open) {
			loadRacks();
		}
	});

	// Reset form when dialog opens/closes or edit data changes
	$effect(() => {
		if (open) {
			if (editData) {
				// Populate form with edit data
				formData = {
					rack_id: editData.rack_id || '',
					name: editData.name,
					device_type: editData.device_type,
					description: editData.description || '',
					manufacturer: editData.manufacturer || '',
					model: editData.model || '',
					serial_number: editData.serial_number || '',
					rack_position: editData.rack_position?.toString() || '',
					rack_size: editData.rack_size?.toString() || '1',
					power_consumption: editData.power_consumption?.toString() || '',
					status: editData.status || 'active',
					purchase_date: editData.purchase_date || '',
					warranty_end: editData.warranty_end || ''
				};

				// Set selected values for svelte-select
				selectedRack = editData.rack_id ? racks.find(r => r.id === editData.rack_id)
					? { value: editData.rack_id, label: racks.find(r => r.id === editData.rack_id)!.name }
					: null : null;
				selectedDeviceType = deviceTypes.find(t => t.value === editData.device_type) || null;
				selectedStatus = statusOptions.find(s => s.value === editData.status) || null;
			} else {
				// Reset to empty form
				formData = {
					rack_id: '',
					name: '',
					device_type: '',
					description: '',
					manufacturer: '',
					model: '',
					serial_number: '',
					rack_position: '',
					rack_size: '1',
					power_consumption: '',
					status: 'active',
					purchase_date: '',
					warranty_end: ''
				};
				selectedRack = null;
				selectedDeviceType = null;
				selectedStatus = statusOptions.find(s => s.value === 'active') || null;
			}
		}
	});

	async function loadRacks() {
		try {
			const response = await rackApi.getRacks({ page: 1, limit: 100 });
			racks = response.racks;
		} catch (error) {
			console.error('Failed to load racks:', error);
		}
	}

	// Update formData when selections change
	$effect(() => {
		if (selectedRack) {
			formData.rack_id = selectedRack.value;
		}
	});

	$effect(() => {
		if (selectedDeviceType) {
			formData.device_type = selectedDeviceType.value;
		}
	});

	$effect(() => {
		if (selectedStatus) {
			formData.status = selectedStatus.value;
		}
	});

	async function handleSubmit() {
		if (!formData.name.trim() || !formData.device_type) {
			alert('디바이스명과 타입은 필수입니다.');
			return;
		}

		loading = true;
		try {
			const requestData: CreateDeviceRequest = {
				rack_id: formData.rack_id || undefined,
				name: formData.name,
				device_type: formData.device_type,
				description: formData.description || undefined,
				manufacturer: formData.manufacturer || undefined,
				model: formData.model || undefined,
				serial_number: formData.serial_number || undefined,
				rack_position: formData.rack_position ? parseInt(formData.rack_position) : undefined,
				rack_size: formData.rack_size ? parseInt(formData.rack_size) : 1,
				power_consumption: formData.power_consumption ? parseInt(formData.power_consumption) : undefined,
				status: formData.status || 'active',
				purchase_date: formData.purchase_date || undefined,
				warranty_end: formData.warranty_end || undefined
			};

			let result: Device;
			if (editData) {
				// Update existing device
				result = await deviceApi.updateDevice(editData.id, requestData);
			} else {
				// Create new device
				result = await deviceApi.createDevice(requestData);
			}

			onSuccess(result);
			onClose();
		} catch (error) {
			console.error('Failed to save device:', error);
			alert(editData ? '디바이스 수정에 실패했습니다.' : '디바이스 등록에 실패했습니다.');
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="max-h-[90vh] w-full max-w-3xl overflow-y-auto border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader class="border-b border-gray-200 pb-4 dark:border-gray-700">
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
					<HardDrive class="h-5 w-5 text-blue-600 dark:text-blue-400" />
				</div>
				<DialogTitle class="{isDesktop ? 'text-lg' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
					{editData ? '디바이스 수정' : '새 디바이스 추가'}
				</DialogTitle>
			</div>
		</DialogHeader>

		<form on:submit|preventDefault={handleSubmit} class="space-y-5 pt-4">
			<!-- 랙 선택 -->
			<div>
				<label
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					랙
				</label>
				<Select
					items={racks.map(r => ({ value: r.id, label: r.name }))}
					bind:value={selectedRack}
					placeholder="랙을 선택하세요"
					clearable={true}
					searchable={true}
					--border-radius="0.5rem"
					--border="1px solid rgb(209 213 219)"
					--border-focused="1px solid rgb(59 130 246)"
					--padding="0.625rem 0.75rem"
					--font-size={isDesktop ? '0.875rem' : '1rem'}
					--background="white"
					--list-background="white"
					--list-max-height="200px"
					--item-hover-bg="rgb(243 244 246)"
					--item-hover-color="rgb(17 24 39)"
					--item-is-active-bg="rgb(219 234 254)"
					--item-is-active-color="rgb(17 24 39)"
					--item-color="rgb(17 24 39)"
					--input-color="rgb(17 24 39)"
					--placeholder-color="rgb(156 163 175)"
					--selected-item-color="rgb(17 24 39)"
					--multi-item-color="rgb(17 24 39)"
				/>
			</div>

			<!-- 디바이스명 -->
			<div>
				<label
					for="name"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					디바이스명 <span class="text-red-500">*</span>
				</label>
				<input
					id="name"
					type="text"
					bind:value={formData.name}
					required
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="예: WEB-SERVER-01"
				/>
			</div>

			<!-- 디바이스 타입과 상태 -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						디바이스 타입 <span class="text-red-500">*</span>
					</label>
					<Select
						items={deviceTypes}
						bind:value={selectedDeviceType}
						placeholder="선택하세요"
						clearable={false}
						searchable={false}
						--border-radius="0.5rem"
						--border="1px solid rgb(209 213 219)"
						--border-focused="1px solid rgb(59 130 246)"
						--padding="0.625rem 0.75rem"
						--font-size={isDesktop ? '0.875rem' : '1rem'}
						--background="white"
						--list-background="white"
						--list-max-height="200px"
						--item-hover-bg="rgb(243 244 246)"
						--item-hover-color="rgb(17 24 39)"
						--item-is-active-bg="rgb(219 234 254)"
						--item-is-active-color="rgb(17 24 39)"
						--item-color="rgb(17 24 39)"
						--input-color="rgb(17 24 39)"
						--placeholder-color="rgb(156 163 175)"
						--selected-item-color="rgb(17 24 39)"
						--multi-item-color="rgb(17 24 39)"
					/>
				</div>

				<div>
					<label
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						상태
					</label>
					<Select
						items={statusOptions}
						bind:value={selectedStatus}
						placeholder="상태 선택"
						clearable={false}
						searchable={false}
						--border-radius="0.5rem"
						--border="1px solid rgb(209 213 219)"
						--border-focused="1px solid rgb(59 130 246)"
						--padding="0.625rem 0.75rem"
						--font-size={isDesktop ? '0.875rem' : '1rem'}
						--background="white"
						--list-background="white"
						--list-max-height="200px"
						--item-hover-bg="rgb(243 244 246)"
						--item-hover-color="rgb(17 24 39)"
						--item-is-active-bg="rgb(219 234 254)"
						--item-is-active-color="rgb(17 24 39)"
						--item-color="rgb(17 24 39)"
						--input-color="rgb(17 24 39)"
						--placeholder-color="rgb(156 163 175)"
						--selected-item-color="rgb(17 24 39)"
						--multi-item-color="rgb(17 24 39)"
					/>
				</div>
			</div>

			<!-- 제조사와 모델 -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label
						for="manufacturer"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						제조사
					</label>
					<input
						id="manufacturer"
						type="text"
						bind:value={formData.manufacturer}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: Dell, HP, Cisco"
					/>
				</div>

				<div>
					<label
						for="model"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						모델
					</label>
					<input
						id="model"
						type="text"
						bind:value={formData.model}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: PowerEdge R740"
					/>
				</div>
			</div>

			<!-- 시리얼 번호 -->
			<div>
				<label
					for="serial_number"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					시리얼 번호
				</label>
				<input
					id="serial_number"
					type="text"
					bind:value={formData.serial_number}
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="예: ABC123XYZ"
				/>
			</div>

			<!-- 랙 위치와 랙 크기 -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
				<div>
					<label
						for="rack_position"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						랙 위치 (U)
					</label>
					<input
						id="rack_position"
						type="number"
						bind:value={formData.rack_position}
						min="1"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="1"
					/>
				</div>

				<div>
					<label
						for="rack_size"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						랙 크기 (U)
					</label>
					<input
						id="rack_size"
						type="number"
						bind:value={formData.rack_size}
						min="1"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="1"
					/>
				</div>

				<div>
					<label
						for="power_consumption"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						소비 전력 (W)
					</label>
					<input
						id="power_consumption"
						type="number"
						bind:value={formData.power_consumption}
						min="0"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="500"
					/>
				</div>
			</div>

			<!-- 구매일과 보증 종료일 -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label
						for="purchase_date"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						구매일
					</label>
					<input
						id="purchase_date"
						type="date"
						bind:value={formData.purchase_date}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					/>
				</div>

				<div>
					<label
						for="warranty_end"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						보증 종료일
					</label>
					<input
						id="warranty_end"
						type="date"
						bind:value={formData.warranty_end}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					/>
				</div>
			</div>

			<!-- 설명 -->
			<div>
				<label
					for="description"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					설명
				</label>
				<textarea
					id="description"
					bind:value={formData.description}
					rows="3"
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} resize-none rounded-lg border border-gray-300
						bg-white text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500
						focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="디바이스에 대한 설명을 입력하세요"
				></textarea>
			</div>

			<!-- Buttons -->
			<div class="flex justify-end space-x-3 border-t border-gray-200 pt-6 dark:border-gray-700">
				<Button
					type="button"
					variant="outline"
					onclick={onClose}
					disabled={loading}
					class="{isDesktop ? 'px-4 py-2 text-sm' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
				>
					취소
				</Button>
				<Button
					type="submit"
					disabled={loading || !formData.name.trim() || !formData.device_type}
					class="bg-blue-600 text-white hover:bg-blue-700 {isDesktop
						? 'px-4 py-2 text-sm'
						: 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? (editData ? '수정 중...' : '추가 중...') : editData ? '수정' : '추가'}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>
