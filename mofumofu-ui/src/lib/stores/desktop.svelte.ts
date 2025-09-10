// src/lib/stores/desktop.svelte.ts

import { browser } from '$app/environment';

// Tauri 환경인지 확인
const isDesktop = browser ? (window as any).__TAURI__ !== undefined : false;

// 사이드바 상태 관리
class DesktopStore {
	private _sidebarOpen = $state(true);
	private _isDesktop = $state(isDesktop);

	get sidebarOpen() {
		return this._sidebarOpen;
	}

	get isDesktop() {
		return this._isDesktop;
	}

	toggleSidebar() {
		this._sidebarOpen = !this._sidebarOpen;
		
		// localStorage에 상태 저장
		if (browser) {
			localStorage.setItem('desktop-sidebar-open', this._sidebarOpen.toString());
		}
	}

	setSidebarOpen(open: boolean) {
		this._sidebarOpen = open;
		
		if (browser) {
			localStorage.setItem('desktop-sidebar-open', open.toString());
		}
	}

	// 초기화 시 localStorage에서 상태 복원
	init() {
		if (browser) {
			const saved = localStorage.getItem('desktop-sidebar-open');
			if (saved !== null) {
				this._sidebarOpen = saved === 'true';
			}
		}
	}
}

export const desktopStore = new DesktopStore();

// 브라우저에서 초기화
if (browser) {
	desktopStore.init();
	
	// 키보드 단축키 (Ctrl/Cmd + B)
	document.addEventListener('keydown', (e) => {
		if ((e.ctrlKey || e.metaKey) && e.key === 'b' && desktopStore.isDesktop) {
			e.preventDefault();
			desktopStore.toggleSidebar();
		}
	});
}