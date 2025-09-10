// src/lib/stores/auth.svelte.ts

import { browser } from '$app/environment';
import { userStore } from './user.svelte';
import { refreshAccessToken, signOut } from '$lib/api/auth/authApi';

const initialToken = browser ? (localStorage.getItem('access_token') ?? '') : '';
let _token = $state(initialToken);

// 브라우저에서 초기화 시 토큰이 없으면 userStore도 clear
if (browser && !initialToken) {
	userStore.clear();
}

export const authStore = {
	get token() {
		return _token;
	},

	get isAuthenticated() {
		return _token !== '';
	},

	setToken(token: string) {
		_token = token;
		if (browser) {
			localStorage.setItem('access_token', token);
			if (token) {
				userStore.loadProfile();
			}
		}
	},

	clearToken() {
		_token = '';
		if (browser) {
			localStorage.removeItem('access_token');
			userStore.clear();
		}
	},

	async tryRefreshToken(): Promise<boolean> {
		// 토큰 자동 새로 고침 비활성화 - 사용자가 수동으로 로그인하도록
		return false;
	},

	async logout(): Promise<void> {
		try {
			// 서버에 로그아웃 요청 (쿠키 무효화)
			await signOut();
		} catch (error) {
			console.error('Logout API failed:', error);
			// API 실패해도 클라이언트 토큰은 정리
		} finally {
			// 클라이언트 토큰 정리
			this.clearToken();
		}
	}
};
