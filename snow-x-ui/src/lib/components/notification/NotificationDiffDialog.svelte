<script lang="ts">
import type { Notification } from '$lib/api/notification';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
import { Button } from '$lib/components/ui/button';
import { goto } from '$app/navigation';

	interface Props {
		open: boolean;
		notification: Notification | null;
		onClose: () => void;
	}

	let { open, notification, onClose }: Props = $props();

	const diffEntries = $derived(notification?.payload?.diff ?? []);

	function formatValue(value: unknown) {
		if (value === undefined) return '—';
		if (value === null) return 'null';
		return String(value);
	}
</script>

<Dialog {open} onOpenChange={(value) => (!value ? onClose() : null)}>
	<DialogContent class="w-full max-w-3xl border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-900">
		<DialogHeader class="border-b border-gray-200 pb-4 dark:border-gray-700">
			<DialogTitle class="text-lg font-semibold text-gray-900 dark:text-white">
				{notification?.title ?? '알림 상세'}
			</DialogTitle>
			<p class="text-sm text-gray-500 dark:text-gray-400">
				{notification?.message ?? '변경된 상세 정보를 확인하세요.'}
			</p>
		</DialogHeader>

		{#if notification}
			<div class="space-y-6 pt-4">
				<div class="grid gap-4 rounded-lg border border-gray-100 bg-gray-50 p-4 text-sm dark:border-gray-800 dark:bg-gray-800/60">
					<div class="flex justify-between text-gray-600 dark:text-gray-300">
						<span>작업자</span>
						<span class="font-medium text-gray-900 dark:text-white">
							{notification.payload?.actor_name ?? '시스템'}
							{#if notification.payload?.actor_handle}
								<span class="text-gray-500 dark:text-gray-400">@{notification.payload.actor_handle}</span>
							{/if}
						</span>
					</div>
					{#if notification.payload?.actor_email}
						<div class="flex justify-between text-gray-600 dark:text-gray-300">
							<span>이메일</span>
							<span class="text-gray-900 dark:text-white">{notification.payload.actor_email}</span>
						</div>
					{/if}
					<div class="flex justify-between text-gray-600 dark:text-gray-300">
						<span>발생 시각</span>
						<span class="text-gray-900 dark:text-white">{new Date(notification.created_at).toLocaleString('ko-KR')}</span>
					</div>
				</div>

				{#if diffEntries.length}
					<div>
						<div class="mb-2 flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
							<span>변경된 필드: {diffEntries.length}개</span>
							{#if notification.payload?.link}
								<Button variant="outline" size="sm" onclick={() => goto(notification.payload?.link as string)}>
									상세 페이지 이동
								</Button>
							{/if}
						</div>
						<div class="overflow-hidden rounded-lg border border-gray-200 dark:border-gray-800">
							<table class="w-full text-sm">
								<thead class="bg-gray-100 text-left text-xs font-semibold uppercase text-gray-500 dark:bg-gray-800 dark:text-gray-400">
									<tr>
										<th class="px-4 py-2">필드</th>
										<th class="px-4 py-2">변경 전</th>
										<th class="px-4 py-2">변경 후</th>
									</tr>
								</thead>
								<tbody class="divide-y divide-gray-100 dark:divide-gray-800">
									{#each diffEntries as entry}
										<tr class="bg-white dark:bg-gray-900">
											<td class="px-4 py-3 font-medium text-gray-900 dark:text-white">{entry.field}</td>
											<td class="px-4 py-3 font-mono text-xs text-red-500 dark:text-red-300">
												{formatValue(entry.before)}
											</td>
											<td class="px-4 py-3 font-mono text-xs text-green-600 dark:text-green-300">
												{formatValue(entry.after)}
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				{:else}
					<div class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-sm text-gray-500 dark:border-gray-800 dark:bg-gray-800/50 dark:text-gray-400">
						표시할 diff 정보가 없습니다.
					</div>
				{/if}
			</div>
		{/if}
	</DialogContent>
</Dialog>
