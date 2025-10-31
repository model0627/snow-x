<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { UserPlus, Search } from '@lucide/svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { contactApi, deviceApi, type Contact, type DeviceContact } from '$lib/api/office';

	interface Props {
		open: boolean;
		deviceId: string;
		deviceName: string;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, deviceId, deviceName, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let contacts = $state<Contact[]>([]);
	let assignedContacts = $state<DeviceContact[]>([]);
	let assignSearchQuery = $state('');
	let manageSearchQuery = $state('');
	let activeTab = $state<'assign' | 'manage'>('assign');
	let roleInput = $state('');

	const isDesktop = $derived(desktopStore.isDesktop);

	$effect(() => {
		if (open) {
			initialize();
		}
	});

	async function initialize() {
		assignSearchQuery = '';
		manageSearchQuery = '';
		roleInput = '';
		activeTab = 'assign';

		await Promise.all([loadContacts(), loadAssignedContacts()]);
	}

	async function loadContacts() {
		try {
			const response = await contactApi.getContacts({
				page: 1,
				limit: 100,
				is_active: true
			});
			contacts = response.contacts;
		} catch (error) {
			console.error('Failed to load contacts:', error);
			contacts = [];
		}
	}

	async function loadAssignedContacts() {
		try {
			const response = await deviceApi.getAssignedContacts(deviceId);
			assignedContacts = response.mappings;
		} catch (error) {
			console.error('Failed to load assigned contacts:', error);
			assignedContacts = [];
		}
	}

	const filteredContacts = $derived.by(() => {
		if (!assignSearchQuery.trim()) return contacts;
		const query = assignSearchQuery.toLowerCase();
		return contacts.filter((contact) => {
			const searchTargets = [
				contact.name,
				contact.email,
				contact.phone,
				contact.mobile,
				contact.department,
				contact.title
			]
				.filter(Boolean)
				.map((value) => value!.toLowerCase());

			return searchTargets.some((target) => target.includes(query));
		});
	});

	const filteredAssignedContacts = $derived.by(() => {
		if (!manageSearchQuery.trim()) return assignedContacts;
		const query = manageSearchQuery.toLowerCase();
		return assignedContacts.filter((contact) => {
			const targets = [
				contact.contact_name,
				contact.role ?? ''
			]
				.map((value) => value.toLowerCase());
			return targets.some((target) => target.includes(query));
		});
	});

	async function handleAssignContact(contact: Contact) {
		loading = true;
		try {
			await deviceApi.assignContact(deviceId, contact.id, roleInput.trim() ? roleInput.trim() : undefined);
			await Promise.all([loadAssignedContacts(), loadContacts()]);
			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to assign contact:', error);
			alert('담당자 연결에 실패했습니다.');
		} finally {
			loading = false;
		}
	}

	async function handleUnassignContact(contact: DeviceContact) {
		if (!confirm(`${contact.contact_name} 담당자 연결을 해제하시겠습니까?`)) return;

		loading = true;
		try {
			await deviceApi.unassignContact(deviceId, contact.contact_id);
			await loadAssignedContacts();
			onSuccess();
		} catch (error) {
			console.error('Failed to unassign contact:', error);
			alert('담당자 연결 해제에 실패했습니다.');
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="max-h-[90vh] w-full max-w-3xl overflow-y-auto border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader class="sticky top-0 z-10 bg-white pb-4 dark:bg-gray-800">
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
					<UserPlus class="h-5 w-5 text-purple-600 dark:text-purple-400" />
				</div>
				<div>
					<DialogTitle class="{isDesktop ? 'text-lg' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
						담당자 연결 관리
					</DialogTitle>
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{deviceName}</p>
				</div>
			</div>
		</DialogHeader>

		<div class="border-b border-gray-200 dark:border-gray-700">
			<div class="flex gap-6 px-1">
				<button
					onclick={() => activeTab = 'assign'}
					class="border-b-2 pb-3 text-sm font-medium transition-colors {activeTab === 'assign'
						? 'border-purple-600 text-purple-600 dark:border-purple-400 dark:text-purple-400'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					담당자 연결
				</button>
				<button
					onclick={() => activeTab = 'manage'}
					class="border-b-2 pb-3 text-sm font-medium transition-colors {activeTab === 'manage'
						? 'border-purple-600 text-purple-600 dark:border-purple-400 dark:text-purple-400'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					연결된 담당자 ({assignedContacts.length})
				</button>
			</div>
		</div>

		<div class="space-y-4 pt-4">
			{#if activeTab === 'assign'}
				<div class="space-y-4">
					<div class="grid gap-4 md:grid-cols-2">
						<div class="relative">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Search class="h-4 w-4 text-gray-400" />
							</div>
							<input
								type="text"
								bind:value={assignSearchQuery}
								placeholder="이름, 이메일, 연락처로 검색..."
								class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10 text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop ? 'text-sm' : 'text-base'}"
							/>
						</div>
						<input
							type="text"
							bind:value={roleInput}
							placeholder="역할 (선택 입력)"
							class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 px-3 text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop ? 'text-sm' : 'text-base'}"
						/>
					</div>

					{#if filteredContacts.length === 0}
						<div class="flex flex-col items-center justify-center py-12 text-center">
							<UserPlus class="mb-4 h-12 w-12 text-gray-300 dark:text-gray-600" />
							<p class="mb-1 text-sm font-medium text-gray-900 dark:text-gray-100">연결 가능한 담당자가 없습니다</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">검색 조건을 변경하거나 새로운 담당자를 추가하세요.</p>
						</div>
					{:else}
						<div class="max-h-96 space-y-2 overflow-y-auto pr-1">
							{#each filteredContacts as contact (contact.id)}
								<div class="flex items-center justify-between rounded-lg border border-gray-200 p-3 transition-all hover:border-purple-500 hover:bg-purple-50 dark:border-gray-700 dark:hover:border-purple-400 dark:hover:bg-purple-900/20">
									<div>
										<p class="font-medium text-gray-900 dark:text-gray-100">{contact.name}</p>
										<div class="mt-1 flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-500 dark:text-gray-400">
											{#if contact.title}<span>{contact.title}</span>{/if}
											{#if contact.department}<span>{contact.department}</span>{/if}
											{#if contact.email}<span>{contact.email}</span>{/if}
											{#if contact.mobile}<span>{contact.mobile}</span>{/if}
										</div>
									</div>
									<Button
										variant="outline"
										disabled={loading}
										onclick={() => handleAssignContact(contact)}
										class="border-purple-600 text-purple-600 hover:bg-purple-600 hover:text-white dark:border-purple-400 dark:text-purple-300 dark:hover:bg-purple-500/20"
									>
										연결
									</Button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{:else}
				<div class="space-y-4">
					<div class="relative max-w-sm">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<Search class="h-4 w-4 text-gray-400" />
						</div>
						<input
							type="text"
							bind:value={manageSearchQuery}
							placeholder="연결된 담당자 검색..."
							class="block w-full rounded-lg border border-gray-300 bg-white py-2.5 pr-3 pl-10 text-gray-900 placeholder-gray-400 transition-colors focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white {isDesktop ? 'text-sm' : 'text-base'}"
						/>
					</div>

					{#if filteredAssignedContacts.length === 0}
						<div class="rounded-lg border border-dashed border-gray-300 p-6 text-center text-sm text-gray-500 dark:border-gray-700 dark:text-gray-400">
							연결된 담당자가 없습니다.
						</div>
					{:else}
						<div class="space-y-2">
							{#each filteredAssignedContacts as contact (contact.id)}
								<div class="flex items-center justify-between rounded-lg border border-gray-200 p-3 transition hover:border-red-400 hover:bg-red-50 dark:border-gray-700 dark:hover:border-red-400 dark:hover:bg-red-900/20">
									<div>
										<p class="font-medium text-gray-900 dark:text-gray-100">{contact.contact_name}</p>
										<div class="mt-1 flex flex-wrap gap-2 text-xs text-gray-500 dark:text-gray-400">
											{#if contact.role}
												<span class="rounded bg-purple-100 px-2 py-0.5 text-xs font-semibold uppercase tracking-wide text-purple-800 dark:bg-purple-800 dark:text-purple-100">
													{contact.role}
												</span>
											{/if}
											<span class="text-gray-400 dark:text-gray-500">연결일 {new Date(contact.created_at).toLocaleString('ko-KR')}</span>
										</div>
									</div>
									<Button
										variant="destructive"
										disabled={loading}
										onclick={() => handleUnassignContact(contact)}
									>
										연결 해제
									</Button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</DialogContent>
</Dialog>
