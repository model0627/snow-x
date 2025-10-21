<svelte:options runes={false} />

<script lang="ts">
	type Device = {
		id: string;
		name: string;
		type: string;
		height: number;
		position: number;
		color?: string;
	};

	export let rackHeight = 42;
	export let devices: Device[] = [];
	export let showLabels = true;

	const slots: { u: number; device: Device | null }[] = [];
	const slotHeight = 18;
	const slotSpacing = 2;
	const svgPadding = 24;

	$: updateSlots();

	function updateSlots() {
		slots.length = 0;
		for (let idx = rackHeight; idx >= 1; idx--) {
			slots.push({ u: idx, device: null });
		}

		for (const device of devices ?? []) {
			const start = Math.max(1, Math.round(device.position ?? 0));
			const height = Math.max(1, Math.round(device.height ?? 1));
			for (let i = 0; i < height; i++) {
				const u = start + i;
				if (u < 1 || u > rackHeight) continue;
				const slotIndex = rackHeight - u;
				slots[slotIndex].device = device;
			}
		}
	}

	const colorMap = {
		server: '#3b82f6',
		switch: '#10b981',
		router: '#f59e0b',
		storage: '#8b5cf6',
		firewall: '#ef4444',
		ups: '#6b7280',
		default: '#6366f1'
	} as const;

	const slotColor = (device: Device | null) =>
		device?.color ?? colorMap[device?.type as keyof typeof colorMap] ?? colorMap.default;

	const svgHeight = svgPadding * 2 + slots.length * (slotHeight + slotSpacing);
	const svgWidth = 220;
</script>

<div class="rack-wrapper">
	{#if showLabels}
		<div class="header">
			<h3>랙 시각화 ({rackHeight}U)</h3>
			<div class="legend">
				<span class="chip used"></span>
				<span>사용중</span>
				<span class="chip free"></span>
				<span>사용가능</span>
			</div>
		</div>
	{/if}

	<svg class="rack-svg" viewBox={`0 0 ${svgWidth} ${svgHeight}`} role="img">
		<defs>
			<linearGradient id="slot-free" x1="0" y1="0" x2="1" y2="1">
				<stop offset="0%" stop-color="#e2e8f0" />
				<stop offset="100%" stop-color="#cbd5f5" />
			</linearGradient>
		</defs>

		<rect x="16" y="{svgPadding - 10}" width="188" height="{slots.length * (slotHeight + slotSpacing) + 12}" rx="12" fill="#f8fafc" stroke="#cbd5f5" />

		{#each slots as slot, index}
			{@const y = svgPadding + index * (slotHeight + slotSpacing)}
			<g>
				<text x="4" y={y + slotHeight / 1.4} class="u-label">{slot.u}</text>
				<rect
					x="28"
					y={y}
					width="164"
					height="{slotHeight}"
					rx="6"
					fill={slot.device ? slotColor(slot.device) : 'url(#slot-free)'}
					opacity={slot.device ? 0.92 : 1}
				/>
				{#if slot.device}
					<text x="32" y={y + slotHeight / 1.4} class="device-label">{slot.device.name}</text>
				{/if}
			</g>
		{/each}
	</svg>
</div>

<style>
	.rack-wrapper {
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 0.75rem;
		background: white;
		color: #0f172a;
		padding-bottom: 1rem;
	}

	:global(body.dark) .rack-wrapper {
		background: #1f2937;
		color: #e2e8f0;
		border-color: rgba(148, 163, 184, 0.4);
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem 0.5rem;
	}

	.header h3 {
		margin: 0;
		font-size: 0.95rem;
	}

	.legend {
		display: flex;
		align-items: center;
		gap: 0.65rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.legend .chip {
		display: inline-block;
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 0.25rem;
		border: 1px solid rgba(148, 163, 184, 0.4);
	}

	.legend .chip.used {
		background: rgba(59, 130, 246, 0.9);
	}

	.legend .chip.free {
		background: linear-gradient(90deg, rgba(226, 232, 240, 0.9), rgba(203, 213, 225, 0.4));
	}

	.rack-svg {
		width: 100%;
		display: block;
	}

	.u-label {
		font-size: 0.7rem;
		font-weight: 600;
		fill: rgba(71, 85, 105, 0.9);
	}

	.device-label {
		font-size: 0.75rem;
		font-weight: 600;
		fill: #fff;
	}

	:global(body.dark) .u-label {
		fill: rgba(203, 213, 225, 0.9);
	}

	:global(body.dark) .device-label {
		fill: #f8fafc;
	}
</style>
