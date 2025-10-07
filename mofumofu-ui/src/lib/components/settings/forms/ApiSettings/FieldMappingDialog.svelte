<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.ts';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog/index.ts';
	import * as Select from '$lib/components/ui/select/index.ts';
	import { Input } from '$lib/components/ui/input/index.ts';
	import { Badge } from '$lib/components/ui/badge/index.ts';
	import { Separator } from '$lib/components/ui/separator/index.ts';
	import { Icon, InformationCircle, Plus, Trash, ArrowRight } from 'svelte-hero-icons';
	import type { FieldMapping } from '$lib/api/external-api.ts';

	interface Props {
		open: boolean;
		sampleData?: any;
		existingMapping?: FieldMapping;
		onClose: () => void;
		onSave: (mapping: FieldMapping) => void;
	}

	let { open = $bindable(), sampleData, existingMapping, onClose, onSave }: Props = $props();

	// 샘플 데이터에서 필드 추출
	let sampleFields = $derived(() => {
		if (!sampleData) {
			// 샘플 데이터가 없으면 기본 필드들 제공
			return [
				{ field: 'id', type: 'string', sample: '1' },
				{ field: 'name', type: 'string', sample: 'Shawn' },
				{ field: 'email', type: 'string', sample: 'shawn@daangn.com' },
				{ field: 'phone', type: 'string', sample: '010-2000-0001' },
				{ field: 'avatar', type: 'string', sample: 'https://cdn...' },
				{ field: 'createdAt', type: 'date', sample: '2025-07-25T18:25:24.724Z' }
			];
		}
		const fields = extractFields(sampleData);
		console.log('Sample data:', sampleData);
		console.log('Extracted fields from sample data:', fields);
		return fields;
	});

	// 대상 필드 옵션 (일반적인 CRM/비즈니스 필드들)
	const targetFieldOptions = [
		{ value: 'email', label: '이메일 (email)', type: 'string' },
		{ value: 'name', label: '이름 (name)', type: 'string' },
		{ value: 'phone', label: '전화번호 (phone)', type: 'string' },
		{ value: 'mobile', label: '휴대폰 (mobile)', type: 'string' },
		{ value: 'title', label: '직책 (title)', type: 'string' },
		{ value: 'department', label: '부서 (department)', type: 'string' },
		{ value: 'office_location', label: '근무지 (office_location)', type: 'string' },
		{ value: 'responsibilities', label: '담당업무 (responsibilities)', type: 'string' },
		{ value: 'company', label: '회사 (company)', type: 'string' },
		{ value: 'address', label: '주소 (address)', type: 'string' },
		{ value: 'created_at', label: '생성일 (created_at)', type: 'date' },
		{ value: 'updated_at', label: '수정일 (updated_at)', type: 'date' },
		{ value: 'id', label: 'ID', type: 'string' },
		{ value: 'status', label: '상태 (status)', type: 'string' }
	];

	// 데이터 타입 옵션
	const dataTypeOptions = [
		{ value: 'string', label: '문자열' },
		{ value: 'number', label: '숫자' },
		{ value: 'boolean', label: '불린 (true/false)' },
		{ value: 'date', label: '날짜' }
	];

	// 변환 옵션
	const transformationOptions = [
		{ value: '', label: '변환 없음' },
		{ value: 'uppercase', label: '대문자로 변환' },
		{ value: 'lowercase', label: '소문자로 변환' },
		{ value: 'trim', label: '공백 제거' },
		{ value: 'title', label: '첫글자 대문자' }
	];

	// 필터 연산자 옵션
	const operatorOptions = [
		{ value: 'equals', label: '같음 (=)' },
		{ value: 'not_equals', label: '같지 않음 (≠)' },
		{ value: 'contains', label: '포함' }
	];

	// 현재 매핑 설정
	let mappings = $state<FieldMapping['mappings']>([]);
	let filterConditions = $state<FieldMapping['filter_conditions']>([]);

	// 기존 매핑이 있으면 로드
	$effect(() => {
		if (existingMapping) {
			mappings = [...(existingMapping.mappings || [])];
			filterConditions = [...(existingMapping.filter_conditions || [])];
		} else {
			// 자동 매핑 제안
			mappings = generateAutoMappingSuggestions();
			filterConditions = [];
		}
	});

	// 샘플 데이터에서 필드 추출 (중첩 지원)
	function extractFields(data: any, prefix = ''): Array<{ field: string; type: string; sample: any }> {
		const fields: Array<{ field: string; type: string; sample: any }> = [];

		if (typeof data === 'object' && data !== null && !Array.isArray(data)) {
			for (const [key, value] of Object.entries(data)) {
				const fieldPath = prefix ? `${prefix}.${key}` : key;
				const type = inferDataType(value);

				// 중첩 객체는 건너뛰고 단순 값만 추가
				if (typeof value !== 'object' || value === null) {
					fields.push({ field: fieldPath, type, sample: value });
				} else if (typeof value === 'object' && !Array.isArray(value) && prefix.split('.').length < 2) {
					// 중첩 객체 처리 (depth 제한)
					fields.push(...extractFields(value, fieldPath));
				}
			}
		}

		return fields;
	}

	// 데이터 타입 추론
	function inferDataType(value: any): string {
		if (typeof value === 'string') {
			// 날짜 형식 확인
			if (/^\d{4}-\d{2}-\d{2}/.test(value)) return 'date';
			// 숫자 문자열 확인
			if (/^\d+(\.\d+)?$/.test(value)) return 'number';
			return 'string';
		}
		if (typeof value === 'number') return 'number';
		if (typeof value === 'boolean') return 'boolean';
		return 'string';
	}

	// 자동 매핑 제안 생성
	function generateAutoMappingSuggestions(): FieldMapping['mappings'] {
		if (!sampleFields.length) return [];

		const suggestions: FieldMapping['mappings'] = [];

		// 필드명 기반 자동 매핑
		const fieldMatches = [
			{ source: ['email', 'mail', 'e_mail'], target: 'email' },
			{ source: ['name', 'username', 'full_name', 'fullname'], target: 'name' },
			{ source: ['phone', 'telephone', 'tel'], target: 'phone' },
			{ source: ['mobile', 'cellphone', 'cell'], target: 'mobile' },
			{ source: ['title', 'position', 'job_title'], target: 'title' },
			{ source: ['department', 'dept', 'division'], target: 'department' },
			{ source: ['office', 'location', 'office_location'], target: 'office_location' },
			{ source: ['id', '_id', 'uuid'], target: 'id' },
			{ source: ['created', 'createdAt', 'created_at'], target: 'created_at' }
		];

		for (const match of fieldMatches) {
			const sourceField = sampleFields.find((f) => match.source.some((s) => f.field.toLowerCase().includes(s)));

			if (sourceField) {
				const targetField = targetFieldOptions.find((t) => t.value === match.target);
				if (targetField) {
					suggestions.push({
						source_field: sourceField.field,
						target_field: match.target,
						data_type: sourceField.type as any,
						default_value: '',
						transformation: ''
					});
				}
			}
		}

		return suggestions;
	}

	// 매핑 추가 - 사용하지 않음 (인라인으로 처리)
	function addMapping() {
		// 사용하지 않음
	}

	// 매핑 제거 - 사용하지 않음 (인라인으로 처리)
	function removeMapping(index: number) {
		// 사용하지 않음
	}

	// 필터 조건 추가
	function addFilterCondition() {
		filterConditions.push({
			field: '',
			operator: 'equals',
			value: ''
		});
	}

	// 필터 조건 제거
	function removeFilterCondition(index: number) {
		filterConditions.splice(index, 1);
	}

	// 저장
	function handleSave() {
		const mapping: FieldMapping = {
			mappings: mappings.filter((m) => m.source_field && m.target_field),
			filter_conditions: filterConditions.filter((f) => f.field && f.value)
		};
		onSave(mapping);
		onClose();
	}

	// 닫기
	function handleClose() {
		onClose();
	}
</script>

<Dialog bind:open>
	<DialogContent class="flex max-h-[85vh] w-[95vw] max-w-5xl flex-col overflow-hidden">
		<DialogHeader class="border-b pb-4">
			<DialogTitle class="text-xl font-semibold">필드 매핑 설정</DialogTitle>
		</DialogHeader>

		<div class="flex-1 overflow-y-auto px-1">
			<div class="space-y-6 py-4">
				<!-- 안내 정보 -->
				<div class="rounded-lg border border-blue-200 bg-blue-50 p-3 dark:border-blue-800 dark:bg-blue-900/20">
					<div class="flex items-start gap-2">
						<Icon src={InformationCircle} size="18" class="mt-0.5 flex-shrink-0 text-blue-600 dark:text-blue-400" />
						<div>
							<p class="text-sm font-medium text-blue-900 dark:text-blue-300">필드 매핑 안내</p>
							<p class="mt-1 text-xs text-blue-800 dark:text-blue-400">
								API 응답의 필드를 데이터베이스 필드와 매핑해주세요. 이 매핑을 통해 외부 데이터를 시스템에 맞게
								변환합니다.
							</p>
						</div>
					</div>
				</div>

				<!-- 샘플 데이터 표시 -->
				<div class="space-y-3">
					<h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">샘플 데이터</h3>
					{#if sampleData}
						<div class="rounded-lg border bg-gray-50 p-4 dark:bg-gray-900">
							<pre class="overflow-x-auto font-mono text-xs">{JSON.stringify(sampleData, null, 2)}</pre>
						</div>
					{:else}
						<div class="rounded-lg border bg-gray-50 p-4 text-center text-gray-500 dark:bg-gray-900">
							<p class="text-sm">샘플 데이터가 없습니다</p>
						</div>
					{/if}
				</div>

				<!-- 필드 매핑 설정 -->
				<div class="space-y-4">
					<!-- 필드 매핑 리스트 -->
					<div class="space-y-4">
						{#each targetFieldOptions as targetField}
							{@const currentMapping = mappings.find((m) => m.target_field === targetField.value) || {
								source_field: '',
								target_field: targetField.value,
								data_type: targetField.type || 'string',
								transformation: '',
								default_value: ''
							}}
							<div class="flex items-center gap-4">
								<!-- 대상 필드 레이블 -->
								<div class="w-40 text-sm text-gray-700 dark:text-gray-300">
									{targetField.label}
									{#if targetField.value === 'name' || targetField.value === 'email'}
										<span class="text-red-500">*</span>
									{/if}
								</div>

								<!-- 화살표 -->
								<Icon src={ArrowRight} size="16" class="flex-shrink-0 text-gray-400" />

								<!-- 소스 필드 선택 -->
								<div class="flex-1">
									<Select.Root
										value={currentMapping.source_field}
										onSelectedChange={(value) => {
											const existingIndex = mappings.findIndex((m) => m.target_field === targetField.value);
											if (value?.value) {
												const newMapping = {
													...currentMapping,
													source_field: value.value,
													target_field: targetField.value,
													data_type: targetField.type || 'string',
													transformation: '',
													default_value: ''
												};
												if (existingIndex >= 0) {
													mappings[existingIndex] = newMapping;
												} else {
													mappings.push(newMapping);
												}
											} else if (existingIndex >= 0) {
												mappings.splice(existingIndex, 1);
											}
										}}
									>
										<Select.Trigger class="h-10 w-full rounded-lg border bg-white px-3 text-left dark:bg-gray-800">
											{#if currentMapping.source_field}
												<span class="text-sm">
													{currentMapping.source_field}
													<span class="ml-1 text-gray-500">
														(예: {sampleFields.find((f) => f.field === currentMapping.source_field)?.sample || ''})
													</span>
												</span>
											{:else}
												<span class="text-gray-500">선택하지 않음</span>
											{/if}
										</Select.Trigger>
										<Select.Content>
											<Select.Item value="">
												<span class="text-gray-500">선택하지 않음</span>
											</Select.Item>
											{#each sampleFields as field}
												<Select.Item value={field.field}>
													<div class="flex w-full items-center justify-between">
														<span class="font-medium">{field.field}</span>
														<span class="ml-2 text-xs text-gray-500"
															>(예: {String(field.sample).substring(0, 20)}{String(field.sample).length > 20
																? '...'
																: ''})</span
														>
													</div>
												</Select.Item>
											{/each}
										</Select.Content>
									</Select.Root>
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
		</div>

		<!-- 버튼 -->
		<div class="flex justify-end gap-3 border-t bg-gray-50 px-6 pt-4 dark:bg-gray-900">
			<Button type="button" variant="outline" size="sm" onclick={handleClose}>취소</Button>
			<Button type="button" size="sm" onclick={handleSave}>저장</Button>
		</div>
	</DialogContent>
</Dialog>
