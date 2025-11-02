<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import RackDevice from './RackDevice.svelte';

	export type RackViewDevice = {
		id: string;
		name: string;
		start: number;
		size: number;
		frontLabels?: string[];
		rearLabels?: string[];
	};

	export let rackHeight = 48;
	export let devices: RackViewDevice[] = [];

	const DEFAULT_HEIGHT = 48;
	const dispatch = createEventDispatcher<{
		'slot-select': { unit: number; column: 'front' | 'rear' };
		'device-select': {
			device: RackViewDevice;
			column: 'front' | 'rear';
		};
	}>();

	let totalUnits = DEFAULT_HEIGHT;
	let units: number[] = [];
	let frontDevices: Array<{
		original: RackViewDevice;
		id: string;
		name: string;
		labels: string[];
		rowStart: number;
		size: number;
		start: number;
	}> = [];
	let rearDevices: Array<{
		original: RackViewDevice;
		id: string;
		name: string;
		labels: string[];
		rowStart: number;
		size: number;
		start: number;
	}> = [];
	let occupiedUnits = new Set<number>();

	const clampSize = (start: number, size: number) => {
		const maxSpan = totalUnits - start + 1;
		return Math.max(1, Math.min(size, maxSpan));
	};

	const toGridRow = (start: number) => totalUnits - start + 1;

	$: {
		const maxDeviceExtent = devices.reduce((max, device) => {
			const start = Number(device.start ?? 0);
			const size = Number(device.size ?? 1);
			const end = Number.isFinite(start) && Number.isFinite(size) ? start + size - 1 : 0;
			return Math.max(max, end);
		}, 0);

		totalUnits = Math.max(
			DEFAULT_HEIGHT,
			rackHeight && rackHeight > 0 ? rackHeight : DEFAULT_HEIGHT,
			maxDeviceExtent
		);
		units = Array.from({ length: totalUnits }, (_, idx) => totalUnits - idx);

		occupiedUnits = new Set();

		frontDevices = devices.map((device, index) => {
			const rawStart = Number(device.start ?? 1);
			const start = Math.max(1, Math.min(totalUnits, Math.round(rawStart)));
			const size = clampSize(start, Math.round(device.size ?? 1));
			for (let offset = 0; offset < size; offset += 1) {
				occupiedUnits.add(start + offset);
			}
			return {
				original: device,
				id: device.id ?? `front-${index}`,
				name: device.name,
				labels: device.frontLabels ?? [],
				rowStart: toGridRow(start),
				size,
				start
			};
		});

		rearDevices = devices.map((device, index) => {
			const rawStart = Number(device.start ?? 1);
			const start = Math.max(1, Math.min(totalUnits, Math.round(rawStart)));
			const size = clampSize(start, Math.round(device.size ?? 1));
			return {
				original: device,
				id: `${device.id ?? `rear-${index}`}-rear`,
				name: device.name,
				labels: device.rearLabels ?? device.frontLabels ?? [],
				rowStart: toGridRow(start),
				size,
				start
			};
		});
	}

	function handleSlotClick(unit: number, column: 'front' | 'rear') {
		if (occupiedUnits.has(unit)) {
			return;
		}
		dispatch('slot-select', { unit, column });
	}

	function handleDeviceSelect(device: RackViewDevice, column: 'front' | 'rear') {
		dispatch('device-select', { device, column });
	}
</script>

<div class="rack-wrapper">
	<div class="rack-header">
		<span class="front-label">Front</span>
		<span class="rear-label">Rear</span>
	</div>

	<div class="rack-grid" style={`--units:${totalUnits}`}>
		{#each units as unit}
			{@const gridRow = toGridRow(unit)}
			<button
				type="button"
				class="slot front"
				class:occupied={occupiedUnits.has(unit)}
				style={`grid-row:${gridRow}; grid-column:1;`}
				data-ru={`U${unit}`}
				on:click={() => handleSlotClick(unit, 'front')}
				aria-label={`Front slot U${unit}${occupiedUnits.has(unit) ? ' (occupied)' : ''}`}
				disabled={occupiedUnits.has(unit)}
			></button>
			<button
				type="button"
				class="slot rear"
				class:occupied={occupiedUnits.has(unit)}
				style={`grid-row:${gridRow}; grid-column:2;`}
				data-ru={`U${unit}`}
				on:click={() => handleSlotClick(unit, 'rear')}
				aria-label={`Rear slot U${unit}${occupiedUnits.has(unit) ? ' (occupied)' : ''}`}
				disabled={occupiedUnits.has(unit)}
			></button>
		{/each}

		{#each frontDevices as device (device.id)}
			<RackDevice
				deviceId={device.original.id}
				name={device.name}
				labels={device.labels}
				rowStart={device.rowStart}
				size={device.size}
				column="front"
				start={device.start}
				on:select={() => handleDeviceSelect(device.original, 'front')}
			/>
		{/each}

		{#each rearDevices as device (device.id)}
			<RackDevice
				deviceId={device.original.id}
				name={device.name}
				labels={device.labels}
				rowStart={device.rowStart}
				size={device.size}
				column="rear"
				start={device.start}
				on:select={() => handleDeviceSelect(device.original, 'rear')}
			/>
		{/each}
	</div>
</div>

<style>
	.rack-wrapper {
		width: 100%;
		max-width: 480px;
		margin: 0 auto;
		padding: 20px 24px 24px;
		background: linear-gradient(180deg, rgba(248, 250, 252, 0.95), rgba(241, 245, 249, 0.85));
		border: 1px solid rgba(148, 163, 184, 0.35);
		border-radius: 18px;
		box-shadow: 0 10px 30px rgba(15, 23, 42, 0.12);
	}

	.rack-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 14px;
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: #475569;
	}

	.front-label {
		color: #0369a1;
	}

	.rear-label {
		color: #0f766e;
	}

	.rack-grid {
		position: relative;
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		grid-template-rows: repeat(var(--units), 20px);
		column-gap: 12px;
		row-gap: 6px;
		padding: 0 48px;
	}

	.slot {
		cursor: pointer;
		position: relative;
		border: 1px dashed rgba(148, 163, 184, 0.45);
		border-radius: 4px;
		background: rgba(248, 250, 252, 0.6);
		padding: 0;
		outline: none;
		transition: background 120ms ease, border-color 120ms ease, opacity 120ms ease;
	}

	.slot:disabled {
		cursor: default;
		opacity: 0.65;
	}

	.slot:not(:disabled):hover,
	.slot:not(:disabled):focus-visible {
		background: rgba(129, 140, 248, 0.18);
		border-color: rgba(79, 70, 229, 0.6);
	}

	.slot::before {
		content: attr(data-ru);
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		font-size: 10px;
		font-weight: 600;
		color: #94a3b8;
		letter-spacing: 0.04em;
	}

	.slot.front::before {
		left: -40px;
	}

	.slot.rear::before {
		right: -40px;
		text-align: right;
	}

	.slot.occupied::after {
		content: '';
		position: absolute;
		inset: 2px;
		border-radius: 4px;
		background: rgba(30, 41, 59, 0.08);
	}

	@media (max-width: 520px) {
		.rack-wrapper {
			padding: 16px 12px 20px;
		}
		.rack-grid {
			padding: 0 36px;
			column-gap: 8px;
		}
		.slot.front::before {
			left: -32px;
		}
		.slot.rear::before {
			right: -32px;
		}
	}
</style>
