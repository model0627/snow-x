<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Dialog, DialogContent, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
	import { officeApi, type Office, type CreateOfficeRequest, type UpdateOfficeRequest } from '$lib/api/office';
	import { desktopStore } from '$lib/stores/desktop.svelte';

	interface Props {
		open: boolean;
		office?: Office | null;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, office = null, onClose, onSuccess }: Props = $props();

	let loading = $state(false);
	let formData = $state({
		name: '',
		description: '',
		address: '',
		contact_person: '',
		phone: '',
		email: ''
	});

	const isDesktop = $derived(desktopStore.isDesktop);
	const isEdit = $derived(office !== null);
	const title = $derived(isEdit ? '사무실 수정' : '새 사무실 추가');

	// Reset form when dialog opens/closes or office changes
	$effect(() => {
		if (open) {
			if (office) {
				// Edit mode - populate form with office data
				formData = {
					name: office.name,
					description: office.description || '',
					address: office.address,
					contact_person: office.contact_person || '',
					phone: office.phone || '',
					email: office.email || ''
				};
			} else {
				// Create mode - reset form
				formData = {
					name: '',
					description: '',
					address: '',
					contact_person: '',
					phone: '',
					email: ''
				};
			}
		}
	});

	async function handleSubmit() {
		if (!formData.name.trim() || !formData.address.trim()) {
			alert('사무실명과 주소는 필수입니다.');
			return;
		}

		loading = true;
		try {
			if (isEdit && office) {
				const updateData: UpdateOfficeRequest = {};
				if (formData.name !== office.name) updateData.name = formData.name;
				if (formData.description !== (office.description || '')) updateData.description = formData.description || undefined;
				if (formData.address !== office.address) updateData.address = formData.address;
				if (formData.contact_person !== (office.contact_person || '')) updateData.contact_person = formData.contact_person || undefined;
				if (formData.phone !== (office.phone || '')) updateData.phone = formData.phone || undefined;
				if (formData.email !== (office.email || '')) updateData.email = formData.email || undefined;

				if (Object.keys(updateData).length > 0) {
					await officeApi.updateOffice(office.id, updateData);
				}
			} else {
				const createData: CreateOfficeRequest = {
					name: formData.name,
					address: formData.address,
					description: formData.description || undefined,
					contact_person: formData.contact_person || undefined,
					phone: formData.phone || undefined,
					email: formData.email || undefined
				};
				await officeApi.createOffice(createData);
			}

			onSuccess();
			onClose();
		} catch (error) {
			console.error('Failed to save office:', error);
			alert(`사무실 ${isEdit ? '수정' : '등록'}에 실패했습니다.`);
		} finally {
			loading = false;
		}
	}
</script>

<Dialog {open} onOpenChange={(value) => !value && onClose()}>
	<DialogContent class="max-w-lg w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-xl">
		<DialogHeader class="pb-4 border-b border-gray-200 dark:border-gray-700">
			<DialogTitle class="{isDesktop ? 'text-sm' : 'text-lg'} text-gray-900 dark:text-white font-semibold">{title}</DialogTitle>
			<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400 mt-1">
				사무실 정보를 {isEdit ? '수정' : '등록'}하세요
			</p>
		</DialogHeader>

		<form on:submit|preventDefault={handleSubmit} class="pt-4 space-y-5">
			<div>
				<label for="name" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
					사무실명 <span class="text-red-500">*</span>
				</label>
				<input
					id="name"
					type="text"
					bind:value={formData.name}
					required
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
						bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
						focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition-colors"
					placeholder="사무실명을 입력하세요"
				/>
			</div>

			<div>
				<label for="description" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
					설명
				</label>
				<textarea
					id="description"
					bind:value={formData.description}
					rows="3"
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
						bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
						focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 resize-none transition-colors"
					placeholder="사무실 설명을 입력하세요"
				></textarea>
			</div>

			<div>
				<label for="address" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
					주소 <span class="text-red-500">*</span>
				</label>
				<textarea
					id="address"
					bind:value={formData.address}
					required
					rows="2"
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
						bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
						focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 resize-none transition-colors"
					placeholder="사무실 주소를 입력하세요"
				></textarea>
			</div>

			<div>
				<label for="contact_person" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
					담당자
				</label>
				<input
					id="contact_person"
					type="text"
					bind:value={formData.contact_person}
					class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
						bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
						focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition-colors"
					placeholder="담당자명을 입력하세요"
				/>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div>
					<label for="phone" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
						전화번호
					</label>
					<input
						id="phone"
						type="tel"
						bind:value={formData.phone}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
							bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
							focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition-colors"
						placeholder="전화번호"
					/>
				</div>

				<div>
					<label for="email" class="block {isDesktop ? 'text-xs' : 'text-sm'} font-medium text-gray-900 dark:text-white mb-2">
						이메일
					</label>
					<input
						id="email"
						type="email"
						bind:value={formData.email}
						class="w-full px-3 py-2.5 {isDesktop ? 'text-xs' : 'text-sm'} border border-gray-300 dark:border-gray-600 rounded-lg
							bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400
							focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition-colors"
						placeholder="이메일 주소"
					/>
				</div>
			</div>

			<div class="flex justify-end space-x-3 pt-6 border-t border-gray-200 dark:border-gray-700">
				<Button
					type="button"
					variant="outline"
					onclick={onClose}
					disabled={loading}
					class="{isDesktop ? 'text-xs px-4 py-2' : 'px-6 py-2'} border-gray-300 dark:border-gray-600"
				>
					취소
				</Button>
				<Button
					type="submit"
					disabled={loading || !formData.name.trim() || !formData.address.trim()}
					class="bg-orange-500 hover:bg-orange-600 text-white {isDesktop ? 'text-xs px-4 py-2' : 'px-6 py-2'} disabled:opacity-50"
				>
					{loading ? '처리 중...' : isEdit ? '수정' : '등록'}
				</Button>
			</div>
		</form>
	</DialogContent>
</Dialog>