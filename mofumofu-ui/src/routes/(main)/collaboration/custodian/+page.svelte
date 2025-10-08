<script lang="ts">
	import { onMount } from 'svelte';
	import { Plus, Edit, Trash2, Play, FileText, Eye, AlertCircle, CheckCircle, History, Clock, XCircle, Loader2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import CodeMirrorYamlEditor from '$lib/components/custodian/CodeMirrorYamlEditor.svelte';
	import {
		getCustodianPolicies,
		createCustodianPolicy,
		updateCustodianPolicy,
		deleteCustodianPolicy,
		executeCustodianPolicy,
		getPolicyExecutions,
		getExecutionResult,
		getTaskStatus,
		type CustodianPolicy,
		type CustodianExecution,
		type TaskStatusResponse
	} from '$lib/api/custodian/custodianApi';

	let policies: CustodianPolicy[] = $state([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let selectedPolicy: CustodianPolicy | null = $state(null);
	let showCreateModal = $state(false);
	let showViewModal = $state(false);
	let showEditModal = $state(false);
	let showDeleteConfirm = $state(false);
	let showExecutionsModal = $state(false);
	let showExecutionDetailModal = $state(false);
	let policyToDelete: CustodianPolicy | null = $state(null);
	let executions: CustodianExecution[] = $state([]);
	let selectedExecution: CustodianExecution | null = $state(null);
	let executionsLoading = $state(false);
	let newPolicyName = $state('');
	let newPolicyDescription = $state('');
	let newPolicyContent = $state('');
	let editPolicyName = $state('');
	let editPolicyDescription = $state('');
	let editPolicyContent = $state('');
	let createValidationError = $state<string | null>(null);
	let editValidationError = $state<string | null>(null);

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
		if (!newPolicyName.trim()) {
			error = 'Policy name is required';
			return;
		}

		if (!newPolicyContent.trim()) {
			error = 'YAML content is required';
			return;
		}

		if (createValidationError) {
			error = 'Please fix YAML validation errors before saving';
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
			createValidationError = null;
		} catch (err) {
			console.error('Failed to create policy:', err);
			error = err instanceof Error ? err.message : 'Failed to create policy';
		}
	}

	async function handleSaveEdit() {
		if (!selectedPolicy) return;

		if (!editPolicyName.trim()) {
			error = 'Policy name is required';
			return;
		}

		if (!editPolicyContent.trim()) {
			error = 'YAML content is required';
			return;
		}

		if (editValidationError) {
			error = 'Please fix YAML validation errors before saving';
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
			editValidationError = null;
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
			alert(`Policy execution ${dryRun ? '(dry-run) ' : ''}started: ${result.id}`);
			// 실행 내역이 열려있으면 자동으로 새로고침
			if (showExecutionsModal && selectedPolicy?.id === policy.id) {
				await loadExecutions(policy);
			}
		} catch (err) {
			console.error('Failed to execute policy:', err);
			error = err instanceof Error ? err.message : 'Failed to execute policy';
		}
	}

	async function handleViewExecutions(policy: CustodianPolicy) {
		selectedPolicy = policy;
		showExecutionsModal = true;
		await loadExecutions(policy);
	}

	async function loadExecutions(policy: CustodianPolicy) {
		try {
			executionsLoading = true;
			executions = await getPolicyExecutions(policy.id);
		} catch (err) {
			console.error('Failed to load executions:', err);
			error = err instanceof Error ? err.message : 'Failed to load executions';
		} finally {
			executionsLoading = false;
		}
	}

	async function handleViewExecutionDetail(execution: CustodianExecution) {
		selectedExecution = execution;
		showExecutionDetailModal = true;
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return 'text-green-600 dark:text-green-400';
			case 'failed':
				return 'text-red-600 dark:text-red-400';
			case 'running':
				return 'text-blue-600 dark:text-blue-400';
			default:
				return 'text-gray-600 dark:text-gray-400';
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'completed':
				return CheckCircle;
			case 'failed':
				return XCircle;
			case 'running':
				return Loader2;
			default:
				return Clock;
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
					<div class="flex flex-col gap-2">
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
								onclick={() => handleDeleteClick(policy)}
								class="flex-1"
							>
								<Trash2 class="h-3.5 w-3.5 text-red-600 dark:text-red-400" />
							</Button>
						</div>
						<div class="flex items-center gap-2">
							<Button
								size="sm"
								variant="outline"
								onclick={() => handleExecute(policy, true)}
								class="flex-1"
								title="Dry Run"
							>
								<Play class="h-3.5 w-3.5" />
								Dry Run
							</Button>
							<Button
								size="sm"
								variant="outline"
								onclick={() => handleExecute(policy, false)}
								class="flex-1"
								title="Execute"
							>
								<Play class="h-3.5 w-3.5 text-green-600 dark:text-green-400" />
								Execute
							</Button>
						</div>
						<Button
							size="sm"
							variant="outline"
							onclick={() => handleViewExecutions(policy)}
							class="w-full"
						>
							<History class="h-3.5 w-3.5" />
							View Executions
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
				<div class="mb-2 flex items-center justify-between">
					<label class="text-sm font-medium text-gray-700 dark:text-gray-300">
						YAML Content *
					</label>
					{#if !createValidationError && newPolicyContent.trim()}
						<div class="flex items-center gap-1 text-xs text-green-600 dark:text-green-400">
							<CheckCircle class="h-3.5 w-3.5" />
							<span>Valid YAML</span>
						</div>
					{/if}
				</div>
				<div class="h-[500px]">
					<CodeMirrorYamlEditor
						bind:value={newPolicyContent}
						placeholder="policies:
  - name: my-policy
    resource: aws.ec2
    filters:
      - type: value
        key: State.Name
        value: running
    actions:
      - stop"
						onValidationError={(err) => createValidationError = err}
					/>
				</div>
			</div>

			<!-- Helper Tips -->
			<div class="mb-4 rounded-lg bg-blue-50 p-3 dark:bg-blue-950/30">
				<p class="text-xs text-blue-600 dark:text-blue-400">
					<strong>💡 Tips:</strong> Use Tab for indentation (2 spaces) • Press Ctrl+Z to undo • Ctrl+Y to redo
				</p>
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
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={() => (showViewModal = false)}>
		<div class="max-h-[90vh] w-full max-w-4xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800" onclick={(e) => e.stopPropagation()}>
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
				{selectedPolicy.name}
			</h2>
			{#if selectedPolicy.description}
				<p class="mb-4 text-sm text-gray-500 dark:text-gray-400">{selectedPolicy.description}</p>
			{/if}
			<div class="h-[500px]">
				<CodeMirrorYamlEditor
					value={selectedPolicy.content}
					readOnly={true}
				/>
			</div>
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
				<div class="mb-2 flex items-center justify-between">
					<label class="text-sm font-medium text-gray-700 dark:text-gray-300">
						YAML Content *
					</label>
					{#if !editValidationError && editPolicyContent.trim()}
						<div class="flex items-center gap-1 text-xs text-green-600 dark:text-green-400">
							<CheckCircle class="h-3.5 w-3.5" />
							<span>Valid YAML</span>
						</div>
					{/if}
				</div>
				<div class="h-[500px]">
					<CodeMirrorYamlEditor
						bind:value={editPolicyContent}
						onValidationError={(err) => editValidationError = err}
					/>
				</div>
			</div>

			<!-- Helper Tips -->
			<div class="mb-4 rounded-lg bg-blue-50 p-3 dark:bg-blue-950/30">
				<p class="text-xs text-blue-600 dark:text-blue-400">
					<strong>💡 Tips:</strong> Use Tab for indentation (2 spaces) • Press Ctrl+Z to undo • Ctrl+Y to redo
				</p>
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

<!-- Executions Modal -->
{#if showExecutionsModal && selectedPolicy}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={() => (showExecutionsModal = false)}>
		<div class="max-h-[90vh] w-full max-w-4xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800" onclick={(e) => e.stopPropagation()}>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
					Execution History: {selectedPolicy.name}
				</h2>
				<Button size="sm" variant="outline" onclick={() => loadExecutions(selectedPolicy!)}>
					Refresh
				</Button>
			</div>

			{#if executionsLoading}
				<div class="flex items-center justify-center py-12">
					<Loader2 class="h-6 w-6 animate-spin text-gray-500" />
				</div>
			{:else if executions.length === 0}
				<div class="flex flex-col items-center justify-center py-12">
					<History class="mb-4 h-12 w-12 text-gray-400" />
					<p class="text-sm text-gray-500 dark:text-gray-400">No executions yet</p>
				</div>
			{:else}
				<div class="space-y-3">
					{#each executions as execution}
						{@const StatusIcon = getStatusIcon(execution.status)}
						<div class="rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-900">
							<div class="flex items-start justify-between">
								<div class="flex-1">
									<div class="flex items-center gap-2">
										<StatusIcon class={`h-5 w-5 ${getStatusColor(execution.status)} ${execution.status === 'running' ? 'animate-spin' : ''}`} />
										<span class="font-mono text-sm text-gray-600 dark:text-gray-400">
											{execution.id.substring(0, 8)}...
										</span>
										<span class={`text-sm font-medium uppercase ${getStatusColor(execution.status)}`}>
											{execution.status}
										</span>
										{#if execution.dry_run}
											<span class="rounded bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200">
												DRY RUN
											</span>
										{/if}
									</div>
									<div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
										<div>Started: {formatDate(execution.started_at || execution.created_at)}</div>
										{#if execution.completed_at}
											<div>Completed: {formatDate(execution.completed_at)}</div>
										{/if}
									</div>
									{#if execution.error}
										<div class="mt-2 rounded bg-red-50 p-2 text-xs text-red-600 dark:bg-red-950/30 dark:text-red-400">
											{execution.error}
										</div>
									{/if}
								</div>
								<Button
									size="sm"
									variant="outline"
									onclick={() => handleViewExecutionDetail(execution)}
								>
									<Eye class="h-3.5 w-3.5" />
									Details
								</Button>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			<div class="mt-4 flex justify-end">
				<Button onclick={() => (showExecutionsModal = false)}>Close</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Execution Detail Modal -->
{#if showExecutionDetailModal && selectedExecution}
	{@const StatusIcon = getStatusIcon(selectedExecution.status)}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={() => (showExecutionDetailModal = false)}>
		<div class="max-h-[90vh] w-full max-w-5xl overflow-y-auto rounded-lg bg-white p-6 dark:bg-gray-800" onclick={(e) => e.stopPropagation()}>
			<h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
				Execution Details
			</h2>

			<!-- Status and Metadata -->
			<div class="mb-4 rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-900">
				<div class="grid grid-cols-2 gap-4">
					<div>
						<div class="text-xs text-gray-500 dark:text-gray-400">Execution ID</div>
						<div class="font-mono text-sm text-gray-900 dark:text-white">{selectedExecution.id}</div>
					</div>
					<div>
						<div class="text-xs text-gray-500 dark:text-gray-400">Status</div>
						<div class="flex items-center gap-2">
							<StatusIcon class={`h-5 w-5 ${getStatusColor(selectedExecution.status)} ${selectedExecution.status === 'running' ? 'animate-spin' : ''}`} />
							<span class={`text-sm font-medium uppercase ${getStatusColor(selectedExecution.status)}`}>
								{selectedExecution.status}
							</span>
							{#if selectedExecution.dry_run}
								<span class="rounded bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-200">
									DRY RUN
								</span>
							{/if}
						</div>
					</div>
					<div>
						<div class="text-xs text-gray-500 dark:text-gray-400">Started At</div>
						<div class="text-sm text-gray-900 dark:text-white">
							{formatDate(selectedExecution.started_at || selectedExecution.created_at)}
						</div>
					</div>
					{#if selectedExecution.completed_at}
						<div>
							<div class="text-xs text-gray-500 dark:text-gray-400">Completed At</div>
							<div class="text-sm text-gray-900 dark:text-white">
								{formatDate(selectedExecution.completed_at)}
							</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- Error Message -->
			{#if selectedExecution.error}
				<div class="mb-4">
					<h3 class="mb-2 text-sm font-medium text-gray-900 dark:text-white">Error</h3>
					<div class="rounded-lg bg-red-50 p-4 font-mono text-xs text-red-600 dark:bg-red-950/30 dark:text-red-400">
						{selectedExecution.error}
					</div>
				</div>
			{/if}

			<!-- Task ID -->
			{#if selectedExecution.task_id}
				<div class="mb-4 rounded-lg border border-blue-200 bg-blue-50 p-3 dark:border-blue-800 dark:bg-blue-950/30">
					<div class="flex items-center justify-between">
						<div>
							<div class="text-xs font-medium text-blue-900 dark:text-blue-300">Celery Task ID</div>
							<div class="font-mono text-xs text-blue-700 dark:text-blue-400">{selectedExecution.task_id}</div>
						</div>
						<Button
							size="sm"
							variant="outline"
							onclick={async () => {
								if (selectedExecution?.task_id) {
									try {
										const taskStatus = await getTaskStatus(selectedExecution.task_id);
										console.log('Task status:', taskStatus);
										if (taskStatus.result) {
											alert(`Task Result:\nStatus: ${taskStatus.result.status}\nReturn Code: ${taskStatus.result.return_code}\nOutput:\n${taskStatus.result.output.substring(0, 500)}...`);
										}
									} catch (err) {
										console.error('Failed to get task status:', err);
										error = err instanceof Error ? err.message : 'Failed to get task status';
									}
								}
							}}
						>
							View Task Result
						</Button>
					</div>
				</div>
			{/if}

			<!-- Output -->
			{#if selectedExecution.output}
				<div class="mb-4">
					<h3 class="mb-2 text-sm font-medium text-gray-900 dark:text-white">Output</h3>
					<div class="max-h-96 overflow-y-auto rounded-lg bg-gray-900 p-4">
						<pre class="font-mono text-xs text-gray-100">{selectedExecution.output}</pre>
					</div>
				</div>
			{:else if selectedExecution.status === 'pending' || selectedExecution.status === 'running'}
				<div class="mb-4 flex items-center justify-center rounded-lg bg-gray-50 p-8 dark:bg-gray-900">
					<div class="text-center">
						<Loader2 class="mx-auto mb-2 h-8 w-8 animate-spin text-gray-400" />
						<p class="text-sm text-gray-500 dark:text-gray-400">
							{selectedExecution.status === 'pending' ? 'Waiting to start...' : 'Execution in progress...'}
						</p>
						{#if selectedExecution.task_id}
							<Button
								size="sm"
								variant="outline"
								class="mt-4"
								onclick={async () => {
									if (selectedExecution?.task_id) {
										try {
											const taskStatus = await getTaskStatus(selectedExecution.task_id);
											if (taskStatus.result) {
												selectedExecution = {
													...selectedExecution,
													output: taskStatus.result.output,
													status: taskStatus.result.status === 'completed' ? 'completed' : taskStatus.result.status === 'failed' ? 'failed' : selectedExecution.status
												};
											}
										} catch (err) {
											console.error('Failed to get task status:', err);
										}
									}
								}}
							>
								Check Task Status
							</Button>
						{/if}
					</div>
				</div>
			{/if}

			<div class="flex justify-end gap-2">
				{#if selectedExecution.status === 'running' || selectedExecution.status === 'pending'}
					<Button
						size="sm"
						variant="outline"
						onclick={async () => {
							const updated = await getExecutionResult(selectedExecution!.id);
							selectedExecution = updated;
							// 실행 목록도 업데이트
							if (selectedPolicy) {
								await loadExecutions(selectedPolicy);
							}
						}}
					>
						Refresh
					</Button>
				{/if}
				<Button onclick={() => (showExecutionDetailModal = false)}>Close</Button>
			</div>
		</div>
	</div>
{/if}
