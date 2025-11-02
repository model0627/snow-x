<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import LabelChip from './LabelChip.svelte';

	export let deviceId: string | undefined;
	export let name: string;
	export let labels: string[] = [];
	export let rowStart: number;
	export let size: number;
	export let start: number;
	export let column: 'front' | 'rear' = 'front';

	const dispatch = createEventDispatcher<{
		select: {
			deviceId: string | undefined;
			column: 'front' | 'rear';
			start: number;
			size: number;
			name: string;
		};
	}>();

	const PANEL_BACKGROUNDS: Record<string, string> = {
		A: 'linear-gradient(135deg, #dcfce7, #bbf7d0)',
		P: 'linear-gradient(135deg, #dbeafe, #bfdbfe)',
		S: 'linear-gradient(135deg, #dbeafe, #bfdbfe)',
		F: 'linear-gradient(135deg, #fee2e2, #fecaca)',
		I: 'linear-gradient(135deg, #e2e8f0, #cbd5f5)',
		default: 'linear-gradient(135deg, #f1f5f9, #e2e8f0)'
	};

	const PANEL_COLORS: Record<string, string> = {
		A: '#166534',
		P: '#1e3a8a',
		S: '#1e3a8a',
		F: '#991b1b',
		I: '#0f172a',
		default: '#0f172a'
	};

	$: mainLabel = labels[0] ?? 'default';
	$: background = PANEL_BACKGROUNDS[mainLabel] ?? PANEL_BACKGROUNDS.default;
	$: textColor = PANEL_COLORS[mainLabel] ?? PANEL_COLORS.default;
	$: gridColumn = column === 'front' ? 1 : 2;

	function handleActivate() {
		dispatch('select', { deviceId, column, start, size, name });
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleActivate();
		}
	}
</script>

<div
	role="button"
	tabindex="0"
	class="device"
	style={`--span-size:${size}; grid-row:${rowStart} / span ${size}; grid-column:${gridColumn}; background:${background}; color:${textColor};`}
	aria-label={`${name} (${size}U)`}
	on:click={handleActivate}
	on:keydown={handleKeydown}
>
	<span class="device-name">{name}</span>
	{#if labels.length}
		<div class="label-row">
			{#each labels as label}
				<LabelChip {label} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.device {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 6px 10px;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(15, 23, 42, 0.12);
		border: 1px solid rgba(15, 23, 42, 0.08);
		transition: transform 120ms ease;
		min-height: calc(20px * var(--span-size, 1) + 6px * (var(--span-size, 1) - 1));
	}

	.device:hover {
		transform: translateY(-1px);
	}

	.device:focus-visible {
		outline: 2px solid rgba(79, 70, 229, 0.65);
		outline-offset: 2px;
	}

	.device-name {
		font-size: 12px;
		font-weight: 600;
		line-height: 1.25;
	}

	.label-row {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
		margin-top: 6px;
	}
</style>
