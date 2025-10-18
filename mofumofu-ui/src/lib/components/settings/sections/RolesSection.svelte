<!-- src/lib/components/settings/sections/RolesSection.svelte -->
<script lang="ts">
	import { ROLE_PERMISSIONS, ROLE_DISPLAY_NAMES, type UserRole } from '$lib/config/roles';
	import { Settings, Shield, User } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';

	// 현재는 프론트엔드에서만 관리, 나중에 백엔드 API로 이동
	let roleConfigs = $state(
		Object.entries(ROLE_PERMISSIONS).map(([role, config]) => ({
			role: role as UserRole,
			displayName: ROLE_DISPLAY_NAMES[role as UserRole],
			allowedMenus: config.allowedMenus,
			isEditing: false,
			tempMenus: [...config.allowedMenus]
		}))
	);

	// 사용 가능한 메뉴 목록
	const availableMenus = [
		{ id: 'dashboard', name: '메인 대시보드' },
		{ id: 'ipam', name: 'IPAM 관리' },
		{ id: 'custodian', name: 'Custodian' },
		{ id: 'settings', name: '환경 설정' },
		{ id: 'relationships', name: '데이터 관계' },
		{ id: 'contacts', name: '연락처' }
	];

	function toggleMenu(roleIndex: number, menuId: string) {
		const role = roleConfigs[roleIndex];
		if (role.tempMenus.includes('*')) return; // Admin은 모든 권한 고정

		const index = role.tempMenus.indexOf(menuId);
		if (index > -1) {
			role.tempMenus = role.tempMenus.filter((m) => m !== menuId);
		} else {
			role.tempMenus = [...role.tempMenus, menuId];
		}
	}

	function startEdit(roleIndex: number) {
		roleConfigs[roleIndex].isEditing = true;
		roleConfigs[roleIndex].tempMenus = [...roleConfigs[roleIndex].allowedMenus];
	}

	function cancelEdit(roleIndex: number) {
		roleConfigs[roleIndex].isEditing = false;
		roleConfigs[roleIndex].tempMenus = [...roleConfigs[roleIndex].allowedMenus];
	}

	function saveEdit(roleIndex: number) {
		const role = roleConfigs[roleIndex];
		role.allowedMenus = [...role.tempMenus];
		role.isEditing = false;

		// TODO: 백엔드 API 호출하여 저장
		console.log('Role permissions updated:', {
			role: role.role,
			allowedMenus: role.allowedMenus
		});

		// 임시: 로컬 스토리지에 저장 (나중에 제거)
		localStorage.setItem('rolePermissions', JSON.stringify(roleConfigs));
	}
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-lg font-semibold text-gray-900 dark:text-white">역할 관리</h2>
		<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
			각 역할별로 접근 가능한 메뉴를 설정합니다. 변경사항은 즉시 적용됩니다.
		</p>
	</div>

	{#each roleConfigs as roleConfig, i}
		<div class="rounded-lg border border-gray-200 p-6 dark:border-gray-700">
			<div class="flex items-start justify-between">
				<div class="flex items-center gap-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-full {roleConfig.role === 'Admin'
							? 'bg-orange-100 dark:bg-orange-950/30'
							: roleConfig.role === 'Manager'
								? 'bg-blue-100 dark:bg-blue-950/30'
								: 'bg-gray-100 dark:bg-gray-800'}"
					>
						{#if roleConfig.role === 'Admin'}
							<Shield class="h-5 w-5 text-orange-600 dark:text-orange-400" />
						{:else if roleConfig.role === 'Manager'}
							<Settings class="h-5 w-5 text-blue-600 dark:text-blue-400" />
						{:else}
							<User class="h-5 w-5 text-gray-600 dark:text-gray-400" />
						{/if}
					</div>
					<div>
						<h3 class="text-base font-semibold text-gray-900 dark:text-white">
							{roleConfig.displayName}
						</h3>
						<p class="text-sm text-gray-500 dark:text-gray-400">
							{roleConfig.role}
						</p>
					</div>
				</div>

				{#if roleConfig.role !== 'Admin' && !roleConfig.isEditing}
					<Button
						variant="outline"
						size="sm"
						onclick={() => startEdit(i)}
						class="text-sm"
					>
						권한 수정
					</Button>
				{:else if roleConfig.isEditing}
					<div class="flex gap-2">
						<Button
							variant="outline"
							size="sm"
							onclick={() => cancelEdit(i)}
							class="text-sm"
						>
							취소
						</Button>
						<Button
							size="sm"
							onclick={() => saveEdit(i)}
							class="bg-orange-500 text-sm text-white hover:bg-orange-600"
						>
							저장
						</Button>
					</div>
				{/if}
			</div>

			<div class="mt-4">
				<h4 class="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">접근 가능한 메뉴</h4>

				{#if roleConfig.allowedMenus.includes('*')}
					<div class="rounded-md bg-orange-50 p-3 dark:bg-orange-950/20">
						<p class="text-sm font-medium text-orange-700 dark:text-orange-400">
							모든 메뉴에 접근 가능 (관리자 권한)
						</p>
					</div>
				{:else}
					<div class="space-y-2">
						{#each availableMenus as menu}
							<label class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-gray-50 dark:hover:bg-gray-800">
								<input
									type="checkbox"
									checked={roleConfig.isEditing
										? roleConfig.tempMenus.includes(menu.id)
										: roleConfig.allowedMenus.includes(menu.id)}
									onchange={() => toggleMenu(i, menu.id)}
									disabled={!roleConfig.isEditing}
									class="h-4 w-4 rounded border-gray-300 text-orange-500 focus:ring-orange-500 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:bg-gray-700"
								/>
								<span class="text-sm text-gray-700 dark:text-gray-300">{menu.name}</span>
							</label>
						{/each}
					</div>
				{/if}
			</div>

			{#if roleConfig.isEditing}
				<div class="mt-4 rounded-md bg-blue-50 p-3 dark:bg-blue-950/20">
					<p class="text-sm text-blue-700 dark:text-blue-400">
						💡 변경사항은 저장 버튼을 누른 후 적용됩니다.
					</p>
				</div>
			{/if}
		</div>
	{/each}

	<div class="rounded-lg border border-amber-200 bg-amber-50 p-4 dark:border-amber-900/50 dark:bg-amber-950/20">
		<h4 class="flex items-center gap-2 text-sm font-semibold text-amber-800 dark:text-amber-400">
			<Shield class="h-4 w-4" />
			개발 중인 기능
		</h4>
		<p class="mt-1 text-sm text-amber-700 dark:text-amber-500">
			현재 역할 권한 설정은 프론트엔드에서만 관리됩니다. 백엔드 API 연동 후 실제 사용자에게 역할을 할당하고 데이터베이스에 저장할 수 있습니다.
		</p>
	</div>
</div>
