<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Search, Pencil, Trash2, User, Mail, Phone, Building2, Briefcase } from 'lucide-svelte';
	import { contactApi, type Contact, type CreateContactRequest, type UpdateContactRequest } from '$lib/api/office';

	interface ContactFormData {
		name: string;
		title: string;
		department: string;
		phone: string;
		mobile: string;
		email: string;
		office_location: string;
		responsibilities: string;
	}

	let contacts = $state<Contact[]>([]);
	let totalCount = $state(0);
	let currentPage = $state(1);
	let pageSize = $state(20);
	let searchQuery = $state('');
	let departmentFilter = $state('');
	let isActiveFilter = $state<boolean | undefined>(undefined);
	let isLoading = $state(false);
	let showCreateDialog = $state(false);
	let showEditDialog = $state(false);
	let selectedContact = $state<Contact | null>(null);

	let formData = $state<ContactFormData>({
		name: '',
		title: '',
		department: '',
		phone: '',
		mobile: '',
		email: '',
		office_location: '',
		responsibilities: ''
	});

	const totalPages = $derived(Math.ceil(totalCount / pageSize));

	async function loadContacts() {
		isLoading = true;
		try {
			const params: any = {
				page: currentPage,
				limit: pageSize
			};

			if (searchQuery) params.search = searchQuery;
			if (departmentFilter) params.department = departmentFilter;
			if (isActiveFilter !== undefined) params.is_active = isActiveFilter;

			const response = await contactApi.getContacts(params);

			contacts = response.contacts;
			totalCount = response.total;
		} catch (error) {
			console.error('Failed to load contacts:', error);
		} finally {
			isLoading = false;
		}
	}

	function openCreateDialog() {
		formData = {
			name: '',
			title: '',
			department: '',
			phone: '',
			mobile: '',
			email: '',
			office_location: '',
			responsibilities: ''
		};
		showCreateDialog = true;
	}

	function openEditDialog(contact: Contact) {
		selectedContact = contact;
		formData = {
			name: contact.name,
			title: contact.title || '',
			department: contact.department || '',
			phone: contact.phone || '',
			mobile: contact.mobile || '',
			email: contact.email || '',
			office_location: contact.office_location || '',
			responsibilities: contact.responsibilities || ''
		};
		showEditDialog = true;
	}

	async function handleCreate(event: SubmitEvent) {
		event.preventDefault();
		try {
			await contactApi.createContact(formData);

			showCreateDialog = false;
			await loadContacts();
		} catch (error) {
			console.error('Failed to create contact:', error);
		}
	}

	async function handleUpdate(event: SubmitEvent) {
		event.preventDefault();
		if (!selectedContact) return;

		try {
			await contactApi.updateContact(selectedContact.id, formData);

			showEditDialog = false;
			selectedContact = null;
			await loadContacts();
		} catch (error) {
			console.error('Failed to update contact:', error);
		}
	}

	async function handleDelete(id: string) {
		if (!confirm('이 담당자를 삭제하시겠습니까?')) return;

		try {
			await contactApi.deleteContact(id);
			await loadContacts();
		} catch (error) {
			console.error('Failed to delete contact:', error);
		}
	}

	onMount(() => {
		loadContacts();
	});
</script>

<div class="min-h-screen bg-gray-50 p-6 dark:bg-gray-900">
	<div class="mx-auto max-w-7xl">
		<!-- Header -->
		<div class="mb-6 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">담당자 관리</h1>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					디바이스 담당자를 관리합니다
				</p>
			</div>
			<button
				onclick={openCreateDialog}
				class="flex items-center gap-2 rounded-lg bg-orange-600 px-4 py-2 font-medium text-white transition hover:bg-orange-700"
			>
				<Plus class="h-5 w-5" />
				담당자 추가
			</button>
		</div>

		<!-- Stats -->
		<div class="mb-6 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-purple-100 p-2 dark:bg-purple-900">
					<User class="h-5 w-5 text-purple-600 dark:text-purple-400" />
				</div>
				<div>
					<p class="text-sm text-gray-500 dark:text-gray-400">전체 담당자</p>
					<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">{totalCount}</p>
				</div>
			</div>
		</div>

		<!-- Filters and Search -->
		<div class="mb-4 rounded-lg bg-white p-4 shadow-sm dark:bg-gray-800">
			<div class="grid gap-4 md:grid-cols-3">
				<div class="relative">
					<Search class="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
					<input
						type="text"
						bind:value={searchQuery}
						onchange={loadContacts}
						placeholder="담당자 검색..."
						class="w-full rounded-lg border border-gray-300 py-2 pl-10 pr-4 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
					/>
				</div>
				<input
					type="text"
					bind:value={departmentFilter}
					onchange={loadContacts}
					placeholder="부서 필터"
					class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
				/>
				<select
					bind:value={isActiveFilter}
					onchange={loadContacts}
					class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
				>
					<option value={undefined}>모든 상태</option>
					<option value={true}>활성</option>
					<option value={false}>비활성</option>
				</select>
			</div>
		</div>

		<!-- Contact List -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100">
					<User class="h-5 w-5" />
					담당자 목록
				</h2>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="h-8 w-8 animate-spin rounded-full border-4 border-orange-600 border-t-transparent"></div>
				</div>
			{:else if contacts.length === 0}
				<div class="py-12 text-center">
					<p class="text-gray-500 dark:text-gray-400">담당자가 없습니다</p>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-900">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">담당자명</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">역할/부서</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">연락처</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">이메일</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">직급/부서</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">상태</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">작업</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
							{#each contacts as contact}
								<tr class="hover:bg-gray-50 dark:hover:bg-gray-700">
									<td class="whitespace-nowrap px-6 py-4">
										<div class="flex items-center">
											<div class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-purple-100 dark:bg-purple-900">
												<User class="h-5 w-5 text-purple-600 dark:text-purple-400" />
											</div>
											<div class="ml-4">
												<div class="text-sm font-medium text-gray-900 dark:text-gray-100">{contact.name}</div>
												{#if contact.office_location}
													<div class="text-sm text-gray-500 dark:text-gray-400">{contact.office_location}</div>
												{/if}
											</div>
										</div>
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-gray-100">
										{#if contact.title}
											<span class="flex items-center gap-1">
												<Briefcase class="h-4 w-4" />
												{contact.title}
											</span>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-gray-100">
										{#if contact.mobile || contact.phone}
											<div class="flex flex-col gap-1">
												{#if contact.mobile}
													<span class="flex items-center gap-1">
														<Phone class="h-4 w-4" />
														{contact.mobile}
													</span>
												{/if}
												{#if contact.phone}
													<span class="text-gray-500">{contact.phone}</span>
												{/if}
											</div>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-gray-100">
										{#if contact.email}
											<span class="flex items-center gap-1">
												<Mail class="h-4 w-4" />
												{contact.email}
											</span>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-gray-100">
										{#if contact.department}
											<span class="flex items-center gap-1">
												<Building2 class="h-4 w-4" />
												{contact.department}
											</span>
										{:else}
											<span class="text-gray-400">-</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-6 py-4">
										{#if contact.is_active}
											<span class="rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-800 dark:bg-green-900 dark:text-green-200">
												활성
											</span>
										{:else}
											<span class="rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-800 dark:bg-red-900 dark:text-red-200">
												비활성
											</span>
										{/if}
									</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm font-medium">
										<div class="flex items-center gap-2">
											<button
												onclick={() => openEditDialog(contact)}
												class="text-orange-600 hover:text-orange-900 dark:text-orange-400 dark:hover:text-orange-300"
											>
												<Pencil class="h-4 w-4" />
											</button>
											<button
												onclick={() => handleDelete(contact.id)}
												class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
											>
												<Trash2 class="h-4 w-4" />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}

			<!-- Pagination -->
			{#if !isLoading && totalCount > 0}
				<div class="border-t border-gray-200 bg-white px-6 py-4 dark:border-gray-700 dark:bg-gray-800">
					<div class="flex items-center justify-between">
						<div class="text-sm text-gray-700 dark:text-gray-300">
							전체 {totalCount}개 중 {(currentPage - 1) * pageSize + 1}-{Math.min(
								currentPage * pageSize,
								totalCount
							)}
						</div>
						<div class="flex gap-2">
							<button
								onclick={() => {
									if (currentPage > 1) {
										currentPage--;
										loadContacts();
									}
								}}
								disabled={currentPage <= 1}
								class="rounded-lg border border-gray-300 px-3 py-1 text-sm disabled:opacity-50 dark:border-gray-600 dark:text-gray-300"
							>
								이전
							</button>
							<span class="flex items-center px-3 text-sm text-gray-700 dark:text-gray-300">
								{currentPage} / {totalPages}
							</span>
							<button
								onclick={() => {
									if (currentPage < totalPages) {
										currentPage++;
										loadContacts();
									}
								}}
								disabled={currentPage >= totalPages}
								class="rounded-lg border border-gray-300 px-3 py-1 text-sm disabled:opacity-50 dark:border-gray-600 dark:text-gray-300"
							>
								다음
							</button>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Create Dialog -->
{#if showCreateDialog}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={(e) => {
			if (e.target === e.currentTarget) showCreateDialog = false;
		}}
	>
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-gray-100">담당자 추가</h2>
			<form onsubmit={handleCreate}>
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">이름 *</label>
						<input
							type="text"
							bind:value={formData.name}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">직급</label>
						<input
							type="text"
							bind:value={formData.title}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">부서</label>
						<input
							type="text"
							bind:value={formData.department}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">전화번호</label>
						<input
							type="tel"
							bind:value={formData.phone}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">휴대폰</label>
						<input
							type="tel"
							bind:value={formData.mobile}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">이메일</label>
						<input
							type="email"
							bind:value={formData.email}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">오피스 위치</label>
						<input
							type="text"
							bind:value={formData.office_location}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">담당 업무</label>
						<textarea
							bind:value={formData.responsibilities}
							rows="3"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						></textarea>
					</div>
				</div>
				<div class="mt-6 flex justify-end gap-2">
					<button
						type="button"
						onclick={() => (showCreateDialog = false)}
						class="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
					>
						추가
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Edit Dialog -->
{#if showEditDialog}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={(e) => {
			if (e.target === e.currentTarget) showEditDialog = false;
		}}
	>
		<div class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-gray-100">담당자 수정</h2>
			<form onsubmit={handleUpdate}>
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">이름 *</label>
						<input
							type="text"
							bind:value={formData.name}
							required
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">직급</label>
						<input
							type="text"
							bind:value={formData.title}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">부서</label>
						<input
							type="text"
							bind:value={formData.department}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">전화번호</label>
						<input
							type="tel"
							bind:value={formData.phone}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">휴대폰</label>
						<input
							type="tel"
							bind:value={formData.mobile}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div>
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">이메일</label>
						<input
							type="email"
							bind:value={formData.email}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">오피스 위치</label>
						<input
							type="text"
							bind:value={formData.office_location}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						/>
					</div>
					<div class="md:col-span-2">
						<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">담당 업무</label>
						<textarea
							bind:value={formData.responsibilities}
							rows="3"
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100"
						></textarea>
					</div>
				</div>
				<div class="mt-6 flex justify-end gap-2">
					<button
						type="button"
						onclick={() => (showEditDialog = false)}
						class="rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					>
						취소
					</button>
					<button
						type="submit"
						class="rounded-lg bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
					>
						저장
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
