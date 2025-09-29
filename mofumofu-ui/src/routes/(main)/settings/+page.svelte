<script lang="ts">
	import {
		User,
		ComputerDesktop,
		Bell,
		ShieldExclamation,
		CheckCircle,
		ArrowUturnLeft,
		Trash,
		ArrowDownTray,
		ArrowUpTray,
		Icon,
		CreditCard,
		PencilSquare,
		Link
	} from 'svelte-hero-icons';
	import { getContext, onMount } from 'svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { browser } from '$app/environment';
	import { getOAuthConnections } from '$lib/api/auth/authApi';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';
	import MobileSettingsLayout from '$lib/components/settings/layouts/MobileSettingsLayout.svelte';
	import DesktopSettingsLayout from '$lib/components/settings/layouts/DesktopSettingsLayout.svelte';
	import ImageCropModal from '$lib/components/modal/ImageCropModal.svelte';
	import * as m from '../../../paraglide/messages';

	let selectedSection = $state(authStore.isAuthenticated ? 'personal' : 'display');
	let saveSuccess = $state(false);
	let isAuthChecking = $state(true); // 인증 체크 중인지
	let authError = $state(false); // 인증 실패 상태

	// 이미지 크롭 관련 상태
	let showImageCrop = $state(false);
	let cropImageSrc = $state('');
	let cropAspectRatio = $state(1);
	let cropShape = $state<'rect' | 'round'>('round');
	let onCropComplete: ((data: any) => void) | null = null;

	// 모바일에서 accordion의 기본 열린 섹션
	let accordionValue = $state(authStore.isAuthenticated ? 'personal' : 'display');

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// Calculate the top position based on navbar state
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	// 인증 상태는 상위 레이아웃에서 처리하므로 제거

	const sections = [
		{
			id: 'personal',
			label: () => m.settings_personal_info(),
			icon: User,
			description: () => m.settings_personal_info_desc(),
			requiresAuth: true
		},
		{
			id: 'account',
			label: () => m.settings_account(),
			icon: CreditCard,
			description: () => m.settings_account_desc(),
			requiresAuth: true
		},
		{
			id: 'api',
			label: () => '외부 API 연결',
			icon: Link,
			description: () => '외부 서비스와의 API 연결을 관리합니다',
			requiresAuth: false
		},
		{
			id: 'display',
			label: () => m.settings_display(),
			icon: ComputerDesktop,
			description: () => m.settings_display_desc(),
			requiresAuth: false
		},
		{
			id: 'writing',
			label: () => m.settings_writing(),
			icon: PencilSquare,
			description: () => m.settings_writing_desc(),
			requiresAuth: true
		},
		{
			id: 'notifications',
			label: () => m.settings_notifications(),
			icon: Bell,
			description: () => m.settings_notifications_desc(),
			requiresAuth: true
		},
		{
			id: 'privacy',
			label: () => m.settings_privacy(),
			icon: ShieldExclamation,
			description: () => m.settings_privacy_desc(),
			requiresAuth: true
		}
	];

	// URL 해시에서 섹션 읽기, 없으면 기본값 사용
	const getInitialSection = () => {
		if (typeof window !== 'undefined') {
			const hash = window.location.hash.slice(1); // # 제거
			if (hash && sections.some((s) => s.id === hash)) {
				return hash;
			}
		}
		return authStore.isAuthenticated ? 'personal' : 'display';
	};

	onMount(async () => {
		// URL 해시에서 초기 섹션 설정
		const initialSection = getInitialSection();

		// 인증이 필요한 섹션인지 확인
		const selectedSectionData = sections.find(s => s.id === initialSection);
		const requiresAuth = selectedSectionData?.requiresAuth ?? false;

		// 인증 체크 (인증이 필요한 섹션에 접근하려는 경우만)
		try {
			if (requiresAuth && !authStore.isAuthenticated) {
				console.log('📝 Settings: Auth required for section', initialSection);
				// 인증이 필요한 섹션이지만 인증되지 않은 경우, display로 리다이렉트
				selectedSection = 'display';
				accordionValue = 'display';
				window.location.hash = 'display';
			} else {
				selectedSection = initialSection;
				accordionValue = initialSection;
			}

			// Initialize settings with default data
			settingsStore.initializeWithDefaults();
		} finally {
			isAuthChecking = false;
		}
	});

	// userStore.user가 로드되면 자동으로 settings 업데이트
	let userInitialized = $state(false);
	let oauthInitialized = $state(false);

	$effect(() => {
		if (authStore.isAuthenticated && userStore.user && !userInitialized) {
			settingsStore.updatePersonalSilent({
				handle: userStore.user.handle,
				name: userStore.user.name,
				bio: userStore.user.bio || '',
				location: userStore.user.location || '',
				website: userStore.user.website || '',
				profileImage: userStore.user.profile_image || null,
				bannerImage: userStore.user.banner_image || null
			});

			// OAuth 데이터도 함께 로드 (한 번만)
			if (!oauthInitialized) {
				loadOAuthData();
				oauthInitialized = true;
			}

			// API 호출이 성공하면 인증된 상태이므로 personal 섹션으로 변경 (단, 해시가 없는 경우만)
			if (!window.location.hash && selectedSection === 'display') {
				selectedSection = 'personal';
				window.location.hash = 'personal';
			}

			userInitialized = true;
		}
	});

	// OAuth 데이터 로드 함수
	async function loadOAuthData() {
		try {
			const response = await getOAuthConnections();
			// Silent 업데이트로 변경사항 감지를 피함
			settingsStore.updateAccountSilent({
				oauthConnections: response.connections,
				isOAuthOnly: response.is_oauth_only
			});
		} catch (error) {
			console.error('Failed to load OAuth connections:', error);
		}
	}

	async function handleSave() {
		const result = await settingsStore.saveChanges();
		if (result.success) {
			saveSuccess = true;
			// 3초 후 saveSuccess 상태를 자동으로 초기화
			setTimeout(() => {
				saveSuccess = false;
			}, 1500);
		} else {
			saveSuccess = false;
		}
	}

	// Reset save success when user makes new changes
	$effect(() => {
		if (settingsStore.hasChanges && saveSuccess) {
			saveSuccess = false;
		}
	});

	function handleSectionChange(sectionId: string) {
		selectedSection = sectionId;
	}

	function handleReset() {
		settingsStore.resetChanges();
		saveSuccess = false;
	}

	// 이미지 크롭 관련 함수들
	function openImageCrop(
		imageSrc: string,
		aspectRatio: number = 1,
		shape: 'rect' | 'round' = 'round',
		onComplete?: (data: any) => void
	) {
		cropImageSrc = imageSrc;
		cropAspectRatio = aspectRatio;
		cropShape = shape;
		onCropComplete = onComplete || null;
		showImageCrop = true;
	}

	function handleCropComplete(data: any) {
		if (onCropComplete) {
			onCropComplete(data);
		}
		showImageCrop = false;
	}

	function handleCropCancel() {
		showImageCrop = false;
	}
</script>

<svelte:head>
	<title>설정 - Mofumofu</title>
	<meta name="description" content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요." />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content="설정 - Mofumofu" />
	<meta
		property="og:description"
		content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="설정 - Mofumofu" />
	<meta
		name="twitter:description"
		content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요."
	/>
</svelte:head>

<LoadingOverlay isVisible={isAuthChecking} message="설정을 불러오는 중..." />

{#if !isAuthChecking}
	<!-- 데스크톱 레이아웃 -->
	<DesktopSettingsLayout
		{sections}
		{selectedSection}
		{topPosition}
		{handleSave}
		{saveSuccess}
		onSectionChange={handleSectionChange}
		{openImageCrop}
		{handleReset}
	/>

	<!-- 모바일 레이아웃 -->
	<MobileSettingsLayout {sections} {handleSave} {saveSuccess} {openImageCrop} {handleReset} />

	<!-- 전역 이미지 크롭 모달 -->
	<ImageCropModal
		bind:isOpen={showImageCrop}
		imageSrc={cropImageSrc}
		aspectRatio={cropAspectRatio}
		{cropShape}
		onCrop={handleCropComplete}
		onCancel={handleCropCancel}
	/>
{/if}
