<script lang="ts">
	import { onMount } from 'svelte';
	import { X, RefreshCw, CheckCircle2, XCircle, Clock } from 'lucide-svelte';
	import { externalApiService, type ExternalApiSyncLog } from '$lib/api/external-api';

	let {
		open = $bindable(false),
		connectionId,
		connectionName
	}: {
		open: boolean;
		connectionId: number;
		connectionName: string;
	} = $props();

	let logs = $state<ExternalApiSyncLog[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	async function loadLogs() {
		loading = true;
		error = null;
		try {
			logs = await externalApiService.getSyncLogs(connectionId, 0, 50);
		} catch (err) {
			error = err instanceof Error ? err.message : '로그를 불러오는데 실패했습니다';
		} finally {
			loading = false;
		}
	}

	function formatDuration(ms?: number): string {
		if (!ms) return '-';
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(2)}초`;
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

	function getStatusBadge(status: string) {
		switch (status) {
			case 'success':
				return { icon: CheckCircle2, text: '완료', class: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' };
			case 'error':
				return { icon: XCircle, text: '실패', class: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300' };
			case 'in_progress':
				return { icon: Clock, text: '진행중', class: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300' };
			default:
				return { icon: Clock, text: status, class: 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300' };
		}
	}

	$effect(() => {
		if (open) {
			loadLogs();
		}
	});
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
		onclick={(e) => {
			if (e.target === e.currentTarget) open = false;
		}}
	>
		<div
			class="relative w-full max-w-5xl max-h-[90vh] overflow-hidden rounded-lg bg-white shadow-xl dark:bg-gray-800"
		>
			<!-- Header -->
			<div class="flex items-center justify-between border-b p-6 dark:border-gray-700">
				<div>
					<h2 class="text-xl font-semibold text-gray-900 dark:text-white">동기화 히스토리</h2>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
						선택한 연결의 동기화 기록을 확인할 수 있습니다
					</p>
				</div>
				<div class="flex items-center gap-2">
					<button
						onclick={loadLogs}
						disabled={loading}
						class="rounded-lg p-2 text-gray-500 hover:bg-gray-100 disabled:opacity-50 dark:text-gray-400 dark:hover:bg-gray-700"
						title="새로고침"
					>
						<RefreshCw class="h-5 w-5 {loading ? 'animate-spin' : ''}" />
					</button>
					<button
						onclick={() => (open = false)}
						class="rounded-lg p-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700"
					>
						<X class="h-5 w-5" />
					</button>
				</div>
			</div>

			<!-- Content -->
			<div class="overflow-y-auto p-6" style="max-height: calc(90vh - 180px);">
				{#if loading}
					<div class="flex items-center justify-center py-12">
						<RefreshCw class="h-8 w-8 animate-spin text-gray-400" />
					</div>
				{:else if error}
					<div class="rounded-lg bg-red-50 p-4 dark:bg-red-900/20">
						<p class="text-sm text-red-800 dark:text-red-300">{error}</p>
					</div>
				{:else if logs.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-center">
						<Clock class="mb-4 h-12 w-12 text-gray-400" />
						<p class="text-sm text-gray-500 dark:text-gray-400">아직 동기화 기록이 없습니다</p>
					</div>
				{:else}
					<div class="overflow-hidden rounded-lg border dark:border-gray-700">
						<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
							<thead class="bg-gray-50 dark:bg-gray-900">
								<tr>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										상태
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										연결 이름
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										시작 시간
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										소요 시간
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										처리 결과
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										실행 형태
									</th>
									<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
										실행자
									</th>
								</tr>
							</thead>
							<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
								{#each logs as log}
									{@const status = getStatusBadge(log.status)}
									<tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
										<td class="whitespace-nowrap px-6 py-4">
											<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium {status.class}">
												<svelte:component this={status.icon} class="h-3.5 w-3.5" />
												{status.text}
											</span>
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-gray-100">
											{connectionName}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
											{formatDateTime(log.started_at)}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
											{formatDuration(log.duration)}
										</td>
										<td class="px-6 py-4 text-sm">
											{#if log.error_message}
												<div class="max-w-md">
													<p class="text-red-600 dark:text-red-400">{log.error_message}</p>
												</div>
											{:else}
												<span class="text-green-600 dark:text-green-400">총 {log.records_processed}개 처리</span>
											{/if}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
											<span class="inline-flex items-center gap-1">
												<span class="h-2 w-2 rounded-full bg-purple-400"></span>
												수동
											</span>
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
											홍길동
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
{/if}
