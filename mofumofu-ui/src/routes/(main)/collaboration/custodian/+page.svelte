<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Edit, Trash2, Play, FileText, Eye, AlertCircle } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import YamlEditor from '$lib/components/custodian/YamlEditor.svelte';
	import {
		getCustodianPolicies,
		createCustodianPolicy,
		updateCustodianPolicy,
		deleteCustodianPolicy,
		executeCustodianPolicy,
		type CustodianPolicy
	} from '$lib/api/custodian/custodianApi';

	let policies: CustodianPolicy[] = $state([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let selectedPolicy: CustodianPolicy | null = $state(null);
	let showCreateModal = $state(false);
	let showViewModal = $state(false);
	let showEditModal = $state(false);
	let showDeleteConfirm = $state(false);
	let policyToDelete: CustodianPolicy | null = $state(null);
	let newPolicyName = $state('');
	let newPolicyDescription = $state('');
	let newPolicyContent = $state('');
	let editPolicyName = $state('');
	let editPolicyDescription = $state('');
	let editPolicyContent = $state('');

	onMount(async () => {
		await loadPolicies();
	});

	async function loadPolicies() {
		try {
			loading = true;
			error = null;
			policies = await getCustodianPolicies();
		} catch (err) {
			console.error('Failed to load policies:', err);
			error = err instanceof Error ? err.message : 'Failed to load policies';
		} finally {
			loading = false;
		}
	}

	function handleCreate() {
		showCreateModal = true;
	}

	function handleView(policy: CustodianPolicy) {
		selectedPolicy = policy;
		showViewModal = true;
	}

	function handleEdit(policy: CustodianPolicy) {
		selectedPolicy = policy;
		editPolicyName = policy.name;
		editPolicyDescription = policy.description || '';
		editPolicyContent = policy.content;
		showEditModal = true;
	}

	async function handleSaveNew() {
		if (!newPolicyName.trim() || !newPolicyContent.trim()) {
			error = 'Name and content are required';
			return;
		}

		try {
			await createCustodianPolicy({
				name: newPolicyName,
				description: newPolicyDescription || undefined,
				content: newPolicyContent
			});
			await loadPolicies();
			showCreateModal = false;
			newPolicyName = '';
			newPolicyDescription = '';
			newPolicyContent = '';
		} catch (err) {
			console.error('Failed to create policy:', err);
			error = err instanceof Error ? err.message : 'Failed to create policy';
		}
	}

	async function handleSaveEdit() {
		if (!selectedPolicy) return;
		if (!editPolicyName.trim() || !editPolicyContent.trim()) {
			error = 'Name and content are required';
			return;
		}

		try {
			await updateCustodianPolicy(selectedPolicy.id, {
				name: editPolicyName,
				description: editPolicyDescription || undefined,
				content: editPolicyContent
			});
			await loadPolicies();
			showEditModal = false;
			selectedPolicy = null;
		} catch (err) {
			console.error('Failed to update policy:', err);
			error = err instanceof Error ? err.message : 'Failed to update policy';
		}
	}

	function handleDeleteClick(policy: CustodianPolicy) {
		policyToDelete = policy;
		showDeleteConfirm = true;
	}

	async function confirmDelete() {
		if (!policyToDelete) return;

		try {
			await deleteCustodianPolicy(policyToDelete.id);
			await loadPolicies();
			showDeleteConfirm = false;
			policyToDelete = null;
		} catch (err) {
			console.error('Failed to delete policy:', err);
			error = err instanceof Error ? err.message : 'Failed to delete policy';
		}
	}

	async function handleExecute(policy: CustodianPolicy, dryRun: boolean = false) {
		try {
			const result = await executeCustodianPolicy({
				policy_id: policy.id,
				dry_run: dryRun
			});
			alert(`Policy execution ${dryRun ? '(dry-run) ' : ''}started: ${result.execution_id}`);
		} catch (err) {
			console.error('Failed to execute policy:', err);
			error = err instanceof Error ? err.message : 'Failed to execute policy';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleString('ko-KR');
	}
</script>

<div class="container mx-auto p-6">
	<!-- Header -->
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Custodian Policies</h1>
			<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
				Cloud Custodian YAML 정책을 관리하고 실행합니다
			</p>
		</div>
		<Button onclick={handleCreate} class="flex items-center gap-2">
			<Plus class="h-4 w-4" />
			<span>New Policy</span>
		</Button>
	</div>

	<!-- Error Message -->
	{#if error}
		<div class="mb-4 rounded-lg bg-red-50 p-4 dark:bg-red-950/30">
			<div class="flex items-center gap-2">
				<AlertCircle class="h-5 w-5 text-red-600 dark:text-red-400" />
				<p class="text-sm text-red-600 dark:text-red-400">{error}</p>
			</div>
		</div>
	{/if}

	<!-- Loading State -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="text-gray-500 dark:text-gray-400">Loading policies...</div>
		</div>
	{:else if policies.length === 0}
		<!-- Empty State -->
		<div class="flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 py-12 dark:border-gray-700">
			<FileText class="mb-4 h-12 w-12 text-gray-400" />
			<h3 class="mb-2 text-lg font-medium text-gray-900 dark:text-white">No policies yet</h3>
			<p class="mb-4 text-sm text-gray-500 dark:text-gray-400">
				Get started by creating your first Custodian policy
			</p>
			<Button onclick={handleCreate}>Create Policy</Button>
		</div>
	{:else}
		<!-- Policies Grid -->
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each policies as policy}
				<div class="rounded-lg border border-gray-200 bg-white p-4 shadow-sm transition-shadow hover:shadow-md dark:border-gray-700 dark:bg-gray-800">
					<!-- Policy Header -->
					<div class="mb-3 flex items-start justify-between">
						<div class="flex-1">
							<h3 class="font-semibold text-gray-900 dark:text-white">{policy.name}</h3>
							{#if policy.description}
								<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{policy.description}</p>
							{/if}
						</div>
					</div>

					<!-- Metadata -->
					<div class="mb-4 space-y-1 text-xs text-gray-500 dark:text-gray-400">
						<div>Created: {formatDate(policy.created_at)}</div>
						<div>Updated: {formatDate(policy.updated_at)}</div>
					</div>

					<!-- Actions -->
					<div class="flex items-center gap-2">
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleView(policy)}
							class="flex-1"
						>
							<Eye class="h-3.5 w-3.5" />
						</Button>
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleEdit(policy)}
							class="flex-1"
						>
							<Edit class="h-3.5 w-3.5" />
						</Button>
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleExecute(policy, true)}
							class="flex-1"
							title="Dry Run"
						>
							<Play class="h-3.5 w-3.5" />
							Dry
						</Button>
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleExecute(policy, false)}
							class="flex-1"
							title="Execute"
						>
							<Play class="h-3.5 w-3.5 text-green-600 dark:text-green-400" />
						</Button>
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleDeleteClick(policy)}
							class="flex-1"
						>
							<Trash2 class="h-3.5 w-3.5 text-red-600 dark:text-red-400" />
						</Button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Create Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={() => (showCreateModal = false)}>
		<div class="max-h-[90vh] w-full max-w-4xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800" onclick={(e) => e.stopPropagation()}>
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">Create New Policy</h2>

			<!-- Name Input -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Policy Name *
				</label>
				<input
					type="text"
					bind:value={newPolicyName}
					placeholder="e.g., ec2-stop-unused-instances"
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<!-- Description Input -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Description
				</label>
				<input
					type="text"
					bind:value={newPolicyDescription}
					placeholder="Brief description of this policy"
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<!-- YAML Content -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					YAML Content *
				</label>
				<textarea
					bind:value={newPolicyContent}
					placeholder={`policies:
  - name: my-policy
    resource: aws.ec2
    filters:
      - type: value
        key: State.Name
        value: running
    actions:
      - stop`}
					class="min-h-[400px] w-full rounded-md border border-gray-300 px-3 py-2 font-mono text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
				></textarea>
			</div>

			<!-- Actions -->
			<div class="flex justify-end gap-2">
				<Button variant="outline" onclick={() => (showCreateModal = false)}>Cancel</Button>
				<Button onclick={handleSaveNew}>Create Policy</Button>
			</div>
		</div>
	</div>
{/if}

<!-- View Modal -->
{#if showViewModal && selectedPolicy}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
		<div class="max-h-[90vh] w-full max-w-4xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
				{selectedPolicy.name}
			</h2>
			{#if selectedPolicy.description}
				<p class="mb-4 text-sm text-gray-500 dark:text-gray-400">{selectedPolicy.description}</p>
			{/if}
			<pre class="rounded-lg bg-gray-100 p-4 text-sm dark:bg-gray-900"><code>{selectedPolicy.content}</code></pre>
			<div class="mt-4 flex justify-end">
				<Button onclick={() => (showViewModal = false)}>Close</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Edit Modal -->
{#if showEditModal && selectedPolicy}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={() => (showEditModal = false)}>
		<div class="max-h-[90vh] w-full max-w-4xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800" onclick={(e) => e.stopPropagation()}>
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
				Edit Policy: {selectedPolicy.name}
			</h2>

			<!-- Name Input -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Policy Name *
				</label>
				<input
					type="text"
					bind:value={editPolicyName}
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<!-- Description Input -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Description
				</label>
				<input
					type="text"
					bind:value={editPolicyDescription}
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<!-- YAML Content -->
			<div class="mb-4">
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					YAML Content *
				</label>
				<textarea
					bind:value={editPolicyContent}
					class="min-h-[400px] w-full rounded-md border border-gray-300 px-3 py-2 font-mono text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
				></textarea>
			</div>

			<!-- Actions -->
			<div class="flex justify-end gap-2">
				<Button variant="outline" onclick={() => (showEditModal = false)}>Cancel</Button>
				<Button onclick={handleSaveEdit}>Save Changes</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && policyToDelete}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
		<div class="w-full max-w-md rounded-lg bg-white p-6 dark:bg-gray-800">
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">Delete Policy</h2>
			<p class="mb-6 text-gray-600 dark:text-gray-400">
				Are you sure you want to delete "<span class="font-semibold">{policyToDelete.name}</span>"? This action cannot be undone.
			</p>
			<div class="flex justify-end gap-2">
				<Button variant="outline" onclick={() => (showDeleteConfirm = false)}>Cancel</Button>
				<Button variant="destructive" onclick={confirmDelete}>Delete</Button>
			</div>
		</div>
	</div>
{/if}
