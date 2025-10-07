<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { Network } from '@lucide/svelte';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { ipRangeApi, type IpRange, type CreateIpRangeRequest } from '$lib/api/office';

	interface IpRangeFormData {
		name: string;
		network_address: string;
		cidr_mask: number;
		gateway: string;
		vlan_id: string;
		dns_servers: string;
		description: string;
	}

	interface Props {
		open: boolean;
		onClose: () => void;
		onSuccess: (newIpRange?: IpRange) => void;
		editData?: IpRange | null;
	}

	let { open, onClose, onSuccess, editData = null }: Props = $props();

	let loading = $state(false);
	let formData = $state<IpRangeFormData>({
		name: '',
		network_address: '',
		cidr_mask: 24,
		gateway: '',
		vlan_id: '',
		dns_servers: '',
		description: ''
	});

	const isDesktop = $derived(desktopStore.isDesktop);

	// Calculate available IPs based on CIDR
	const availableIps = $derived(() => {
		if (formData.cidr_mask >= 1 && formData.cidr_mask <= 30) {
			return Math.pow(2, 32 - formData.cidr_mask) - 2; // -2 for network and broadcast
		}
		return 0;
	});

	// Reset form when dialog opens/closes or edit data changes
	$effect(() => {
		if (open) {
			if (editData) {
				// Populate form with edit data
				formData = {
					name: editData.name,
					network_address: editData.network_address,
					cidr_mask: editData.subnet_mask,
					gateway: editData.gateway || '',
					vlan_id: editData.vlan_id?.toString() || '',
					dns_servers: editData.dns_servers?.join(', ') || '',
					description: editData.description || ''
				};
			} else {
				// Reset to empty form
				formData = {
					name: '',
					network_address: '',
					cidr_mask: 24,
					gateway: '',
					vlan_id: '',
					dns_servers: '',
					description: ''
				};
			}
		}
	});

	async function handleSubmit() {
		if (!formData.name.trim() || !formData.network_address.trim()) {
			alert('IP 대역명과 네트워크 주소는 필수입니다.');
			return;
		}

		// Basic IP address validation
		const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
		if (!ipRegex.test(formData.network_address)) {
			alert('올바른 네트워크 주소를 입력해주세요.');
			return;
		}

		if (formData.gateway && !ipRegex.test(formData.gateway)) {
			alert('올바른 게이트웨이 주소를 입력해주세요.');
			return;
		}

		loading = true;
		try {
			// Parse DNS servers
			const dnsServers = formData.dns_servers
				? formData.dns_servers
						.split(',')
						.map((s) => s.trim())
						.filter((s) => s)
				: undefined;

			const requestData: CreateIpRangeRequest = {
				name: formData.name,
				network_address: formData.network_address,
				subnet_mask: formData.cidr_mask,
				gateway: formData.gateway || undefined,
				vlan_id: formData.vlan_id ? parseInt(formData.vlan_id) : undefined,
				dns_servers: dnsServers,
				description: formData.description || undefined,
				ip_version: 4 // Default to IPv4
			};

			let result: IpRange;
			if (editData) {
				// Update existing IP range
				result = await ipRangeApi.updateIpRange(editData.id, requestData);
			} else {
				// Create new IP range
				result = await ipRangeApi.createIpRange(requestData);
			}

			onSuccess(result);
			onClose();
		} catch (error) {
			console.error('Failed to save IP range:', error);
			alert(editData ? 'IP 대역 수정에 실패했습니다.' : 'IP 대역 등록에 실패했습니다.');
		} finally {
			loading = false;
		}
	}

	// Auto-generate gateway suggestion
	function generateGatewaySuggestion() {
		if (formData.network_address && !formData.gateway) {
			const parts = formData.network_address.split('.');
			if (parts.length === 4) {
				parts[3] = '1'; // Common gateway convention
				formData.gateway = parts.join('.');
			}
		}
	}

	// Watch network address changes to suggest gateway
	$effect(() => {
		if (formData.network_address) {
			generateGatewaySuggestion();
		}
	});
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent
		class="w-full max-w-2xl border border-gray-200 bg-white shadow-xl dark:border-gray-700 dark:bg-gray-800"
	>
		<DialogHeader class="border-b border-gray-200 pb-4 dark:border-gray-700">
			<div class="flex items-center gap-3">
				<div class="rounded-lg bg-blue-100 p-2 dark:bg-blue-900">
					<Network class="h-5 w-5 text-blue-600 dark:text-blue-400" />
				</div>
				<DialogTitle class="{isDesktop ? 'text-lg' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
					{editData ? 'IP 대역 수정' : '새 IP 대역 추가'}
				</DialogTitle>
			</div>
		</DialogHeader>

		<form on:submit|preventDefault={handleSubmit} class="space-y-5 pt-4">
			<!-- IP 대역명 -->
			<div>
				<label
					for="name"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					IP 대역명 <span class="text-red-500">*</span>
				</label>
				<input
					id="name"
					type="text"
					bind:value={formData.name}
					required
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="예: 사무실 내부망, DMZ 구간"
				/>
			</div>

			<!-- 네트워크 주소와 CIDR -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
				<div class="md:col-span-2">
					<label
						for="network_address"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						네트워크 주소 <span class="text-red-500">*</span>
					</label>
					<input
						id="network_address"
						type="text"
						bind:value={formData.network_address}
						required
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 192.168.1.0"
					/>
				</div>

				<div>
					<label
						for="cidr_mask"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						서브넷 마스크 (CIDR) <span class="text-red-500">*</span>
					</label>
					<input
						id="cidr_mask"
						type="number"
						bind:value={formData.cidr_mask}
						required
						min="1"
						max="30"
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="24"
					/>
					{#if availableIps > 0}
						<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
							사용 가능한 IP: {availableIps.toLocaleString()}개
						</p>
					{/if}
				</div>
			</div>

			<!-- 게이트웨이와 VLAN ID -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label
						for="gateway"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						게이트웨이
					</label>
					<input
						id="gateway"
						type="text"
						bind:value={formData.gateway}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 192.168.1.1"
					/>
				</div>

				<div>
					<label
						for="vlan_id"
						class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
					>
						VLAN ID
					</label>
					<input
						id="vlan_id"
						type="text"
						bind:value={formData.vlan_id}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
							text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
							focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
						placeholder="예: 100"
					/>
				</div>
			</div>

			<!-- DNS 서버 -->
			<div>
				<label
					for="dns_servers"
					class="block {isDesktop ? 'text-sm' : 'text-base'} mb-2 font-medium text-gray-900 dark:text-white"
				>
					DNS 서버
				</label>
				<input
					id="dns_servers"
					type="text"
					bind:value={formData.dns_servers}
					class="w-full px-3 py-2.5 {isDesktop ? 'text-sm' : 'text-base'} rounded-lg border border-gray-300 bg-white
						text-gray-900 placeholder-gray-400 transition-colors focus:border-blue-500 focus:ring-2
						focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
					placeholder="예: 8.8.8.8, 8.8.4.4"
				/>
				<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">여러 DNS 서버는 쉼표로 구분하여 입력하세요</p>
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
					placeholder="IP 대역에 대한 설명을 입력하세요"
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
					disabled={loading || !formData.name.trim() || !formData.network_address.trim()}
					class="bg-blue-600 text-white hover:bg-blue-700 {isDesktop
						? 'px-4 py-2 text-sm'
						: 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? (editData ? '수정 중...' : '추가 중...') : editData ? '수정' : '추가'}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>
