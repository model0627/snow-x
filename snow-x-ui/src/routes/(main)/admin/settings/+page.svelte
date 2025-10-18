<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Settings, Clock, ListChecks, History, RefreshCw } from 'lucide-svelte';
	import {
		externalApiService,
		type ExternalApiConnection,
		type ExternalApiSyncLog,
		type ConnectionTargetType
	} from '$lib/api/external-api';

	type AdminMenu = {
		id: string;
		label: string;
		description: string;
		icon: typeof Settings;
	};

	type CelerySchedule = {
		id: string;
		task: string;
		intervalSeconds: number;
		description: string;
		source: string;
	};

	type SyncLogRow = ExternalApiSyncLog & {
		connectionId: number;
		connectionName: string;
		targetType: ConnectionTargetType;
	};

	const adminMenus: AdminMenu[] = [
		{
			id: 'logs',
			label: '동기화 히스토리',
			description: '외부 API 동기화 결과를 빠르게 살펴봅니다.',
			icon: History
		},
		{
			id: 'celery',
			label: 'Celery 스케줄',
			description: '주기적 작업과 비동기 워커 상태를 점검합니다.',
			icon: ListChecks
		}
	];

	let activeMenu = $state(adminMenus[0]!.id);
	let recentLogs = $state<SyncLogRow[]>([]);
	let logsLoading = $state(false);
	let logsError = $state<string | null>(null);

	const celerySchedules: CelerySchedule[] = [
		{
			id: 'cleanup-expired-refresh-tokens',
			task: 'cleanup_expired_refresh_tokens',
			intervalSeconds: 3600,
			description: '만료된 리프레시 토큰을 정리해 인증 저장소를 깔끔하게 유지합니다.',
			source: 'tasks/app/tasks/token_tasks.py'
		},
		{
			id: 'reindex-all-posts-daily',
			task: 'reindex_all_posts',
			intervalSeconds: 86400,
			description: '게시글 검색 인덱스를 새로 구축해 검색 품질을 보장합니다.',
			source: 'tasks/app/tasks/post_tasks.py'
		},
		{
			id: 'check-meilisearch-health',
			task: 'check_meilisearch_health',
			intervalSeconds: 1800,
			description: 'Meilisearch 인스턴스 헬스 체크로 인덱싱 장애를 조기에 감지합니다.',
			source: 'tasks/app/tasks/search_tasks.py'
		},
		{
			id: 'cleanup-old-system-events',
			task: 'cleanup_old_system_events',
			intervalSeconds: 86400,
			description: '오래된 시스템 이벤트 로그를 삭제해 저장소 용량을 확보합니다.',
			source: 'tasks/app/tasks/token_tasks.py'
		},
		{
			id: 'sync-all-counts-daily',
			task: 'sync_all_counts',
			intervalSeconds: 86400,
			description: '좋아요·팔로우 등 누적 카운트를 다시 집계해 데이터 정합성을 맞춥니다.',
			source: 'tasks/app/tasks/count_tasks.py'
		},
		{
			id: 'external-api-sync',
			task: 'external_api.sync_all_active',
			intervalSeconds: 300,
			description: '외부 API 연결의 next_sync_at을 확인해 필요한 동기화를 큐에 등록합니다.',
			source: 'tasks/app/tasks/external_api_sync.py'
		},
		{
			id: 'cleanup-old-api-data',
			task: 'external_api.cleanup_old_data',
			intervalSeconds: 86400,
			description: '아카이브된 외부 동기화 데이터와 오래된 로그를 주기적으로 삭제합니다.',
			source: 'tasks/app/tasks/external_api_sync.py'
		}
	];

	const targetTypeLabels: Record<ConnectionTargetType, string> = {
		device: '디바이스',
		device_library: '라이브러리',
		contact: '담당자'
	};

	function formatInterval(seconds: number): string {
		if (seconds % 3600 === 0) {
			const hours = seconds / 3600;
			return `${hours}시간`;
		}
		if (seconds % 60 === 0) {
			const minutes = seconds / 60;
			return `${minutes}분`;
		}
		return `${seconds}초`;
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('ko-KR', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}

	function formatDuration(ms?: number): string {
		if (!ms) return '-';
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(2)}초`;
	}

	async function loadRecentLogs() {
		logsLoading = true;
		logsError = null;
		try {
			const connections = await externalApiService.listConnections(0, 20);
			if (!connections.length) {
				recentLogs = [];
				return;
			}

			const logPromises = connections.map(async (connection: ExternalApiConnection) => {
				try {
					const logs = await externalApiService.getSyncLogs(connection.id, 0, 5);
					return logs.map((log) => ({
						...log,
						connectionId: connection.id,
						connectionName: connection.name,
						targetType: connection.target_type
					}));
				} catch (error) {
					console.error('Failed to load logs for connection', connection.id, error);
					return [] as SyncLogRow[];
				}
			});

			const results = await Promise.all(logPromises);
			recentLogs = results
				.flat()
				.sort((a, b) => new Date(b.started_at).getTime() - new Date(a.started_at).getTime())
				.slice(0, 10);
		} catch (error) {
			console.error('Failed to load recent sync logs:', error);
			logsError = error instanceof Error ? error.message : '동기화 로그를 불러오지 못했습니다.';
		} finally {
			logsLoading = false;
		}
	}

	onMount(() => {
		loadRecentLogs();
	});
</script>

<div class="min-h-screen bg-gray-50 p-6 dark:bg-gray-900">
	<div class="mx-auto max-w-6xl space-y-6">
	<header class="flex flex-col gap-2">
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-full bg-orange-500/10">
				<Settings class="h-5 w-5 text-orange-500" />
			</div>
			<div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">관리자 설정</h1>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					운영에 필요한 백오피스 도구와 스케줄을 확인하고 관리합니다.
				</p>
			</div>
		</div>
	</header>

		<nav class="flex flex-wrap gap-3">
			{#each adminMenus as menu}
				<Button
					variant={activeMenu === menu.id ? 'default' : 'ghost'}
					class="flex items-center gap-2 rounded-full border border-transparent px-4 py-2 text-sm transition hover:-translate-y-0.5 hover:shadow-sm dark:hover:bg-gray-800"
					onclick={() => (activeMenu = menu.id)}
				>
					<svelte:component this={menu.icon} class="h-4 w-4" />
					<span>{menu.label}</span>
				</Button>
			{/each}
		</nav>

		{#if activeMenu === 'celery'}
			<section class="space-y-6">
				<div class="rounded-2xl bg-white p-6 shadow-sm ring-1 ring-gray-100 dark:bg-gray-800 dark:ring-gray-700/60">
					<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
						<div>
							<h2 class="flex items-center gap-2 text-xl font-semibold text-gray-900 dark:text-gray-50">
								<ListChecks class="h-5 w-5 text-orange-500" />
								Celery 스케줄 개요
							</h2>
							<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
								Celery Beat 구성은 Python 태스크 API의 <code class="rounded bg-gray-100 px-1 py-0.5 text-xs text-gray-700 dark:bg-gray-900 dark:text-gray-200">tasks/app/core/celery_app.py</code>에서 관리됩니다.
							</p>
						</div>
						<div class="flex items-center gap-2 rounded-full bg-orange-500/10 px-4 py-2 text-xs font-semibold text-orange-600 dark:bg-orange-400/10 dark:text-orange-300">
							<Clock class="h-4 w-4" />
							<span>Celery Beat 모니터링 권장</span>
						</div>
					</div>
				</div>

				<div class="overflow-hidden rounded-2xl bg-white shadow-sm ring-1 ring-gray-100 dark:bg-gray-800 dark:ring-gray-700/60">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-900/40">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									ID
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									Celery 태스크
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									주기
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									설명
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									정의 위치
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each celerySchedules as schedule}
								<tr class="hover:bg-gray-50 dark:hover:bg-gray-900/50">
									<td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900 dark:text-gray-100">
										{schedule.id}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-200">
										{schedule.task}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-200">
										{formatInterval(schedule.intervalSeconds)}
										<span class="ml-2 text-xs text-gray-400 dark:text-gray-500">
											({schedule.intervalSeconds.toLocaleString()}초)
										</span>
									</td>
									<td class="px-6 py-4 text-sm text-gray-700 dark:text-gray-200">
										{schedule.description}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
										{schedule.source}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<p class="text-sm text-gray-500 dark:text-gray-400">
					주기 변경이 필요하면 <code class="rounded bg-gray-100 px-1 py-0.5 text-xs text-gray-700 dark:bg-gray-900 dark:text-gray-200">tasks/app/core/celery_app.py</code>를 수정한 뒤 Celery Beat 프로세스를 재시작하세요. 자동 동기화가 동작하려면 각 연결의 <code class="text-orange-500">auto_sync</code>와 <code class="text-orange-500">is_active</code> 속성이 활성 상태여야 합니다.
				</p>
			</section>
		{/if}

		{#if activeMenu === 'logs'}
		<section class="space-y-4">
			<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
				<div>
					<h2 class="flex items-center gap-2 text-xl font-semibold text-gray-900 dark:text-gray-50">
						<History class="h-5 w-5 text-orange-500" />
						최근 외부 동기화 히스토리
					</h2>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
						외부 API 연결에서 최근 실행된 동기화 작업의 상태와 소요 시간을 확인합니다.
					</p>
				</div>
				<Button
					variant="ghost"
					class="gap-2 rounded-full border border-gray-200 px-4 py-2 text-sm text-gray-600 hover:-translate-y-0.5 hover:bg-gray-50 hover:shadow-sm dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-800"
					onclick={loadRecentLogs}
					disabled={logsLoading}
				>
					<RefreshCw class="h-4 w-4 {logsLoading ? 'animate-spin' : ''}" />
					새로고침
				</Button>
			</div>

			<div class="overflow-hidden rounded-2xl bg-white shadow-sm ring-1 ring-gray-100 dark:bg-gray-800 dark:ring-gray-700/60">
				{#if logsLoading}
					<div class="flex items-center justify-center py-12">
						<RefreshCw class="h-8 w-8 animate-spin text-gray-400" />
					</div>
				{:else if logsError}
					<div class="px-6 py-5 text-sm text-red-600 dark:text-red-400">
						{logsError}
					</div>
				{:else if recentLogs.length === 0}
					<div class="px-6 py-5 text-sm text-gray-500 dark:text-gray-400">
						최근 동기화 기록이 없습니다.
					</div>
				{:else}
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-900/40">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									연결
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									대상
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									상태
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									시작 시간
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									소요 시간
								</th>
								<th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
									결과
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each recentLogs as log}
								<tr class="hover:bg-gray-50 dark:hover:bg-gray-900/30">
									<td class="px-6 py-4 text-sm font-medium text-gray-900 dark:text-gray-100">
										{log.connectionName}
									</td>
									<td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-300">
										{targetTypeLabels[log.targetType]}
									</td>
									<td class="px-6 py-4 text-sm">
										{#if log.status === 'success'}
											<span class="inline-flex items-center gap-2 rounded-full bg-green-100 px-3 py-1 text-xs font-semibold text-green-700 dark:bg-green-900 dark:text-green-300">
												<div class="h-2 w-2 rounded-full bg-green-500"></div>
												완료
											</span>
										{:else if log.status === 'error'}
											<span class="inline-flex items-center gap-2 rounded-full bg-red-100 px-3 py-1 text-xs font-semibold text-red-700 dark:bg-red-900 dark:text-red-300">
												<div class="h-2 w-2 rounded-full bg-red-500"></div>
												실패
											</span>
										{:else}
											<span class="inline-flex items-center gap-2 rounded-full bg-blue-100 px-3 py-1 text-xs font-semibold text-blue-700 dark:bg-blue-900 dark:text-blue-300">
												<div class="h-2 w-2 rounded-full bg-blue-500"></div>
												진행중
											</span>
										{/if}
									</td>
									<td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-300">
										{formatDateTime(log.started_at)}
									</td>
									<td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-300">
										{formatDuration(log.duration)}
									</td>
									<td class="px-6 py-4 text-sm text-gray-700 dark:text-gray-200">
										{#if log.error_message}
											<span class="text-red-600 dark:text-red-400">{log.error_message}</span>
										{:else}
											총 {log.records_processed}건 처리
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				{/if}
			</div>
		</section>
		{/if}
	</div>
</div>
