<svelte:options runes={false} />

<script lang="ts">
	import { desktopStore } from '$lib/stores/desktop.svelte';

	interface Device {
		id: string;
		name: string;
		type: string;
		height: number; // U 단위
		position: number; // 시작 위치 (1부터 시작)
		color?: string;
	}

	export let rackHeight: number = 42;
	export let devices: Device[] = [];
	export let onSlotClick: ((position: number) => void) | null | undefined;
	export let onDeviceClick: ((device: Device) => void) | null | undefined;
	export let showLabels = true;

	let slotUsage: (Device | null)[] = [];

	$: {
		const usage: (Device | null)[] = Array(rackHeight).fill(null);
		for (const device of devices ?? []) {
			for (let i = 0; i < device.height; i++) {
				const slotIndex = device.position - 1 + i;
				if (slotIndex >= 0 && slotIndex < rackHeight) {
					usage[slotIndex] = device;
				}
			}
		}
		slotUsage = usage;
	}

	const deviceColors = {
		server: '#3b82f6',
		switch: '#10b981',
		router: '#f59e0b',
		storage: '#8b5cf6',
		firewall: '#ef4444',
		ups: '#6b7280',
		default: '#6366f1'
	} as const;

	function getDeviceColor(device: Device): string {
		if (device.color) return device.color;
		return deviceColors[device.type as keyof typeof deviceColors] ?? deviceColors.default;
	}

	function handleSlotClick(position: number) {
		if (onSlotClick && !slotUsage[position - 1]) {
			onSlotClick(position);
		}
	}

	function handleDeviceClick(device: Device) {
		if (onDeviceClick) {
			onDeviceClick(device);
		}
	}
</script>

<div class="rack-container rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
	{#if showLabels}
		<div class="mb-4 flex items-center justify-between">
			<h3 class="{$desktopStore ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
				랙 시각화 ({rackHeight}U)
			</h3>
			<div class="flex items-center gap-4 text-xs">
				<div class="flex items-center gap-1">
					<div class="h-3 w-3 rounded bg-blue-500"></div>
					<span class="text-gray-600 dark:text-gray-400">사용중</span>
				</div>
				<div class="flex items-center gap-1">
					<div class="h-3 w-3 rounded border bg-gray-200 dark:bg-gray-600"></div>
					<span class="text-gray-600 dark:text-gray-400">사용가능</span>
				</div>
			</div>
		</div>
	{/if}

	<div class="rack-visualization relative">
		<div class="rack-frame relative rounded-lg border-2 border-gray-800 bg-gray-100 p-2 dark:border-gray-600 dark:bg-gray-700">
			<div class="flex">
				<div class="flex w-8 flex-col-reverse text-xs text-gray-500 dark:text-gray-400">
					{#each Array(rackHeight) as _, i}
						<div class="flex h-5 items-center justify-center border-b border-gray-300 last:border-b-0 dark:border-gray-600">
							{i + 1}
						</div>
					{/each}
				</div>

				<div class="relative flex-1">
					<div class="grid grid-cols-1 gap-0">
						{#each Array(rackHeight) as _, i}
							{@const position = rackHeight - i}
							{@const device = slotUsage[position - 1]}
							{@const isDeviceStart = device && device.position === position}
							{@const isOccupied = !!device}

							<div
								class="rack-slot relative h-5 cursor-pointer border border-gray-300 transition-colors dark:border-gray-600 {isOccupied ? 'cursor-default' : 'hover:bg-blue-50 dark:hover:bg-blue-900/20'} {isDeviceStart ? 'z-10' : ''}"
								style={isDeviceStart ? `background-color: ${getDeviceColor(device)}; height: ${device.height * 1.25}rem;` : ''}
								onclick={() => (isOccupied ? device && handleDeviceClick(device) : handleSlotClick(position))}
								title={device ? `${device.name} (${device.type})` : `U${position} - 사용 가능`}
							>
								{#if isDeviceStart}
									<div class="absolute inset-0 flex items-center justify-center px-2 text-xs font-medium text-white">
										<div class="text-center">
											<div class="truncate">{device.name}</div>
											<div class="text-xs opacity-75">{device.type}</div>
										</div>
									</div>
								{:else if !isOccupied}
									<div class="absolute inset-0 border border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800"></div>
								{/if}
							</div>
						{/each}
					</div>
				</div>

				<div class="flex w-8 flex-col-reverse text-xs text-gray-500 dark:text-gray-400">
					{#each Array(rackHeight) as _, i}
						<div class="flex h-5 items-center justify-center border-b border-gray-300 last:border-b-0 dark:border-gray-600">
							{i + 1}
						</div>
					{/each}
				</div>
			</div>

			<div class="absolute right-2 bottom-2 text-xs text-gray-500 dark:text-gray-400">
				1U = {(1.75 * 25.4).toFixed(1)}mm (1.75인치) 표준 랙 단위
			</div>
		</div>

		{#if devices.length > 0}
			<div class="mt-4">
				<h4 class="{$desktopStore ? 'text-xs' : 'text-sm'} mb-2 font-medium text-gray-900 dark:text-white">설치된 디바이스</h4>
				<div class="space-y-1">
					{#each devices as device}
						<div
							class="flex cursor-pointer items-center gap-2 rounded border border-gray-200 p-2 hover:bg-gray-50 dark:border-gray-600 dark:hover:bg-gray-700"
							onclick={() => handleDeviceClick(device)}
						>
							<div class="h-3 w-3 rounded" style="background-color: {getDeviceColor(device)}"></div>
							<div class="flex-1">
								<div class="{$desktopStore ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white">
									{device.name}
								</div>
								<div class="{$desktopStore ? 'text-xs' : 'text-xs'} text-gray-500 dark:text-gray-400">
									{device.type} • U{device.position}-{device.position + device.height - 1} ({device.height}U)
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.rack-slot {
		min-height: 1.25rem;
	}

	.rack-frame {
		background: linear-gradient(145deg, #f3f4f6, #e5e7eb);
	}

	:global(.dark) .rack-frame {
		background: linear-gradient(145deg, #374151, #4b5563);
	}

	.rack-slot:hover:not(.occupied) {
		box-shadow: inset 0 0 0 1px rgb(59 130 246);
	}
</style>
