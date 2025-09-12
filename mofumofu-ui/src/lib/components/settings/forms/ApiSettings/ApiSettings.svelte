<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Select from '$lib/components/ui/select';
	import { Icon, Link, Trash, Pencil, Cog6Tooth, Eye, EyeSlash, Plus } from 'svelte-hero-icons';

	// API 연결 상태
	let apiConnections = $state([
		{
			id: 'ehr-1',
			name: 'eHR',
			url: 'https://6883db6274530380a37090c.mockapi.io/api/v1/e...',
			lastSync: '2025.09.02. 오전 07:51',
			status: 'active',
			syncEnabled: true,
			registrationDate: '2025.09.10 20:17',
			actions: 5
		}
	]);

	let showAddForm = $state(false);
	let newConnection = $state({
		name: '',
		type: 'REST API',
		category: '라이브러리 동기화',
		url: '',
		description: '',
		headers: [{ key: '', value: '' }],
		autoSync: false
	});

	function addApiConnection() {
		if (newConnection.name && newConnection.url) {
			const newId = `api-${Date.now()}`;
			apiConnections = [...apiConnections, {
				id: newId,
				name: newConnection.name,
				url: newConnection.url,
				lastSync: '연결 대기 중',
				status: 'pending',
				syncEnabled: false,
				registrationDate: new Date().toLocaleString('ko-KR'),
				actions: 0
			}];
			
			// 폼 리셋
			newConnection = {
				name: '',
				type: 'REST API',
				category: '라이브러리 동기화',
				url: '',
				description: '',
				headers: [{ key: '', value: '' }],
				autoSync: false
			};
			showAddForm = false;
		}
	}

	function removeConnection(id: string) {
		apiConnections = apiConnections.filter(conn => conn.id !== id);
	}

	function toggleSync(id: string) {
		apiConnections = apiConnections.map(conn => 
			conn.id === id 
				? { ...conn, syncEnabled: !conn.syncEnabled }
				: conn
		);
	}

	function refreshConnection(id: string) {
		const connection = apiConnections.find(conn => conn.id === id);
		if (connection) {
			connection.lastSync = new Date().toLocaleString('ko-KR');
			connection.status = 'active';
		}
	}

	function addHeader() {
		newConnection.headers = [...newConnection.headers, { key: '', value: '' }];
	}

	function removeHeader(index: number) {
		newConnection.headers = newConnection.headers.filter((_, i) => i !== index);
	}

	function testApiConnection() {
		// API 테스트 로직
		alert('API 연결을 테스트합니다.');
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
						<Select.Root>
							<Select.Trigger class="w-full text-sm">
								<Select.Value placeholder={newConnection.category} />
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
						<Select.Root>
							<Select.Trigger class="w-full text-sm">
								<Select.Value placeholder={newConnection.type} />
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
				<div class="flex gap-2">
					<Input
						id="api-url"
						bind:value={newConnection.url}
						placeholder="https://api.example.com/devices"
						type="url"
						class="flex-1 text-sm"
					/>
					<Button onclick={testApiConnection} variant="default" class="bg-blue-500 hover:bg-blue-600 text-white px-4">
						테스트
					</Button>
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

			<!-- 버튼들 -->
			<div class="flex justify-end gap-3 mt-8 pt-4 border-t">
				<Button onclick={() => showAddForm = false} variant="outline">
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
		{#each apiConnections as connection}
			<div class="border rounded-lg p-4 dark:bg-mofu-dark-700 bg-mofu-light-700">
				<div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
					<!-- 기본 정보 -->
					<div class="flex-1 min-w-0">
						<div class="flex items-center gap-3 mb-2">
							<h3 class="font-medium text-base">{connection.name}</h3>
							<span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium
								{connection.status === 'active' 
									? 'bg-green-100 text-green-800 dark:bg-green-800/20 dark:text-green-400'
									: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-800/20 dark:text-yellow-400'
								}">
								{connection.status === 'active' ? '성공' : '대기'}
							</span>
						</div>
						
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
							onclick={() => refreshConnection(connection.id)}
							aria-label="새로고침"
						>
							<Icon src={Cog6Tooth} size="16" />
						</Button>
						
						<Button
							variant="ghost"
							size="sm"
							onclick={() => {}}
							aria-label="수정"
						>
							<Icon src={Pencil} size="16" />
						</Button>
						
						<Button
							variant="ghost"
							size="sm"
							onclick={() => {}}
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
	</div>

	{#if apiConnections.length === 0}
		<div class="text-center py-12">
			<Icon src={Link} size="40" class="mx-auto text-gray-400 mb-4" />
			<p class="text-sm text-gray-500 mb-2">연결된 외부 API가 없습니다</p>
			<p class="text-xs text-gray-400">+ API 연결 추가 버튼을 클릭하여 새로운 API를 연결하세요</p>
		</div>
	{/if}
</div>