<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { AlertCircle, CheckCircle } from 'lucide-svelte';

	let {
		value = $bindable(''),
		onSave,
		onCancel,
		readOnly = false,
		title = 'YAML Editor'
	}: {
		value?: string;
		onSave?: (content: string) => void | Promise<void>;
		onCancel?: () => void;
		readOnly?: boolean;
		title?: string;
	} = $props();

	let name = $state('');
	let description = $state('');
	let content = $state(value);
	let validationError = $state<string | null>(null);
	let saving = $state(false);

	// Sync content with value prop
	$effect(() => {
		content = value;
	});

	async function handleSave() {
		if (!name.trim() && !readOnly) {
			validationError = 'Policy name is required';
			return;
		}

		if (!content.trim()) {
			validationError = 'YAML content is required';
			return;
		}

		// Basic YAML validation
		try {
			// Simple check for YAML structure
			if (!content.includes('policies:')) {
				validationError = 'YAML must contain a "policies:" section';
				return;
			}

			validationError = null;
			saving = true;

			if (onSave) {
				await onSave(content);
			}
		} catch (err) {
			validationError = err instanceof Error ? err.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}

	function handleCancel() {
		if (onCancel) {
			onCancel();
		}
	}

	// Auto-resize textarea
	let textarea: HTMLTextAreaElement;
	function autoResize() {
		if (textarea) {
			textarea.style.height = 'auto';
			textarea.style.height = textarea.scrollHeight + 'px';
		}
	}

	onMount(() => {
		autoResize();
	});
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="mb-4">
		<h2 class="text-xl font-semibold text-gray-900 dark:text-white">{title}</h2>
	</div>

	<!-- Form -->
	<div class="flex-1 space-y-4 overflow-y-auto">
		{#if !readOnly}
			<!-- Name Input -->
			<div>
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Policy Name *
				</label>
				<input
					type="text"
					bind:value={name}
					placeholder="e.g., ec2-stop-unused-instances"
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<!-- Description Input -->
			<div>
				<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
					Description
				</label>
				<input
					type="text"
					bind:value={description}
					placeholder="Brief description of this policy"
					class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
				/>
			</div>
		{/if}

		<!-- YAML Content -->
		<div>
			<label class="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
				YAML Content *
			</label>
			<textarea
				bind:this={textarea}
				bind:value={content}
				oninput={autoResize}
				readonly={readOnly}
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

		<!-- Validation Error -->
		{#if validationError}
			<div class="rounded-lg bg-red-50 p-3 dark:bg-red-950/30">
				<div class="flex items-center gap-2">
					<AlertCircle class="h-4 w-4 text-red-600 dark:text-red-400" />
					<p class="text-sm text-red-600 dark:text-red-400">{validationError}</p>
				</div>
			</div>
		{/if}

		<!-- Tips -->
		{#if !readOnly}
			<div class="rounded-lg bg-blue-50 p-3 dark:bg-blue-950/30">
				<div class="flex items-start gap-2">
					<AlertCircle class="mt-0.5 h-4 w-4 text-blue-600 dark:text-blue-400" />
					<div class="text-sm text-blue-600 dark:text-blue-400">
						<p class="font-medium">Tips:</p>
						<ul class="mt-1 list-inside list-disc space-y-1">
							<li>YAML은 들여쓰기에 민감합니다 (2칸 공백 권장)</li>
							<li>policies 섹션은 필수입니다</li>
							<li>Cloud Custodian 문법을 따라야 합니다</li>
						</ul>
					</div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Actions -->
	{#if !readOnly}
		<div class="mt-4 flex justify-end gap-2">
			<Button variant="outline" onclick={handleCancel}>Cancel</Button>
			<Button onclick={handleSave} disabled={saving}>
				{saving ? 'Saving...' : 'Save Policy'}
			</Button>
		</div>
	{/if}
</div>
