<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Select from '$lib/components/ui/select';
	import { Icon, Link, Trash, Pencil, Cog6Tooth, Eye, EyeSlash, Plus, ArrowRight } from 'svelte-hero-icons';
	import { externalApiService, type FieldMapping, type CreateConnectionRequest } from '$lib/api/external-api.ts';
	import FieldMappingDialog from './FieldMappingDialog.svelte';

	// API 연결 상태
	let apiConnections = $state<any[]>([]);
	let isLoading = $state(true);

	let showAddForm = $state(false);
	let newConnection = $state({
		name: '',
		type: 'REST API',
		category: '라이브러리 동기화',
		url: '',
		description: '',
		headers: [{ key: '', value: '' }],
		autoSync: false,
		fieldMapping: {
			mappings: [{ 
				source_field: '', 
				target_field: '', 
				data_type: 'string' as const,
				default_value: '',
				transformation: undefined
			}],
			filter_conditions: []
		} as FieldMapping
	});

	let showFieldMapping = $state(false);
	let testConnectionResult = $state<any>(null);
	let isTestingConnection = $state(false);
	let showFieldMappingDialog = $state(false);
	let sampleDataForMapping = $state<any>(null);

	// API 연결 목록 로드
	async function loadApiConnections() {
		try {
			const connections = await externalApiService.listConnections();
			apiConnections = connections.map(conn => ({
				id: conn.id.toString(),
				name: conn.name,
				url: conn.base_url,
				lastSync: conn.last_sync_at ? new Date(conn.last_sync_at).toLocaleString('ko-KR') : '동기화 안됨',
				status: conn.is_active ? 'active' : 'inactive',
				syncEnabled: conn.auto_sync,
				registrationDate: new Date(conn.created_at).toLocaleString('ko-KR'),
				actions: conn.sync_count,
				hasError: !!conn.last_error_message,
				errorMessage: conn.last_error_message
			}));
		} catch (error) {
			console.error('API 연결 목록 로드 실패:', error);
		} finally {
			isLoading = false;
		}
	}

	// 컴포넌트 마운트 시 데이터 로드
	$effect(() => {
		loadApiConnections();
		
		// 테스트용 샘플 데이터 (임시)
		if (!sampleDataForMapping) {
			sampleDataForMapping = {
				"createdAt": "2025-07-25T18:25:24.724Z",
				"name": "Shawn",
				"avatar": "https://cdn.jsdelivr.net/gh/faker-js/assets-person-portrait/male/512/71.jpg",
				"phone": "010-2000-0001",
				"email": "shawn@daangn.com",
				"id": "1"
			};
		}
	});

	async function addApiConnection() {
		if (newConnection.name && newConnection.url) {
			try {
				const headers = newConnection.headers.reduce((acc, header) => {
					if (header.key && header.value) {
						acc[header.key] = header.value;
					}
					return acc;
				}, {} as Record<string, string>);

				const requestData: CreateConnectionRequest = {
					name: newConnection.name,
					base_url: newConnection.url,
					description: newConnection.description,
					headers: Object.keys(headers).length > 0 ? headers : undefined,
					field_mapping: showFieldMapping ? newConnection.fieldMapping : undefined,
					sync_interval: 3600,
					is_active: true,
					auto_sync: newConnection.autoSync
				};

				const connection = await externalApiService.createConnection(requestData);
				
				// 성공적으로 추가되면 목록을 다시 로드
				await loadApiConnections();
				
				// 폼 리셋
				resetForm();
				showAddForm = false;
				
			} catch (error) {
				console.error('API 연결 추가 실패:', error);
				alert('API 연결 추가에 실패했습니다.');
			}
		}
	}

	function resetForm() {
		newConnection = {
			name: '',
			type: 'REST API',
			category: '라이브러리 동기화',
			url: '',
			description: '',
			headers: [{ key: '', value: '' }],
			autoSync: false,
			fieldMapping: {
				mappings: [{ 
					source_field: '', 
					target_field: '', 
					data_type: 'string' as const,
					default_value: '',
					transformation: undefined
				}],
				filter_conditions: []
			}
		};
		showFieldMapping = false;
		testConnectionResult = null;
	}

	async function removeConnection(id: string) {
		try {
			await externalApiService.deleteConnection(parseInt(id));
			await loadApiConnections(); // 목록 새로고침
		} catch (error) {
			console.error('API 연결 삭제 실패:', error);
			alert('API 연결 삭제에 실패했습니다.');
		}
	}

	async function toggleSync(id: string) {
		try {
			const connection = apiConnections.find(conn => conn.id === id);
			if (!connection) return;

			await externalApiService.updateConnection(parseInt(id), {
				auto_sync: !connection.syncEnabled
			});
			await loadApiConnections(); // 목록 새로고침
		} catch (error) {
			console.error('동기화 설정 변경 실패:', error);
			alert('동기화 설정 변경에 실패했습니다.');
		}
	}


	function addHeader() {
		newConnection.headers = [...newConnection.headers, { key: '', value: '' }];
	}

	function removeHeader(index: number) {
		newConnection.headers = newConnection.headers.filter((_, i) => i !== index);
	}

	async function testApiConnection() {
		if (!newConnection.url) {
			alert('API URL을 입력해주세요.');
			return;
		}

		isTestingConnection = true;
		testConnectionResult = null;

		try {
			const headers = newConnection.headers.reduce((acc, header) => {
				if (header.key && header.value) {
					acc[header.key] = header.value;
				}
				return acc;
			}, {} as Record<string, string>);

			const result = await externalApiService.testConnection({
				base_url: newConnection.url,
				headers: Object.keys(headers).length > 0 ? headers : undefined
			});

			testConnectionResult = result;
			
			// 성공한 경우 샘플 데이터를 저장하여 필드 매핑에 활용
			if (result.status === 'success' && result.sample_data) {
				sampleDataForMapping = result.sample_data;
			}

		} catch (error) {
			console.error('API 연결 테스트 실패:', error);
			testConnectionResult = {
				status: 'error',
				message: error instanceof Error ? error.message : 'API 연결 테스트에 실패했습니다.'
			};
		} finally {
			isTestingConnection = false;
		}
	}

	// 필드 매핑 다이얼로그 열기
	async function openFieldMappingDialog(connection?: any) {
		if (connection) {
			// 기존 연결 편집 모드
			editingConnection = { ...connection };
			editingConnectionId = connection.id;

			// 항상 기본 샘플 데이터 사용 (API 호출하지 않고)
			sampleDataForMapping = {
				id: "1",
				name: "Shawn",
				email: "shawn@daangn.com",
				phone: "010-2000-0001",
				avatar: "https://cdn.jsdelivr.net/gh/faker-js/assets-person-portrait/male/512/771.jpg",
				createdAt: "2025-07-25T18:25:24.724Z"
			};

			console.log('Sample data for mapping:', sampleDataForMapping);
		}
		showFieldMappingDialog = true;
	}

	// 필드 매핑 다이얼로그 닫기
	function closeFieldMappingDialog() {
		showFieldMappingDialog = false;
	}

	// 필드 매핑 저장
	function saveFieldMapping(mapping: FieldMapping) {
		newConnection.fieldMapping = mapping;
		showFieldMapping = true; // 필드 매핑이 설정되었음을 표시
	}

	function addFieldMapping() {
		newConnection.fieldMapping.mappings = [...newConnection.fieldMapping.mappings, {
			source_field: '',
			target_field: '',
			data_type: 'string' as const,
			default_value: '',
			transformation: undefined
		}];
	}

	function removeFieldMapping(index: number) {
		newConnection.fieldMapping.mappings = newConnection.fieldMapping.mappings.filter((_, i) => i !== index);
	}

	function addFilterCondition() {
		newConnection.fieldMapping.filter_conditions = [...newConnection.fieldMapping.filter_conditions, {
			field: '',
			operator: 'equals' as const,
			value: ''
		}];
	}

	function removeFilterCondition(index: number) {
		newConnection.fieldMapping.filter_conditions = newConnection.fieldMapping.filter_conditions.filter((_, i) => i !== index);
	}

	// 연결 편집 모드
	let editingConnectionId = $state<string | null>(null);
	let editingConnection = $state<any>(null);

	// 연결 편집 시작
	function startEditConnection(connection: any) {
		editingConnectionId = connection.id;
		editingConnection = { ...connection };
		console.log('연결 편집 시작:', connection);
	}

	// 연결 상세 보기
	function viewConnection(connection: any) {
		console.log('연결 상세 보기:', connection);
		// TODO: 상세 보기 모달이나 페이지 구현
		alert(`${connection.name} 연결의 상세 정보를 봅니다.\nURL: ${connection.url}\n상태: ${connection.status}\n마지막 동기화: ${connection.lastSync}`);
	}

	// 연결 새로고침 (수동 동기화)
	async function refreshConnection(id: string) {
		try {
			console.log('연결 새로고침 시작:', id);
			const result = await externalApiService.syncConnection(parseInt(id));
			console.log('동기화 결과:', result);
			
			// 성공 메시지 표시
			alert('동기화가 시작되었습니다. 잠시 후 목록이 업데이트됩니다.');
			
			// 목록 새로고침
			setTimeout(() => {
				loadApiConnections();
			}, 2000); // 2초 후 목록 새로고침
			
		} catch (error) {
			console.error('동기화 실패:', error);
			alert('동기화에 실패했습니다.');
		}
	}
</script>

<div class="dark:bg-mofu-dark-800 bg-mofu-light-800 rounded-lg border p-6">
	<div class="mb-6 flex items-center justify-between">
		<div class="flex items-center gap-3">
			<Icon src={Link} size="20" class="text-mofu" />
			<h2 class="text-lg font-bold">외부 API 연결</h2>
		</div>
		
		<Button 
			onclick={() => showAddForm = !showAddForm}
			class="bg-orange-500 hover:bg-orange-600 text-white"
		>
			+ API 연결 추가
		</Button>
	</div>

	<!-- Add New API Form -->
	{#if showAddForm}
		<div class="mb-6 p-6 border rounded-lg dark:bg-mofu-dark-700 bg-mofu-light-700">
			<h3 class="text-base font-semibold mb-6">새 API 연결 추가</h3>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- 좌측 열 -->
				<div class="space-y-4">
					<div>
						<label for="api-name" class="block text-sm font-medium mb-2">
							연결 이름 <span class="text-red-500">*</span>
						</label>
						<Input
							id="api-name"
							bind:value={newConnection.name}
							placeholder="예: Kandji MDM API"
							class="text-sm"
						/>
					</div>
					
					<div>
						<label for="api-category" class="block text-sm font-medium mb-2">
							연결 대상 <span class="text-red-500">*</span>
						</label>
						<Select.Root bind:value={newConnection.category}>
							<Select.Trigger class="w-full text-sm">
								<Select.Value placeholder="연결 대상을 선택하세요" />
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="library-sync">라이브러리 동기화</Select.Item>
								<Select.Item value="user-management">사용자 관리</Select.Item>
								<Select.Item value="device-management">기기 관리</Select.Item>
								<Select.Item value="analytics">분석</Select.Item>
							</Select.Content>
						</Select.Root>
					</div>
				</div>

				<!-- 우측 열 -->
				<div class="space-y-4">
					<div>
						<label for="api-type" class="block text-sm font-medium mb-2">
							연결 타입 <span class="text-red-500">*</span>
						</label>
						<Select.Root bind:value={newConnection.type}>
							<Select.Trigger class="w-full text-sm">
								<Select.Value placeholder="연결 타입을 선택하세요" />
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="REST API">REST API</Select.Item>
								<Select.Item value="GraphQL">GraphQL</Select.Item>
								<Select.Item value="WebSocket">WebSocket</Select.Item>
								<Select.Item value="SOAP">SOAP</Select.Item>
							</Select.Content>
						</Select.Root>
					</div>
				</div>
			</div>

			<!-- API URL -->
			<div class="mt-6">
				<label for="api-url" class="block text-sm font-medium mb-2">
					API URL <span class="text-red-500">*</span>
				</label>
				<div class="space-y-2">
					<div class="flex gap-2">
						<Input
							id="api-url"
							bind:value={newConnection.url}
							placeholder="https://api.example.com/devices"
							type="url"
							class="flex-1 text-sm"
						/>
						<Button 
							onclick={testApiConnection} 
							variant="default" 
							class="bg-blue-500 hover:bg-blue-600 text-white px-4"
							disabled={isTestingConnection}
						>
							{isTestingConnection ? '테스트 중...' : '테스트'}
						</Button>
					</div>
					
					{#if testConnectionResult}
						<div class="p-3 rounded-md text-sm {testConnectionResult.status === 'success' 
							? 'bg-green-50 text-green-700 border border-green-200 dark:bg-green-900/20 dark:text-green-400' 
							: 'bg-red-50 text-red-700 border border-red-200 dark:bg-red-900/20 dark:text-red-400'}">
							<div class="flex items-center gap-2">
								<div class="font-medium">
									{testConnectionResult.status === 'success' ? '✓ 연결 성공' : '✗ 연결 실패'}
								</div>
								{#if testConnectionResult.response_time_ms}
									<span class="text-xs opacity-75">({testConnectionResult.response_time_ms}ms)</span>
								{/if}
							</div>
							<div class="mt-1">{testConnectionResult.message}</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- 설명 -->
			<div class="mt-6">
				<label for="api-description" class="block text-sm font-medium mb-2">설명</label>
				<Textarea
					id="api-description"
					bind:value={newConnection.description}
					placeholder="API 연결에 대한 설명을 입력하세요."
					rows="3"
					class="text-sm"
				/>
			</div>

			<!-- HTTP 헤더 -->
			<div class="mt-6">
				<div class="flex items-center justify-between mb-3">
					<label class="block text-sm font-medium">HTTP 헤더</label>
					<Button onclick={addHeader} variant="ghost" size="sm" class="text-sm">
						<Icon src={Plus} size="16" class="mr-1" />
					</Button>
				</div>
				
				{#each newConnection.headers as header, index}
					<div class="flex gap-2 mb-2">
						<Input
							bind:value={header.key}
							placeholder="헤더 이름 (예: Authorization)"
							class="flex-1 text-sm"
						/>
						<Input
							bind:value={header.value}
							placeholder="헤더 값 (예: Bearer token123)"
							class="flex-1 text-sm"
						/>
						{#if newConnection.headers.length > 1}
							<Button onclick={() => removeHeader(index)} variant="ghost" size="sm" class="text-red-500">
								<Icon src={Trash} size="16" />
							</Button>
						{/if}
					</div>
				{/each}
			</div>

			<!-- 자동 동기화 -->
			<div class="mt-6">
				<div class="flex items-center gap-3">
					<input
						type="checkbox"
						id="auto-sync"
						bind:checked={newConnection.autoSync}
						class="w-4 h-4"
					/>
					<label for="auto-sync" class="text-sm font-medium">자동 동기화 활성화</label>
				</div>
			</div>

			<!-- 필드 매핑 설정 -->
			<div class="mt-6">
				<div class="flex items-center justify-between mb-3">
					<label class="block text-sm font-medium">필드 매핑 설정</label>
					<Button 
						onclick={() => showFieldMapping = !showFieldMapping} 
						variant="ghost" 
						size="sm" 
						class="text-sm"
					>
						{showFieldMapping ? '숨기기' : '고급 설정'}
					</Button>
				</div>
				
				{#if showFieldMapping}
					<div class="border rounded-lg p-4 space-y-4 dark:bg-mofu-dark-600 bg-mofu-light-600">
						<!-- 필드 매핑 -->
						<div>
							<div class="flex items-center justify-between mb-3">
								<h4 class="font-medium text-sm">데이터 필드 매핑</h4>
								<div class="flex gap-2">
									{#if sampleDataForMapping}
										<Button 
											onclick={openFieldMappingDialog} 
											variant="outline" 
											size="sm" 
											class="text-sm"
										>
											<Icon src={Cog6Tooth} size="16" class="mr-1" />
											필드 설정
										</Button>
									{/if}
									<Button onclick={addFieldMapping} variant="ghost" size="sm" class="text-sm">
										<Icon src={Plus} size="16" class="mr-1" />
										매핑 추가
									</Button>
								</div>
							</div>
							
							{#each newConnection.fieldMapping.mappings as mapping, index}
								<div class="grid grid-cols-1 md:grid-cols-5 gap-2 mb-3 p-3 border rounded-md">
									<div>
										<Input
											bind:value={mapping.source_field}
											placeholder="원본 필드명"
											class="text-xs"
										/>
									</div>
									<div class="flex items-center justify-center">
										<Icon src={ArrowRight} size="16" class="text-gray-400" />
									</div>
									<div>
										<Input
											bind:value={mapping.target_field}
											placeholder="대상 필드명"
											class="text-xs"
										/>
									</div>
									<div>
										<Select.Root bind:selected={mapping.data_type}>
											<Select.Trigger class="w-full text-xs">
												<Select.Value placeholder={mapping.data_type} />
											</Select.Trigger>
											<Select.Content>
												<Select.Item value="string">문자열</Select.Item>
												<Select.Item value="number">숫자</Select.Item>
												<Select.Item value="boolean">참/거짓</Select.Item>
												<Select.Item value="date">날짜</Select.Item>
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex gap-1">
										<Input
											bind:value={mapping.default_value}
											placeholder="기본값"
											class="text-xs flex-1"
										/>
										{#if newConnection.fieldMapping.mappings.length > 1}
											<Button 
												onclick={() => removeFieldMapping(index)} 
												variant="ghost" 
												size="sm" 
												class="text-red-500"
											>
												<Icon src={Trash} size="14" />
											</Button>
										{/if}
									</div>
									
									<!-- 변환 옵션 -->
									<div class="md:col-span-5 mt-2">
										<Select.Root bind:selected={mapping.transformation}>
											<Select.Trigger class="w-full text-xs">
												<Select.Value placeholder="변환 옵션 (선택사항)" />
											</Select.Trigger>
											<Select.Content>
												<Select.Item value="uppercase">대문자 변환</Select.Item>
												<Select.Item value="lowercase">소문자 변환</Select.Item>
												<Select.Item value="trim">공백 제거</Select.Item>
												<Select.Item value="title">제목 형식</Select.Item>
											</Select.Content>
										</Select.Root>
									</div>
								</div>
							{/each}
						</div>

						<!-- 필터 조건 -->
						<div>
							<div class="flex items-center justify-between mb-3">
								<h4 class="font-medium text-sm">데이터 필터 조건</h4>
								<Button onclick={addFilterCondition} variant="ghost" size="sm" class="text-sm">
									<Icon src={Plus} size="16" class="mr-1" />
									조건 추가
								</Button>
							</div>
							
							{#each newConnection.fieldMapping.filter_conditions as condition, index}
								<div class="grid grid-cols-1 md:grid-cols-4 gap-2 mb-2 p-2 border rounded-md">
									<div>
										<Input
											bind:value={condition.field}
											placeholder="필드명"
											class="text-xs"
										/>
									</div>
									<div>
										<Select.Root bind:selected={condition.operator}>
											<Select.Trigger class="w-full text-xs">
												<Select.Value placeholder={condition.operator} />
											</Select.Trigger>
											<Select.Content>
												<Select.Item value="equals">같음</Select.Item>
												<Select.Item value="not_equals">다름</Select.Item>
												<Select.Item value="contains">포함</Select.Item>
											</Select.Content>
										</Select.Root>
									</div>
									<div>
										<Input
											bind:value={condition.value}
											placeholder="값"
											class="text-xs"
										/>
									</div>
									<div>
										<Button 
											onclick={() => removeFilterCondition(index)} 
											variant="ghost" 
											size="sm" 
											class="text-red-500"
										>
											<Icon src={Trash} size="14" />
										</Button>
									</div>
								</div>
							{/each}
							
							{#if newConnection.fieldMapping.filter_conditions.length === 0}
								<div class="text-xs text-gray-500 text-center py-2">
									필터 조건이 없으면 모든 데이터가 동기화됩니다.
								</div>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- 버튼들 -->
			<div class="flex justify-end gap-3 mt-8 pt-4 border-t">
				<Button onclick={() => { showAddForm = false; resetForm(); }} variant="outline">
					취소
				</Button>
				<Button onclick={addApiConnection} class="bg-orange-500 hover:bg-orange-600 text-white">
					API 연결 추가
				</Button>
			</div>
		</div>
	{/if}

	<!-- API Connections Cards (responsive) -->
	<div class="space-y-4">
		{#if isLoading}
			<div class="text-center py-8">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-orange-500 mx-auto"></div>
				<p class="text-sm text-gray-500 mt-2">API 연결 목록을 불러오는 중...</p>
			</div>
		{:else}
			{#each apiConnections as connection}
				<div class="border rounded-lg p-4 dark:bg-mofu-dark-700 bg-mofu-light-700">
					<div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
						<!-- 기본 정보 -->
						<div class="flex-1 min-w-0">
							<div class="flex items-center gap-3 mb-2">
								<h3 class="font-medium text-base">{connection.name}</h3>
								<span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium
									{connection.status === 'active' 
										? (connection.hasError 
											? 'bg-orange-100 text-orange-800 dark:bg-orange-800/20 dark:text-orange-400'
											: 'bg-green-100 text-green-800 dark:bg-green-800/20 dark:text-green-400')
										: 'bg-gray-100 text-gray-800 dark:bg-gray-800/20 dark:text-gray-400'
									}">
									{connection.status === 'active' 
										? (connection.hasError ? '에러' : '성공')
										: '비활성'}
								</span>
							</div>
							
							{#if connection.hasError && connection.errorMessage}
								<div class="mb-2 p-2 bg-red-50 text-red-700 text-xs rounded border border-red-200 dark:bg-red-900/20 dark:text-red-400">
									<strong>에러:</strong> {connection.errorMessage}
								</div>
							{/if}
						
						<div class="text-xs text-gray-500 font-mono mb-2 break-all">
							{connection.url}
						</div>
						
						<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2 text-xs">
							<div>
								<span class="font-medium">등록일:</span>
								<div>{connection.registrationDate}</div>
							</div>
							<div>
								<span class="font-medium">마지막 동기화:</span>
								<div>{connection.lastSync}</div>
							</div>
							<div>
								<span class="font-medium">동기화 횟수:</span>
								<div>{connection.actions}회</div>
							</div>
							<div class="flex items-center gap-2">
								<span class="font-medium">자동 동기화:</span>
								<Switch
									checked={connection.syncEnabled}
									onCheckedChange={() => toggleSync(connection.id)}
								/>
							</div>
						</div>
					</div>
					
					<!-- 작업 버튼들 -->
					<div class="flex items-center gap-2 flex-shrink-0">
						<Button
							variant="ghost"
							size="sm"
							onclick={() => openFieldMappingDialog(connection)}
							aria-label="필드 매핑"
						>
							<Icon src={Cog6Tooth} size="16" />
						</Button>
						
						<Button
							variant="ghost"
							size="sm"
							onclick={() => startEditConnection(connection)}
							aria-label="수정"
						>
							<Icon src={Pencil} size="16" />
						</Button>
						
						<Button
							variant="ghost"
							size="sm"
							onclick={() => viewConnection(connection)}
							aria-label="보기"
						>
							<Icon src={Eye} size="16" />
						</Button>
						
						<Button
							variant="ghost"
							size="sm"
							onclick={() => removeConnection(connection.id)}
							aria-label="삭제"
							class="text-red-500 hover:text-red-700"
						>
							<Icon src={Trash} size="16" />
						</Button>
					</div>
					</div>
				</div>
			{/each}
			
			{#if apiConnections.length === 0}
				<div class="text-center py-12">
					<Icon src={Link} size="40" class="mx-auto text-gray-400 mb-4" />
					<p class="text-sm text-gray-500 mb-2">연결된 외부 API가 없습니다</p>
					<p class="text-xs text-gray-400">+ API 연결 추가 버튼을 클릭하여 새로운 API를 연결하세요</p>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- 필드 매핑 다이얼로그 -->
<FieldMappingDialog
	bind:open={showFieldMappingDialog}
	sampleData={sampleDataForMapping}
	existingMapping={editingConnection?.fieldMapping || newConnection.fieldMapping}
	onClose={closeFieldMappingDialog}
	onSave={saveFieldMapping}
/>