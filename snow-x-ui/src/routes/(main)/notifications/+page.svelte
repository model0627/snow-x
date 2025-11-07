<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import {
		notificationApi,
		type Notification,
		type NotificationListResponse,
		type CreateNotificationRequest
	} from '$lib/api/notification';
	import NotificationDiffDialog from '$lib/components/notification/NotificationDiffDialog.svelte';

	const STATUS_OPTIONS = ['pending', 'processing', 'done', 'failed'] as const;

	let notifications = $state<Notification[]>([]);
	let isLoading = $state(false);
	let error = $state('');
	let page = $state(1);
	let limit = $state(20);
	let total = $state(0);
	let statusFilter = $state('');
	let channelFilter = $state('');
	let refreshing = $state(false);
	let autoRefresh: ReturnType<typeof setInterval> | null = null;
	const AUTO_REFRESH_INTERVAL = 3000;
	let diffDialogOpen = $state(false);
	let diffNotification = $state<Notification | null>(null);

let newNotificationForm = $state<CreateNotificationRequest>({
	channel: 'web',
	title: '',
	message: '',
	payload: {}
});
	let payloadInput = $state('{\n  "url": "https://example.com/webhook"\n}');
	let creating = $state(false);
	let createError = $state('');

	const totalPages = $derived(Math.max(1, Math.ceil(total / limit)));

	async function loadNotifications(options?: { silent?: boolean }) {
		try {
			if (!options?.silent) {
				isLoading = true;
			}
			error = '';
			const response = await notificationApi.getNotifications({
				page,
				limit,
				status: statusFilter || undefined,
				channel: channelFilter || undefined
			});
			applyResponse(response);
		} catch (err) {
			console.error('Failed to load notifications:', err);
			error = '알림 목록을 불러오지 못했습니다.';
		} finally {
			if (!options?.silent) {
				isLoading = false;
			}
		}
	}

	function applyResponse(response: NotificationListResponse) {
		notifications = response.notifications;
		total = response.total;
		page = response.page;
		limit = response.limit;
	}

	onMount(() => {
		loadNotifications();
		if (typeof window !== 'undefined') {
			autoRefresh = window.setInterval(() => {
				loadNotifications({ silent: true });
			}, AUTO_REFRESH_INTERVAL);
		}

		return () => {
			if (autoRefresh) {
				window.clearInterval(autoRefresh);
				autoRefresh = null;
			}
		};
	});

	async function handleStatusUpdate(notification: Notification, status: string) {
		if (notification.status === status) return;
		try {
			await notificationApi.updateNotificationStatus(notification.id, { status });
			await loadNotifications();
		} catch (err) {
			console.error('Failed to update notification status:', err);
			error = '상태 업데이트에 실패했습니다.';
		}
	}

	async function handleRefresh() {
		refreshing = true;
		await loadNotifications({ silent: true });
		refreshing = false;
	}

	function formatDate(value?: string) {
		if (!value) return '-';
		return new Date(value).toLocaleString('ko-KR');
	}

	async function handleCreateNotification(event: SubmitEvent) {
		event.preventDefault();
		createError = '';
		creating = true;

		try {
			let parsedPayload: Record<string, unknown> | undefined = undefined;
			if (payloadInput.trim()) {
				try {
					parsedPayload = JSON.parse(payloadInput);
				} catch (err) {
					createError = 'Payload JSON 형식이 올바르지 않습니다.';
					return;
				}
			}

			await notificationApi.createNotification({
				...newNotificationForm,
				payload: parsedPayload
			});

			newNotificationForm = {
				channel: newNotificationForm.channel,
				title: '',
				message: '',
				payload: parsedPayload
			};
			await loadNotifications();
		} catch (err) {
			console.error('Failed to create notification:', err);
			createError = '알림 생성에 실패했습니다.';
		} finally {
			creating = false;
		}
	}

	function handlePageChange(direction: 'prev' | 'next') {
		if (direction === 'prev' && page > 1) {
			page -= 1;
			loadNotifications();
		}
		if (direction === 'next' && page < totalPages) {
			page += 1;
			loadNotifications();
		}
	}

	function openDiffModal(notification: Notification) {
		diffNotification = notification;
		diffDialogOpen = true;
	}

	function closeDiffModal() {
		diffDialogOpen = false;
	}
</script>

<div class="min-h-screen bg-gray-50 p-6 dark:bg-gray-900">
	<div class="mx-auto max-w-6xl space-y-6">
		<div class="flex flex-wrap items-center justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">알림 큐</h1>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					백엔드 알림 아웃박스 상태를 확인하고 수동으로 상태를 조정할 수 있습니다.
				</p>
			</div>
			<Button onclick={handleRefresh} disabled={refreshing || isLoading}>
				{refreshing ? '새로고침 중...' : '새로고침'}
			</Button>
		</div>

		<div class="rounded-xl bg-white p-4 shadow-sm dark:bg-gray-800">
			<form class="grid gap-4 md:grid-cols-4">
				<div>
					<label
						class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300"
						for="status-filter"
					>
						상태
					</label>
					<select
						id="status-filter"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
						bind:value={statusFilter}
						onchange={() => {
							page = 1;
							loadNotifications();
						}}
					>
						<option value="">전체</option>
						{#each STATUS_OPTIONS as status}
							<option value={status}>{status}</option>
						{/each}
					</select>
				</div>
				<div>
					<label
						class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300"
						for="channel-filter"
					>
						채널
					</label>
					<input
						id="channel-filter"
						type="text"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
						placeholder="예: slack, webhook"
						bind:value={channelFilter}
						onchange={() => {
							page = 1;
							loadNotifications();
						}}
					/>
				</div>
				<div>
					<label
						class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300"
						for="limit-select"
					>
						페이지당 개수
					</label>
					<select
						id="limit-select"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
						bind:value={limit}
						onchange={() => {
							page = 1;
							loadNotifications();
						}}
					>
						<option value={10}>10</option>
						<option value={20}>20</option>
						<option value={50}>50</option>
					</select>
				</div>
				<div class="flex items-end">
					<div class="text-sm text-gray-500 dark:text-gray-400">
						총 {total.toLocaleString()}건 / {page} / {totalPages}페이지
					</div>
				</div>
			</form>
		</div>

		{#if error}
			<div class="rounded-lg border border-red-200 bg-red-50 p-4 text-sm text-red-700 dark:border-red-500/40 dark:bg-red-500/10 dark:text-red-200">
				{error}
			</div>
		{/if}

		<div class="overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
			{#if isLoading}
				<div class="py-12 text-center text-gray-500 dark:text-gray-400">불러오는 중...</div>
			{:else if notifications.length === 0}
				<div class="py-12 text-center text-gray-500 dark:text-gray-400">표시할 알림이 없습니다.</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-[1000px] divide-y divide-gray-200 text-sm dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-900">
							<tr>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">ID</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">채널</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">요청자</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">제목</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">상태</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">예약 시각</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">처리 시각</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">재시도</th>
								<th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">작업</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-100 dark:divide-gray-800">
							{#each notifications as notification}
								{@const isUnread = notification.status !== 'done'}
								<tr
									class={`bg-white hover:bg-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700/50 ${isUnread ? '' : 'opacity-75'}`}
								>
									<td class="px-4 py-3 font-mono text-xs text-gray-500 dark:text-gray-400">
										{notification.id.slice(0, 8)}…
									</td>
									<td class="px-4 py-3 text-gray-900 dark:text-gray-100">
										<div class="capitalize">{notification.channel}</div>
										{#if notification.category}
											<div class="text-xs text-gray-500 dark:text-gray-400">{notification.category}</div>
										{/if}
									</td>
									<td class="px-4 py-3 text-gray-900 dark:text-gray-100">
										<div>{notification.payload?.actor_name ?? '시스템'}</div>
										{#if notification.payload?.actor_handle}
											<div class="text-xs text-gray-500 dark:text-gray-400">@{notification.payload.actor_handle}</div>
										{/if}
										{#if notification.payload?.actor_email}
											<div class="text-xs text-gray-400 dark:text-gray-500">{notification.payload.actor_email}</div>
										{/if}
									</td>
									<td class="px-4 py-3">
										<div class={isUnread ? 'text-gray-900 dark:text-gray-100 font-semibold' : 'text-gray-500 dark:text-gray-400'}>
											{notification.title || '-'}
										</div>
										{#if notification.message}
											<div class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">{notification.message}</div>
										{/if}
										{#if notification.payload?.diff?.length}
											<div class="mt-2 rounded border border-gray-200 text-xs dark:border-gray-700">
												<div class="grid grid-cols-2 bg-gray-50 px-2 py-1 font-medium text-gray-600 dark:bg-gray-900 dark:text-gray-300">
													<span>항목</span>
													<span class="text-right">변경</span>
												</div>
												{#each notification.payload.diff as change}
													<div class="grid grid-cols-2 border-t border-gray-100 px-2 py-1 text-[11px] dark:border-gray-800">
														<span class="text-gray-600 dark:text-gray-300">{change.field}</span>
														<span class="text-right text-gray-500 dark:text-gray-400">
															<span class="line-through text-red-500/80 dark:text-red-400/80">{change.before ?? '-'}</span>
															<span class="mx-1 text-gray-400">→</span>
															<span class="text-green-600 dark:text-green-400">{change.after ?? '-'}</span>
														</span>
													</div>
												{/each}
											</div>
											<Button variant="ghost" size="xs" class="mt-2" onclick={() => openDiffModal(notification)}>
												상세 diff 보기
											</Button>
										{/if}
									</td>
									<td class="px-4 py-3">
										<label class="sr-only" for={`status-${notification.id}`}>상태</label>
										<select
											id={`status-${notification.id}`}
											class="rounded-lg border border-gray-300 px-2 py-1 text-xs capitalize dark:border-gray-600 dark:bg-gray-900"
											value={notification.status}
										onchange={(event) =>
												handleStatusUpdate(notification, (event.target as HTMLSelectElement).value)}
										>
											{#each STATUS_OPTIONS as status}
												<option value={status}>{status}</option>
											{/each}
										</select>
										{#if notification.last_error}
											<div class="mt-1 text-xs text-red-500 dark:text-red-300">
												{notification.last_error}
											</div>
										{/if}
									</td>
									<td class="px-4 py-3 text-gray-900 dark:text-gray-100">{formatDate(notification.scheduled_at)}</td>
									<td class="px-4 py-3 text-gray-900 dark:text-gray-100">{formatDate(notification.processed_at)}</td>
									<td class="px-4 py-3 text-gray-900 dark:text-gray-100">
										{notification.retry_count} / {notification.max_retries}
									</td>
									<td class="px-4 py-3">
										<div class="flex gap-2">
											<Button variant="outline" size="sm" onclick={() => handleStatusUpdate(notification, 'pending')}>
												대기
											</Button>
											<Button variant="outline" size="sm" onclick={() => handleStatusUpdate(notification, 'failed')}>
												실패
											</Button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<div class="flex items-center justify-between border-t border-gray-200 bg-gray-50 px-4 py-3 text-sm dark:border-gray-700 dark:bg-gray-900">
					<Button variant="outline" size="sm" disabled={page === 1} onclick={() => handlePageChange('prev')}>
						이전
					</Button>
					<div class="text-gray-600 dark:text-gray-300">
						Page {page} of {totalPages}
					</div>
					<Button variant="outline" size="sm" disabled={page >= totalPages} onclick={() => handlePageChange('next')}>
						다음
					</Button>
				</div>
			{/if}
		</div>

		<div class="rounded-xl border border-dashed border-gray-300 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
			<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">테스트 알림 생성</h2>
			<p class="text-sm text-gray-500 dark:text-gray-400">간단한 메시지를 큐에 넣어 채널 동작을 확인할 수 있습니다.</p>

			<form class="mt-4 space-y-4" onSubmit={handleCreateNotification}>
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300" for="create-channel"
							>채널</label
						>
						<select
							id="create-channel"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
							bind:value={newNotificationForm.channel}
						>
							<option value="web">웹 알림</option>
							<option value="webhook">Webhook</option>
							<option value="slack">Slack</option>
						</select>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300" for="create-category"
							>카테고리</label
						>
						<input
							id="create-category"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
							placeholder="예: alert, report"
							bind:value={newNotificationForm.category}
						/>
					</div>
				</div>
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300" for="create-title"
							>제목</label
						>
						<input
							id="create-title"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
							bind:value={newNotificationForm.title}
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300" for="create-message"
							>메시지</label
						>
						<input
							id="create-message"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
							bind:value={newNotificationForm.message}
						/>
					</div>
				</div>
				<div>
					<label
						class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300"
						for="payload-input"
					>
						Payload (JSON)
					</label>
					<textarea
						id="payload-input"
						rows="6"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 font-mono text-xs dark:border-gray-700 dark:bg-gray-900"
						bind:value={payloadInput}
					></textarea>
					<p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
						채널에 필요한 추가 정보를 JSON 형식으로 입력하세요. 예) 웹훅 URL, Slack Block 등
					</p>
				</div>
				{#if createError}
					<div class="rounded-md border border-red-200 bg-red-50 px-4 py-2 text-sm text-red-600 dark:border-red-500/40 dark:bg-red-500/10 dark:text-red-200">
						{createError}
					</div>
				{/if}
				<Button type="submit" disabled={creating}>
					{creating ? '생성 중...' : '알림 생성'}
				</Button>
			</form>
		</div>
	</div>
</div>

<NotificationDiffDialog open={diffDialogOpen} notification={diffNotification} onClose={closeDiffModal} />
