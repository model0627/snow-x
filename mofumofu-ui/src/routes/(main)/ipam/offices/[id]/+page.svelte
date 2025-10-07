<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		Building,
		MapPin,
		Calendar,
		User,
		Phone,
		Mail,
		ArrowLeft,
		Edit,
		Trash2,
		Plus,
		Server,
		Monitor,
		Settings,
		ChevronRight
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { desktopStore } from '$lib/stores/desktop.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { officeApi, type Office } from '$lib/api/office';
	import OfficeFormDialog from '$lib/components/office/OfficeFormDialog.svelte';
	import { onMount } from 'svelte';

	// Get office ID from route params
	const officeId = $page.params.id;

	let office = $state<Office | null>(null);
	let loading = $state(true);
	let showEditDialog = $state(false);

	const isDesktop = $derived(desktopStore.isDesktop);

	// Load office details
	async function loadOffice() {
		loading = true;
		try {
			office = await officeApi.getOffice(officeId);
		} catch (error) {
			console.error('Failed to load office:', error);
			// Redirect back to list if office not found
			goto('/ipam/offices');
		} finally {
			loading = false;
		}
	}

	// Delete office
	async function deleteOffice() {
		if (office && confirm(`"${office.name}" 사무실을 삭제하시겠습니까?`)) {
			try {
				await officeApi.deleteOffice(office.id);
				goto('/ipam/offices');
			} catch (error) {
				console.error('Failed to delete office:', error);
				alert('사무실 삭제에 실패했습니다.');
			}
		}
	}

	// Open edit dialog
	function openEditDialog() {
		showEditDialog = true;
	}

	// Close dialog
	function closeDialog() {
		showEditDialog = false;
	}

	// Handle successful form submission
	function handleFormSuccess() {
		loadOffice(); // Reload office data
		closeDialog();
	}

	// Format date
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date
			.toLocaleDateString('ko-KR', {
				year: 'numeric',
				month: '2-digit',
				day: '2-digit'
			})
			.replace(/\. /g, '. ')
			.replace(/\.$/, '');
	}

	// Navigate to server management
	function navigateToServers() {
		goto(`/ipam/offices/${officeId}/servers`);
	}

	// Navigate to device management
	function navigateToDevices() {
		goto(`/ipam/offices/${officeId}/devices`);
	}

	onMount(() => {
		loadOffice();
	});
</script>

{#if loading}
	<div class="flex min-h-screen flex-1 items-center justify-center">
		<div class="text-gray-500 dark:text-gray-400">로딩 중...</div>
	</div>
{:else if office}
	<div class="min-h-screen flex-1 bg-gray-50 dark:bg-gray-900">
		<!-- Header with back button -->
		<div class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
			<div class="px-6 py-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-3">
						<button
							onclick={() => goto('/ipam/offices')}
							class="rounded-lg p-2 transition-colors hover:bg-gray-100 dark:hover:bg-gray-700"
							title="목록으로 돌아가기"
						>
							<ArrowLeft class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-gray-600 dark:text-gray-400" />
						</button>
						<Building class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-orange-500" />
						<div>
							<h1 class="{isDesktop ? 'text-base' : 'text-xl'} font-semibold text-gray-900 dark:text-white">
								{office.name}
							</h1>
							<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">사무실 상세 정보</p>
						</div>
					</div>
					<div class="flex items-center gap-2">
						<Button
							onclick={openEditDialog}
							variant="outline"
							size={isDesktop ? 'sm' : 'default'}
							class="border-gray-300 dark:border-gray-600"
						>
							<Edit class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							수정
						</Button>
						<Button
							onclick={deleteOffice}
							variant="outline"
							size={isDesktop ? 'sm' : 'default'}
							class="border-red-300 text-red-600 hover:bg-red-50 dark:border-red-600 dark:text-red-400 dark:hover:bg-red-950/20"
						>
							<Trash2 class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mr-1" />
							삭제
						</Button>
					</div>
				</div>
			</div>
		</div>

		<!-- Content -->
		<div class="p-6">
			<div class="mx-auto max-w-6xl space-y-6">
				<!-- Basic Information Card -->
				<div class="rounded-lg border border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
					<div class="p-6">
						<h2 class="{isDesktop ? 'text-sm' : 'text-base'} mb-4 font-semibold text-gray-900 dark:text-white">
							기본 정보
						</h2>

						<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
							<div class="space-y-4">
								<div>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">사무실명</p>
									<p class="{isDesktop ? 'text-sm' : 'text-base'} font-medium text-gray-900 dark:text-white">
										{office.name}
									</p>
								</div>

								<div>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">설명</p>
									<p class="{isDesktop ? 'text-sm' : 'text-base'} text-gray-700 dark:text-gray-300">
										{office.description || '설명 없음'}
									</p>
								</div>

								<div>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">주소</p>
									<div class="flex items-start gap-2">
										<MapPin class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} mt-0.5 text-gray-400" />
										<p class="{isDesktop ? 'text-sm' : 'text-base'} text-gray-700 dark:text-gray-300">
											{office.address}
										</p>
									</div>
								</div>
							</div>

							<div class="space-y-4">
								<div>
									<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">생성일</p>
									<div class="flex items-center gap-2">
										<Calendar class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
										<p class="{isDesktop ? 'text-sm' : 'text-base'} text-gray-700 dark:text-gray-300">
											{formatDate(office.created_at)}
										</p>
									</div>
								</div>

								{#if office.contact_person}
									<div>
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">담당자</p>
										<div class="flex items-center gap-2">
											<User class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
											<p class="{isDesktop ? 'text-sm' : 'text-base'} text-gray-700 dark:text-gray-300">
												{office.contact_person}
											</p>
										</div>
									</div>
								{/if}

								{#if office.phone}
									<div>
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">연락처</p>
										<div class="flex items-center gap-2">
											<Phone class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
											<p class="{isDesktop ? 'text-sm' : 'text-base'} text-gray-700 dark:text-gray-300">
												{office.phone}
											</p>
										</div>
									</div>
								{/if}

								{#if office.email}
									<div>
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} mb-1 text-gray-500 dark:text-gray-400">이메일</p>
										<div class="flex items-center gap-2">
											<Mail class="{isDesktop ? 'h-3 w-3' : 'h-4 w-4'} text-gray-400" />
											<a
												href="mailto:{office.email}"
												class="{isDesktop ? 'text-sm' : 'text-base'} text-blue-600 hover:underline dark:text-blue-400"
											>
												{office.email}
											</a>
										</div>
									</div>
								{/if}
							</div>
						</div>
					</div>
				</div>

				<!-- Stats Cards -->
				<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
					<!-- Server Stats -->
					<div class="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-800">
						<div class="mb-4 flex items-center justify-between">
							<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">서버실</h3>
							<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-500" />
						</div>
						<div class="flex items-end justify-between">
							<div>
								<p class="{isDesktop ? 'text-2xl' : 'text-3xl'} font-bold text-gray-900 dark:text-white">0</p>
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} mt-1 text-gray-500 dark:text-gray-400">0개의 서버실</p>
							</div>
						</div>
					</div>

					<!-- Device Stats -->
					<div class="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-800">
						<div class="mb-4 flex items-center justify-between">
							<h3 class="{isDesktop ? 'text-sm' : 'text-base'} font-semibold text-gray-900 dark:text-white">
								디바이스
							</h3>
							<Settings class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-green-500" />
						</div>
						<div class="flex items-end justify-between">
							<div>
								<p class="{isDesktop ? 'text-2xl' : 'text-3xl'} font-bold text-gray-900 dark:text-white">0</p>
								<p class="{isDesktop ? 'text-xs' : 'text-sm'} mt-1 text-gray-500 dark:text-gray-400">0개의 디바이스</p>
							</div>
						</div>
					</div>
				</div>

				<!-- Related Items Section -->
				<div class="rounded-lg border border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
					<div class="p-6">
						<h2 class="{isDesktop ? 'text-sm' : 'text-base'} mb-4 font-semibold text-gray-900 dark:text-white">
							관련 항목
						</h2>

						<div class="space-y-2">
							<!-- Server Room Link -->
							<button
								onclick={navigateToServers}
								class="flex w-full items-center justify-between rounded-lg p-4 transition-colors hover:bg-gray-50 dark:hover:bg-gray-700"
							>
								<div class="flex items-center gap-3">
									<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30">
										<Server class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-blue-600 dark:text-blue-400" />
									</div>
									<div class="text-left">
										<p class="{isDesktop ? 'text-sm' : 'text-base'} font-medium text-gray-900 dark:text-white">
											서버실
										</p>
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">0개의 서버실</p>
									</div>
								</div>
								<div class="flex items-center gap-2 text-gray-400">
									<span class={isDesktop ? 'text-xs' : 'text-sm'}>관리하기</span>
									<ChevronRight class={isDesktop ? 'h-3 w-3' : 'h-4 w-4'} />
								</div>
							</button>

							<!-- Device Link -->
							<button
								onclick={navigateToDevices}
								class="flex w-full items-center justify-between rounded-lg p-4 transition-colors hover:bg-gray-50 dark:hover:bg-gray-700"
							>
								<div class="flex items-center gap-3">
									<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900/30">
										<Settings class="{isDesktop ? 'h-4 w-4' : 'h-5 w-5'} text-green-600 dark:text-green-400" />
									</div>
									<div class="text-left">
										<p class="{isDesktop ? 'text-sm' : 'text-base'} font-medium text-gray-900 dark:text-white">
											디바이스
										</p>
										<p class="{isDesktop ? 'text-xs' : 'text-sm'} text-gray-500 dark:text-gray-400">0개의 디바이스</p>
									</div>
								</div>
								<div class="flex items-center gap-2 text-gray-400">
									<span class={isDesktop ? 'text-xs' : 'text-sm'}>관리하기</span>
									<ChevronRight class={isDesktop ? 'h-3 w-3' : 'h-4 w-4'} />
								</div>
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Edit Dialog -->
	<OfficeFormDialog open={showEditDialog} {office} onClose={closeDialog} onSuccess={handleFormSuccess} />
{/if}
