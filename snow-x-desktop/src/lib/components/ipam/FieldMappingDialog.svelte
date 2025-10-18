<script lang="ts">
	import { X, ArrowRight } from 'lucide-svelte';
	import type { ConnectionTargetType, FieldMapping } from '$lib/api/external-api';

	interface Props {
		show: boolean;
		targetType: ConnectionTargetType;
		apiUrl: string;
		headers?: Record<string, string>;
		currentMapping?: FieldMapping;
		onClose: () => void;
		onSave: (mapping: FieldMapping) => void;
	}

	let { show = $bindable(), targetType, apiUrl, headers, currentMapping, onClose, onSave }: Props = $props();

	// Debug log
	$effect(() => {
		if (show) {
			console.log('[FieldMappingDialog] Opened with targetType:', targetType);
			console.log('[FieldMappingDialog] API URL:', apiUrl);
		}
	});

	// Target-specific field definitions
	const fieldDefinitions: Record<ConnectionTargetType, { key: string; label: string; required?: boolean }[]> = {
		contact: [
			{ key: 'email', label: '이메일 (email)', required: true },
			{ key: 'name', label: '이름 (name)', required: true },
			{ key: 'phone', label: '전화번호 (phone)' },
			{ key: 'mobile', label: '휴대폰 (mobile)' },
			{ key: 'title', label: '직책 (title)' },
			{ key: 'department', label: '부서 (department)' },
			{ key: 'office_location', label: '근무지 (office_location)' },
			{ key: 'responsibilities', label: '담당업무 (responsibilities)' }
		],
		device: [
			{ key: 'name', label: '이름 (name)', required: true },
			{ key: 'device_type', label: '디바이스 타입 (device_type)', required: true },
			{ key: 'manufacturer', label: '제조사 (manufacturer)' },
			{ key: 'model', label: '모델 (model)' },
			{ key: 'serial_number', label: '시리얼 번호 (serial_number)' },
			{ key: 'description', label: '설명 (description)' },
			{ key: 'rack_size', label: '랙 크기 (rack_size)' },
			{ key: 'power_consumption', label: '전력 소비 (power_consumption)' },
			{ key: 'status', label: '상태 (status)' }
		],
		device_library: [
			{ key: 'name', label: '이름 (name)', required: true },
			{ key: 'device_type', label: '디바이스 타입 (device_type)', required: true },
			{ key: 'manufacturer', label: '제조사 (manufacturer)' },
			{ key: 'model', label: '모델 (model)' },
			{ key: 'description', label: '설명 (description)' },
			{ key: 'default_rack_size', label: '기본 랙 크기 (default_rack_size)' },
			{ key: 'default_power_consumption', label: '기본 전력 소비 (default_power_consumption)' }
		]
	};

	let sampleData = $state<any>(null);
	let availableFields = $state<string[]>([]);
	let isLoadingSample = $state(false);

	let mappings = $state<Record<string, string>>({});

	// Initialize mappings from current mapping
	$effect(() => {
		if (currentMapping?.mappings) {
			const newMappings: Record<string, string> = {};
			currentMapping.mappings.forEach(m => {
				newMappings[m.target_field] = m.source_field;
			});
			mappings = newMappings;
		}
	});

	async function loadSampleData() {
		isLoadingSample = true;
		try {
			const response = await fetch(apiUrl, {
				headers: headers || {}
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}`);
			}

			const data = await response.json();

			// Handle array or single object
			if (Array.isArray(data)) {
				sampleData = data[0] || {};
			} else {
				sampleData = data;
			}

			// Extract available fields from sample data
			availableFields = Object.keys(sampleData);
		} catch (error) {
			console.error('Failed to load sample data:', error);
			sampleData = { error: '샘플 데이터를 불러올 수 없습니다.' };
			availableFields = [];
		} finally {
			isLoadingSample = false;
		}
	}

	function handleSave() {
		const fieldMappingArray = Object.entries(mappings)
			.filter(([_, sourceField]) => sourceField && sourceField !== '')
			.map(([targetField, sourceField]) => ({
				source_field: sourceField,
				target_field: targetField,
				data_type: 'string' as const,
				default_value: undefined,
				transformation: undefined
			}));

		const mapping: FieldMapping = {
			mappings: fieldMappingArray,
			filter_conditions: []
		};

		onSave(mapping);
		show = false;
	}

	// Load sample data when dialog opens
	$effect(() => {
		if (show && apiUrl) {
			loadSampleData();
		}
	});
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={(e) => {
			if (e.target === e.currentTarget) onClose();
		}}
	>
		<div class="w-full max-w-4xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
					{targetType === 'contact' ? '담당자' : targetType === 'device' ? '디바이스' : '라이브러리'} 필드 매핑 설정
				</h2>
				<button
					onclick={onClose}
					class="rounded-lg p-1 hover:bg-gray-100 dark:hover:bg-gray-700"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<div class="mb-4 rounded-lg bg-blue-50 p-4 dark:bg-blue-900/20">
				<div class="flex items-start gap-2">
					<div class="mt-0.5 rounded-full bg-blue-100 p-1 dark:bg-blue-900">
						<svg class="h-4 w-4 text-blue-600 dark:text-blue-400" fill="currentColor" viewBox="0 0 20 20">
							<path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
						</svg>
					</div>
					<div class="flex-1">
						<h3 class="font-medium text-blue-900 dark:text-blue-100">필드 매핑 안내</h3>
						<p class="mt-1 text-sm text-blue-700 dark:text-blue-300">
							API 응답의 필드를 데이터베이스 필드에 매핑해주세요. 이메일은 필수 필드입니다.
						</p>
					</div>
				</div>
			</div>

			<div class="mb-6 grid gap-3 rounded-lg border border-gray-200 p-4 dark:border-gray-700">
				{#if fieldDefinitions[targetType] && fieldDefinitions[targetType].length > 0}
					{#each fieldDefinitions[targetType] as field}
						<div class="flex items-center gap-3">
							<div class="w-48 text-sm font-medium text-gray-700 dark:text-gray-300">
								{field.label}
								{#if field.required}
									<span class="text-red-500">*</span>
								{/if}
							</div>
							<ArrowRight class="h-4 w-4 text-gray-400" />
							<select
								bind:value={mappings[field.key]}
								class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
							>
								<option value="">선택하지 않음</option>
								{#each availableFields as apiField}
									<option value={apiField}>{apiField} (예: {String(sampleData?.[apiField] || '').substring(0, 30)}...)</option>
								{/each}
							</select>
						</div>
					{/each}
				{:else}
					<div class="py-8 text-center text-sm text-gray-500">
						<p>필드 정의를 불러오는 중 오류가 발생했습니다.</p>
						<p class="mt-2 text-xs">targetType: {targetType}</p>
					</div>
				{/if}
			</div>

			<!-- Sample Data Preview -->
			<div class="mb-6">
				<h3 class="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">샘플 데이터</h3>
				<div class="rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
					{#if isLoadingSample}
						<div class="flex items-center justify-center py-4">
							<div class="h-6 w-6 animate-spin rounded-full border-4 border-orange-600 border-t-transparent"></div>
						</div>
					{:else if sampleData}
						<pre class="overflow-x-auto text-xs text-gray-800 dark:text-gray-200">{JSON.stringify(sampleData, null, 2)}</pre>
					{:else}
						<p class="text-sm text-gray-500">샘플 데이터를 불러오는 중...</p>
					{/if}
				</div>
			</div>

			<div class="flex justify-end gap-2">
				<button
					onclick={onClose}
					class="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
				>
					취소
				</button>
				<button
					onclick={handleSave}
					class="rounded-lg bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
				>
					저장
				</button>
			</div>
		</div>
	</div>
{/if}
