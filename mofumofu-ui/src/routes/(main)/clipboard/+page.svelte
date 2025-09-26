<!-- 클립보드 페이지 -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Clipboard, Copy, Trash2, Plus, FileText, Link, Image } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Badge } from '$lib/components/ui/badge';
	import { browser } from '$app/environment';

	// 클립보드 데이터 - localStorage에서 로드
	let clipboardItems = $state([]);
	let newItemContent = $state('');
	let newItemTitle = $state('');
	let isTauri = $state(false);

	onMount(async () => {
		console.log('🔍 Clipboard page mounted');

		// localStorage에서 클립보드 아이템 로드
		if (browser) {
			try {
				const savedItems = localStorage.getItem('clipboardItems');
				if (savedItems) {
					clipboardItems = JSON.parse(savedItems);
					console.log('📋 Loaded clipboard items from localStorage:', clipboardItems.length);
				}
			} catch (error) {
				console.error('❌ Failed to load clipboard items:', error);
			}
		}

		// Tauri 환경 감지
		if (browser && (window as any).__TAURI__) {
			isTauri = true;
			console.log('🖥️ Running in Tauri desktop app - global monitoring active');
		} else {
			console.log('🌐 Running in web browser');
		}

		// 실시간 업데이트를 위한 storage 이벤트 리스너
		const handleStorageChange = (e: StorageEvent) => {
			if (e.key === 'clipboardItems' && e.newValue) {
				try {
					clipboardItems = JSON.parse(e.newValue);
					console.log('📋 Clipboard items updated from storage');
				} catch (error) {
					console.error('❌ Failed to parse storage update:', error);
				}
			}
		};

		// localStorage 변경사항을 실시간으로 감지
		const checkLocalStorage = () => {
			try {
				const savedItems = localStorage.getItem('clipboardItems');
				if (savedItems) {
					const newItems = JSON.parse(savedItems);
					if (JSON.stringify(newItems) !== JSON.stringify(clipboardItems)) {
						clipboardItems = newItems;
						console.log('📋 Clipboard items refreshed');
					}
				}
			} catch (error) {
				console.error('❌ Failed to check localStorage:', error);
			}
		};

		// 1초마다 localStorage 체크
		const storageCheckInterval = setInterval(checkLocalStorage, 1000);

		window.addEventListener('storage', handleStorageChange);

		// 클린업 함수 반환
		return () => {
			window.removeEventListener('storage', handleStorageChange);
			clearInterval(storageCheckInterval);
		};
	});

	function addClipboardItem() {
		if (!newItemContent.trim()) return;

		const newItem = {
			id: Date.now(),
			type: detectContentType(newItemContent),
			content: newItemContent,
			title: newItemTitle || generateAutoTitle(newItemContent),
			timestamp: new Date().toLocaleString('ko-KR', {
				year: 'numeric',
				month: '2-digit',
				day: '2-digit',
				hour: '2-digit',
				minute: '2-digit'
			}),
			tags: ['수동추가']
		};

		clipboardItems = [newItem, ...clipboardItems];

		// localStorage에 저장
		try {
			localStorage.setItem('clipboardItems', JSON.stringify(clipboardItems));
			console.log('✅ Manually added item saved to localStorage');
		} catch (error) {
			console.error('❌ Failed to save to localStorage:', error);
		}

		newItemContent = '';
		newItemTitle = '';
	}

	async function copyToClipboard(content: string) {
		try {
			if (isTauri && typeof globalThis !== 'undefined' && (globalThis as any).__TAURI_INTERNALS__) {
				const { invoke } = (globalThis as any).__TAURI_INTERNALS__;
				await invoke('plugin:clipboard-manager|write_text', { text: content });
			} else {
				await navigator.clipboard.writeText(content);
			}
			console.log('📋 Copied to clipboard:', `"${content.substring(0, 30)}..."`);
		} catch (error) {
			console.error('❌ Failed to copy to clipboard:', error);
		}
	}

	function deleteItem(id: number) {
		clipboardItems = clipboardItems.filter(item => item.id !== id);

		// localStorage에 저장
		try {
			localStorage.setItem('clipboardItems', JSON.stringify(clipboardItems));
			console.log('✅ Item deleted and saved to localStorage');
		} catch (error) {
			console.error('❌ Failed to save after delete:', error);
		}
	}

	function detectContentType(content: string): string {
		const urlPattern = /^https?:\/\/.+/i;
		if (urlPattern.test(content.trim())) {
			return 'url';
		}

		const codePatterns = [
			/function\s+\w+\s*\(/,
			/const\s+\w+\s*=/,
			/let\s+\w+\s*=/,
			/var\s+\w+\s*=/,
			/class\s+\w+/,
			/import\s+.+from/,
			/export\s+(default\s+)?/,
			/console\.(log|error|warn)/,
			/\$\(.*\)/,
			/\{\s*\n.*\n\s*\}/s,
		];

		if (codePatterns.some(pattern => pattern.test(content))) {
			return 'code';
		}

		return 'text';
	}

	function generateAutoTitle(content: string): string {
		const trimmed = content.trim();

		if (trimmed.startsWith('http')) {
			try {
				const url = new URL(trimmed);
				return url.hostname;
			} catch {
				return 'URL';
			}
		}

		const firstLine = trimmed.split('\n')[0];
		return firstLine.length > 50
			? firstLine.substring(0, 47) + '...'
			: firstLine || '제목 없음';
	}

	function getItemIcon(type: string) {
		switch (type) {
			case 'url':
				return Link;
			case 'code':
				return FileText;
			case 'image':
				return Image;
			default:
				return FileText;
		}
	}

	function getItemTypeColor(type: string) {
		switch (type) {
			case 'url':
				return 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400';
			case 'code':
				return 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400';
			case 'image':
				return 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400';
			default:
				return 'bg-gray-100 text-gray-800 dark:bg-gray-800/20 dark:text-gray-400';
		}
	}
</script>

<svelte:head>
	<title>클립보드 - Mofumofu</title>
	<meta name="description" content="자주 사용하는 텍스트, 링크, 코드를 저장하고 관리하세요." />
</svelte:head>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="flex items-center gap-3 mb-4">
				<Clipboard class="h-8 w-8 text-gray-700 dark:text-gray-300" />
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white">클립보드</h1>
				{#if isTauri}
					<Badge variant="secondary" class="bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400">
						자동 모니터링 활성
					</Badge>
				{/if}
			</div>
			<p class="text-lg text-gray-600 dark:text-gray-400">
				{#if isTauri}
					클립보드 변경이 자동으로 감지되어 저장됩니다. 수동으로도 추가할 수 있습니다.
				{:else}
					자주 사용하는 텍스트, 링크, 코드 스니펫을 저장하고 빠르게 사용하세요.
				{/if}
			</p>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- 새 아이템 추가 -->
			<div class="lg:col-span-1">
				<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6">
					<h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
						<Plus class="h-5 w-5" />
						새 아이템 추가
					</h2>

					<div class="space-y-4">
						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
								제목 (선택사항)
							</label>
							<Input
								bind:value={newItemTitle}
								placeholder="클립보드 아이템 제목"
								class="w-full"
							/>
						</div>

						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
								내용
							</label>
							<Textarea
								bind:value={newItemContent}
								placeholder="저장할 텍스트, 링크, 코드를 입력하세요..."
								rows="4"
								class="w-full"
							/>
						</div>

						<Button
							onclick={addClipboardItem}
							class="w-full"
							disabled={!newItemContent.trim()}
						>
							<Plus class="h-4 w-4 mr-2" />
							추가하기
						</Button>
					</div>
				</div>
			</div>

			<!-- 클립보드 아이템 목록 -->
			<div class="lg:col-span-2">
				<div class="space-y-4">
					<div class="flex items-center justify-between">
						<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
							저장된 아이템 ({clipboardItems.length})
						</h2>
					</div>

					{#if clipboardItems.length === 0}
						<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-12 text-center">
							<Clipboard class="h-16 w-16 text-gray-400 dark:text-gray-500 mx-auto mb-4" />
							<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">
								{#if isTauri}
									클립보드 모니터링이 활성화되었습니다
								{:else}
									아직 저장된 아이템이 없습니다
								{/if}
							</h3>
							<p class="text-gray-600 dark:text-gray-400">
								{#if isTauri}
									Ctrl+C로 복사하면 자동으로 여기에 저장됩니다.
								{:else}
									자주 사용하는 텍스트나 링크를 저장해보세요.
								{/if}
							</p>
						</div>
					{:else}
						{#each clipboardItems as item}
							{@const IconComponent = getItemIcon(item.type)}
							<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6">
								<div class="flex items-start justify-between mb-4">
									<div class="flex items-center gap-3">
										<IconComponent class="h-5 w-5 text-gray-600 dark:text-gray-400" />
										<div>
											<h3 class="font-semibold text-gray-900 dark:text-white">
												{item.title}
											</h3>
											<p class="text-sm text-gray-500 dark:text-gray-400">
												{item.timestamp}
											</p>
										</div>
									</div>

									<div class="flex items-center gap-2">
										<Button
											variant="ghost"
											size="sm"
											onclick={() => copyToClipboard(item.content)}
											title="클립보드에 복사"
										>
											<Copy class="h-4 w-4" />
										</Button>
										<Button
											variant="ghost"
											size="sm"
											onclick={() => deleteItem(item.id)}
											title="삭제"
											class="text-red-500 hover:text-red-700"
										>
											<Trash2 class="h-4 w-4" />
										</Button>
									</div>
								</div>

								<div class="mb-4">
									{#if item.type === 'code'}
										<pre class="bg-gray-100 dark:bg-gray-700 p-4 rounded-lg text-sm overflow-x-auto font-mono"><code>{item.content}</code></pre>
									{:else if item.type === 'url'}
										<div class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
											<a
												href={item.content}
												target="_blank"
												rel="noopener noreferrer"
												class="text-blue-600 dark:text-blue-400 hover:underline break-all"
											>
												{item.content}
											</a>
										</div>
									{:else}
										<div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
											<p class="text-gray-800 dark:text-gray-200 whitespace-pre-wrap break-words">
												{item.content}
											</p>
										</div>
									{/if}
								</div>

								<div class="flex flex-wrap gap-2">
									<Badge variant="secondary" class={getItemTypeColor(item.type)}>
										{item.type}
									</Badge>
									{#each item.tags as tag}
										<Badge variant="outline" class="text-xs">
											{tag}
										</Badge>
									{/each}
								</div>
							</div>
						{/each}
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>