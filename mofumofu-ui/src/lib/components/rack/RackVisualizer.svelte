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

	interface Props {
		rackHeight: number; // 총 U 단위
		devices?: Device[];
		onSlotClick?: (position: number) => void;
		onDeviceClick?: (device: Device) => void;
		showLabels?: boolean;
	}

	let {
		rackHeight = 42,
		devices = [],
		onSlotClick,
		onDeviceClick,
		showLabels = true
	}: Props = $props();

	const isDesktop = $derived(desktopStore.isDesktop);

	// 각 U 슬롯의 사용 상태를 계산
	const slotUsage = $derived(() => {
		const usage = new Array(rackHeight).fill(null);
		devices.forEach(device => {
			for (let i = 0; i < device.height; i++) {
				const slotIndex = device.position - 1 + i;
				if (slotIndex >= 0 && slotIndex < rackHeight) {
					usage[slotIndex] = device;
				}
			}
		});
		return usage;
	});

	// 디바이스 타입별 색상
	const deviceColors = {
		server: '#3b82f6', // blue
		switch: '#10b981', // emerald
		router: '#f59e0b', // amber
		storage: '#8b5cf6', // violet
		firewall: '#ef4444', // red
		ups: '#6b7280', // gray
		default: '#6366f1' // indigo
	};

	function getDeviceColor(device: Device): string {
		if (device.color) return device.color;
		return deviceColors[device.type as keyof typeof deviceColors] || deviceColors.default;
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

<div class="rack-container bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4">
	{#if showLabels}
		<div class="flex items-center justify-between mb-4">
			<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
				랙 시각화 ({rackHeight}U)
			</h3>
			<div class="flex items-center gap-4 text-xs">
				<div class="flex items-center gap-1">
					<div class="w-3 h-3 bg-blue-500 rounded"></div>
					<span class="text-gray-600 dark:text-gray-400">사용중</span>
				</div>
				<div class="flex items-center gap-1">
					<div class="w-3 h-3 bg-gray-200 dark:bg-gray-600 rounded border"></div>
					<span class="text-gray-600 dark:text-gray-400">사용가능</span>
				</div>
			</div>
		</div>
	{/if}

	<div class="rack-visualization relative">
		<!-- 랙 프레임 -->
		<div class="rack-frame relative bg-gray-100 dark:bg-gray-700 border-2 border-gray-800 dark:border-gray-600 rounded-lg p-2">
			<!-- U 단위 눈금 -->
			<div class="flex">
				<!-- 왼쪽 눈금 -->
				<div class="w-8 flex flex-col-reverse text-xs text-gray-500 dark:text-gray-400">
					{#each Array(rackHeight) as _, i}
						<div class="h-5 flex items-center justify-center border-b border-gray-300 dark:border-gray-600 last:border-b-0">
							{i + 1}
						</div>
					{/each}
				</div>

				<!-- 랙 슬롯들 -->
				<div class="flex-1 relative">
					<div class="grid grid-cols-1 gap-0">
						{#each Array(rackHeight) as _, i}
							{@const position = rackHeight - i}
							{@const device = slotUsage[position - 1]}
							{@const isDeviceStart = device && device.position === position}
							{@const isOccupied = !!device}

							<div
								class="rack-slot h-5 border border-gray-300 dark:border-gray-600 relative cursor-pointer transition-colors
									{isOccupied ? 'cursor-default' : 'hover:bg-blue-50 dark:hover:bg-blue-900/20'}
									{isDeviceStart ? 'z-10' : ''}
								"
								style="{isDeviceStart ? `background-color: ${getDeviceColor(device)}; height: ${device.height * 1.25}rem;` : ''}"
								onclick={() => isOccupied ? (device && handleDeviceClick(device)) : handleSlotClick(position)}
								title="{device ? `${device.name} (${device.type})` : `U${position} - 사용 가능`}"
							>
								{#if isDeviceStart}
									<!-- 디바이스 표시 -->
									<div class="absolute inset-0 flex items-center justify-center text-white text-xs font-medium px-2">
										<div class="text-center">
											<div class="truncate">{device.name}</div>
											<div class="text-xs opacity-75">{device.type}</div>
										</div>
									</div>
								{:else if !isOccupied}
									<!-- 빈 슬롯 -->
									<div class="absolute inset-0 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700"></div>
								{/if}
							</div>
						{/each}
					</div>
				</div>

				<!-- 오른쪽 눈금 -->
				<div class="w-8 flex flex-col-reverse text-xs text-gray-500 dark:text-gray-400">
					{#each Array(rackHeight) as _, i}
						<div class="h-5 flex items-center justify-center border-b border-gray-300 dark:border-gray-600 last:border-b-0">
							{i + 1}
						</div>
					{/each}
				</div>
			</div>

			<!-- 1U 크기 참조 표시 -->
			<div class="absolute bottom-2 right-2 text-xs text-gray-500 dark:text-gray-400">
				1U = {((1.75 * 25.4).toFixed(1))}mm (1.75인치) 표준 랙 단위
			</div>
		</div>

		<!-- 디바이스 목록 -->
		{#if devices.length > 0}
			<div class="mt-4">
				<h4 class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
					설치된 디바이스
				</h4>
				<div class="space-y-1">
					{#each devices as device}
						<div
							class="flex items-center gap-2 p-2 rounded border border-gray-200 dark:border-gray-600 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
							onclick={() => handleDeviceClick(device)}
						>
							<div
								class="w-3 h-3 rounded"
								style="background-color: {getDeviceColor(device)}"
							></div>
							<div class="flex-1">
								<div class="{isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white">
									{device.name}
								</div>
								<div class="{isDesktop ? 'text-xs' : 'text-xs'} text-gray-500 dark:text-gray-400">
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
		min-height: 1.25rem; /* 5 * 0.25rem = 20px, 대략 1U */
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