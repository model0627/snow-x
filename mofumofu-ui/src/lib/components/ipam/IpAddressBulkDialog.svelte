<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { Network } from '@lucide/svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { ipRangeApi, type IpRange } from '$lib/api/office';
	import Select from 'svelte-select';

	interface BulkFormData {
		ip_range_id: string;
		start_ip: string;
		end_ip: string;
		status: 'available' | 'reserved' | 'unavailable';
		description: string;
	}

	interface Props {
		open: boolean;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let ipRanges = $state<IpRange[]>([]);
	let selectedRange = $state<IpRange | null>(null);
	let selectedIpRange = $state<{value: string, label: string} | null>(null);
	let selectedStatus = $state<{value: string, label: string} | null>(null);

	let formData = $state<BulkFormData>({
		ip_range_id: '',
		start_ip: '',
		end_ip: '',
		status: 'available',
		description: ''
	});

	const isDesktop = $derived(desktopStore.isDesktop);

	const statusOptions = [
		{ value: 'available', label: '사용가능' },
		{ value: 'reserved', label: '예약됨' },
		{ value: 'unavailable', label: '사용불가' }
	];

	// Calculate IP count
	const ipCount = $derived.by(() => {
		if (!formData.start_ip || !formData.end_ip) return 0;

		try {
			const start = ipToNumber(formData.start_ip);
			const end = ipToNumber(formData.end_ip);

			if (start > end) return 0;
			return end - start + 1;
		} catch {
			return 0;
		}
	});

	// Load IP ranges when dialog opens
	$effect(() => {
		if (open) {
			loadIpRanges();
			// Reset form
			formData = {
				ip_range_id: '',
				start_ip: '',
				end_ip: '',
				status: 'available',
				description: ''
			};
			selectedRange = null;
			selectedIpRange = null;
			selectedStatus = statusOptions[0]; // default to 'available'
		}
	});

	async function loadIpRanges() {
		try {
			const response = await ipRangeApi.getIpRanges({ page: 1, limit: 100 });
			ipRanges = response.ip_ranges;
		} catch (error) {
			console.error('Failed to load IP ranges:', error);
		}
	}

	function ipToNumber(ip: string): number {
		const parts = ip.split('.').map(Number);
		return (parts[0] << 24) + (parts[1] << 16) + (parts[2] << 8) + parts[3];
	}

	function numberToIp(num: number): string {
		return [(num >>> 24) & 255, (num >>> 16) & 255, (num >>> 8) & 255, num & 255].join('.');
	}

	// Update formData and auto-fill IPs when IP range selection changes
	$effect(() => {
		if (selectedIpRange) {
			formData.ip_range_id = selectedIpRange.value;
			selectedRange = ipRanges.find((r) => r.id === selectedIpRange.value) || null;

			if (selectedRange) {
				// Auto-fill start and end IP based on network
				const networkBase = ipToNumber(selectedRange.network_address);
				const subnetMask = selectedRange.subnet_mask;
				const hostBits = 32 - subnetMask;
				const maxHosts = Math.pow(2, hostBits) - 2; // -2 for network and broadcast

				formData.start_ip = numberToIp(networkBase + 1);
				formData.end_ip = numberToIp(networkBase + maxHosts);
			}
		}
	});

	// Update status in formData when selection changes
	$effect(() => {
		if (selectedStatus) {
			formData.status = selectedStatus.value;
		}
	});

	async function handleSubmit() {
		if (!formData.ip_range_id || !formData.start_ip || !formData.end_ip) {
			alert('IP 대역, 시작 IP, 종료 IP는 필수입니다.');
			return;
		}

		// Validate IP format
		const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
		if (!ipRegex.test(formData.start_ip) || !ipRegex.test(formData.end_ip)) {
			alert('올바른 IP 주소 형식을 입력해주세요.');
			return;
		}

		// Validate range
		if (ipToNumber(formData.start_ip) > ipToNumber(formData.end_ip)) {
			alert('시작 IP가 종료 IP보다 클 수 없습니다.');
			return;
		}

		// Validate within network range
		if (selectedRange) {
			const networkBase = ipToNumber(selectedRange.network_address);
			const subnetMask = selectedRange.subnet_mask;
			const hostBits = 32 - subnetMask;
			const networkEnd = networkBase + Math.pow(2, hostBits) - 1;

			const startNum = ipToNumber(formData.start_ip);
			const endNum = ipToNumber(formData.end_ip);

			if (startNum < networkBase || endNum > networkEnd) {
				alert(`선택한 IP 대역 범위(${selectedRange.network_address}/${subnetMask})를 벗어났습니다.`);
				return;
			}
		}

		loading = true;
		try {
			const { ipAddressApi } = await import('$lib/api/office');

			await ipAddressApi.createBulkIpAddresses({
				ip_range_id: formData.ip_range_id,
				start_ip: formData.start_ip,
				end_ip: formData.end_ip,
				status: formData.status,
				description: formData.description || undefined
			});

			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to create IPs:', error);
			alert('IP 주소 생성에 실패했습니다.');
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="max-h-[90vh] w-full max-w-2xl overflow-y-auto border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader
			class="sticky top-0 z-10 border-b border-gray-200 bg-white pb-4 dark:border-gray-700 dark:bg-gray-800"
		>
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
					<Network class="h-5 w-5 text-blue-600 dark:text-blue-400" />
				</div>
				<DialogTitle class="{isDesktop ? 'text-lg' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
					범위로 IP 주소 추가
				</DialogTitle>
			</div>
		</DialogHeader>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				handleSubmit();
			}}
			class="space-y-5 pt-4"
		>
			<!-- IP 대역 선택 -->
			<div>
				<label
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					IP 대역 <span class="text-red-500">*</span>
				</label>
				<Select
					items={ipRanges.map(r => ({
						value: r.id,
						label: `${r.name} (${r.network_address}/${r.subnet_mask})`
					}))}
					bind:value={selectedIpRange}
					placeholder="IP 대역을 선택하세요"
					clearable={false}
					searchable={true}
					--border-radius="0.5rem"
					--border="1px solid rgb(209 213 219)"
					--border-focused="1px solid rgb(59 130 246)"
					--padding="0.625rem 0.75rem"
					--font-size={isDesktop ? '0.875rem' : '1rem'}
					--background="white"
					--list-background="white"
					--list-max-height="200px"
					--item-hover-bg="rgb(243 244 246)"
					--item-hover-color="rgb(17 24 39)"
					--item-is-active-bg="rgb(219 234 254)"
					--item-is-active-color="rgb(17 24 39)"
					--item-color="rgb(17 24 39)"
					--input-color="rgb(17 24 39)"
					--placeholder-color="rgb(156 163 175)"
					--selected-item-color="rgb(17 24 39)"
					--multi-item-color="rgb(17 24 39)"
				/>
			</div>

			{#if selectedRange}
				<div class="rounded-lg border border-blue-200 bg-blue-50 p-3 dark:border-blue-800 dark:bg-blue-900/20">
					<p class="text-sm text-blue-800 dark:text-blue-200">
						선택한 대역: {selectedRange.network_address}/{selectedRange.subnet_mask}
					</p>
					<p class="mt-1 text-xs text-blue-600 dark:text-blue-300">
						사용 가능한 IP: {Math.pow(2, 32 - selectedRange.subnet_mask) - 2}개
					</p>
				</div>
			{/if}

			<!-- IP 범위 -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label
						for="start_ip"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						시작 IP <span class="text-red-500">*</span>
					</label>
					<input
						id="start_ip"
						type="text"
						bind:value={formData.start_ip}
						required
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 192.168.1.10"
					/>
				</div>

				<div>
					<label
						for="end_ip"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						종료 IP <span class="text-red-500">*</span>
					</label>
					<input
						id="end_ip"
						type="text"
						bind:value={formData.end_ip}
						required
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 192.168.1.100"
					/>
				</div>
			</div>

			{#if ipCount > 0}
				<div class="rounded-lg border border-green-200 bg-green-50 p-3 dark:border-green-800 dark:bg-green-900/20">
					<p class="text-sm font-medium text-green-800 dark:text-green-200">
						생성될 IP 주소: {ipCount}개
					</p>
				</div>
			{/if}

			<!-- 상태 -->
			<div>
				<label
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					기본 상태 <span class="text-red-500">*</span>
				</label>
				<Select
					items={statusOptions}
					bind:value={selectedStatus}
					placeholder="상태 선택"
					clearable={false}
					searchable={false}
					--border-radius="0.5rem"
					--border="1px solid rgb(209 213 219)"
					--border-focused="1px solid rgb(59 130 246)"
					--padding="0.625rem 0.75rem"
					--font-size={isDesktop ? '0.875rem' : '1rem'}
					--background="white"
					--list-background="white"
					--list-max-height="200px"
					--item-hover-bg="rgb(243 244 246)"
					--item-hover-color="rgb(17 24 39)"
					--item-is-active-bg="rgb(219 234 254)"
					--item-is-active-color="rgb(17 24 39)"
					--item-color="rgb(17 24 39)"
					--input-color="rgb(17 24 39)"
					--placeholder-color="rgb(156 163 175)"
					--selected-item-color="rgb(17 24 39)"
					--multi-item-color="rgb(17 24 39)"
				/>
			</div>

			<!-- 설명 -->
			<div>
				<label
					for="description"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					설명
				</label>
				<textarea
					id="description"
					bind:value={formData.description}
					rows="3"
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} resize-none rounded-lg border border-gray-300
						bg-white text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500
						focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="IP 주소 범위에 대한 설명을 입력하세요"
				></textarea>
			</div>

			<!-- Buttons -->
			<div class="flex justify-end space-x-3 border-t border-gray-200 pt-6 dark:border-gray-700">
				<Button
					type="button"
					variant="outline"
					onclick={onClose}
					disabled={loading}
					class="{isDesktop ? 'px-4 py-2 text-sm' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
				>
					취소
				</Button>
				<Button
					type="submit"
					disabled={loading || !formData.ip_range_id || !formData.start_ip || !formData.end_ip || ipCount === 0}
					class="bg-blue-600 text-white hover:bg-blue-700 {isDesktop
						? 'px-4 py-2 text-sm'
						: 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? '추가 중...' : `${ipCount}개 IP 추가`}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>
