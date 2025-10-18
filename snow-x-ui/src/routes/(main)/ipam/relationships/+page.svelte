<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Network } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { deviceApi, deviceLibraryApi, contactApi } from '$lib/api/office';
	import type { Device, DeviceLibrary, Contact } from '$lib/api/office';

	interface Node {
		id: string;
		type: 'device' | 'library' | 'contact';
		label: string;
		data: any;
		x: number;
		y: number;
	}

	interface Link {
		source: string;
		target: string;
		type: 'creates' | 'manages' | 'syncs';
		label?: string;
	}

	let nodes = $state<Node[]>([]);
	let links = $state<Link[]>([]);
	let isLoading = $state(true);
	let dataTruncated = $state(false);
	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null = null;
	let selectedNode = $state<Node | null>(null);
	let isDragging = false;
	let dragNode: Node | null = null;
	let dragOffset = { x: 0, y: 0 };

	// 색상 정의
	const colors = {
		device: '#3b82f6',
		library: '#8b5cf6',
		contact: '#10b981'
	};

onMount(async () => {
	await loadData();
	setupCanvas();
	render();
});

onDestroy(() => {
	if (!canvas) return;
	canvas.removeEventListener('mousedown', handleMouseDown);
	canvas.removeEventListener('mousemove', handleMouseMove);
	canvas.removeEventListener('mouseup', handleMouseUp);
	canvas.removeEventListener('click', handleClick);
});

async function loadData() {
	isLoading = true;
	try {
		// 데이터 로드
		const [devicesResponse, librariesResponse, contactsResponse] = await Promise.all([
			deviceApi.getDevices({ limit: 200 }),
			deviceLibraryApi.getDeviceLibraries({ limit: 200 }),
			contactApi.getContacts({ limit: 200 })
		]);

		const devices = devicesResponse.devices;
		const libraries = librariesResponse.libraries;
		const contacts = contactsResponse.contacts;

		dataTruncated =
			devicesResponse.total > devices.length ||
			librariesResponse.total > libraries.length ||
			contactsResponse.total > contacts.length;

			// 캔버스 크기 기준으로 초기 위치 설정
			const canvasWidth = canvas?.offsetWidth || 1200;

			// 노드 생성 - 타입별로 세로로 배치
			const deviceNodes: Node[] = devices.map((d, i) => ({
				id: `device-${d.id}`,
				type: 'device' as const,
				label: d.name,
				data: d,
				x: 150,
				y: 100 + i * 80
			}));

			const libraryNodes: Node[] = libraries.map((l, i) => ({
				id: `library-${l.id}`,
				type: 'library' as const,
				label: l.name,
				data: l,
				x: canvasWidth - 150,
				y: 100 + i * 80
			}));

			const contactNodes: Node[] = contacts.map((c, i) => ({
				id: `contact-${c.id}`,
				type: 'contact' as const,
				label: c.name,
				data: c,
				x: canvasWidth / 2,
				y: 100 + i * 80
			}));

			nodes = [...deviceNodes, ...libraryNodes, ...contactNodes];

		// 링크 생성
		const newLinks: Link[] = [];
		const linkKeys = new Set<string>();

		function addLink(link: Link) {
			const key = `${link.source}-${link.target}-${link.type}`;
			if (linkKeys.has(key)) return;
			linkKeys.add(key);
			newLinks.push(link);
		}

		// 라이브러리 -> 디바이스 연결 (library의 device_id 기반)
		libraries.forEach((library) => {
			if (!library.device_id) return;
			const deviceExists = devices.some((d) => d.id === library.device_id);
			if (!deviceExists) return;

			addLink({
				source: `library-${library.id}`,
				target: `device-${library.device_id}`,
				type: 'creates',
				label: '구성 요소'
			});
		});

		// 담당자 -> 장비/라이브러리 연결 추론
		contacts.forEach((contact) => {
			const contactNodeId = `contact-${contact.id}`;
			const matchedDevices: Device[] = [];

			// 1) 외부 연동 ID 매칭
			if (contact.external_api_connection_id) {
				devices.forEach((device) => {
					if (device.external_api_connection_id && device.external_api_connection_id === contact.external_api_connection_id) {
						matchedDevices.push(device);
					}
				});
			}

			// 2) responsibilities 텍스트 기반 탐색
			const responsibilityText = (contact.responsibilities || '').toLowerCase();

			if (responsibilityText) {
				devices.forEach((device) => {
					if (!matchedDevices.includes(device) && responsibilityText.includes(device.name.toLowerCase())) {
						matchedDevices.push(device);
					}
				});

				libraries.forEach((library) => {
					if (responsibilityText.includes(library.name.toLowerCase())) {
						addLink({
							source: contactNodeId,
							target: `library-${library.id}`,
							type: 'manages',
							label: '담당 라이브러리'
						});
					}
				});
			}

			matchedDevices.forEach((device) => {
				const isSynced =
					contact.external_api_connection_id &&
					device.external_api_connection_id &&
					contact.external_api_connection_id === device.external_api_connection_id;

				addLink({
					source: contactNodeId,
					target: `device-${device.id}`,
					type: isSynced ? 'syncs' : 'manages',
					label: isSynced ? '동기화' : '담당 장비'
				});
			});
		});

		links = newLinks;
	} catch (error) {
		console.error('Failed to load data:', error);
	} finally {
		isLoading = false;
		render();
	}
}

	function setupCanvas() {
		if (!canvas) return;
		ctx = canvas.getContext('2d');
		if (!ctx) return;

		// 캔버스 크기 설정
		canvas.width = canvas.offsetWidth;
		canvas.height = canvas.offsetHeight;

		// 이벤트 리스너
		canvas.addEventListener('mousedown', handleMouseDown);
		canvas.addEventListener('mousemove', handleMouseMove);
		canvas.addEventListener('mouseup', handleMouseUp);
		canvas.addEventListener('click', handleClick);
	}

	function render() {
		if (!ctx || !canvas) return;

		// 캔버스 지우기
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		// 링크 그리기 (베지어 곡선 사용)
		links.forEach((link) => {
			const sourceNode = nodes.find((n) => n.id === link.source);
			const targetNode = nodes.find((n) => n.id === link.target);
			if (!sourceNode || !targetNode) return;

			// 노드의 반지름
			const radius = 40;

			// 노드 중심에서 가장자리까지의 벡터 계산
			const dx = targetNode.x - sourceNode.x;
			const dy = targetNode.y - sourceNode.y;
			const distance = Math.sqrt(dx * dx + dy * dy);

			if (distance < radius * 2) return; // 너무 가까우면 선을 그리지 않음

			// 정규화된 방향 벡터
			const nx = dx / distance;
			const ny = dy / distance;

			// 노드 가장자리에서 시작/끝나도록 조정
			const startX = sourceNode.x + nx * radius;
			const startY = sourceNode.y + ny * radius;
			const endX = targetNode.x - nx * radius;
			const endY = targetNode.y - ny * radius;

			// 베지어 곡선 제어점 계산
			const controlPointOffset = distance * 0.3;
			const perpX = -ny;
			const perpY = nx;

			const midX = (startX + endX) / 2;
			const midY = (startY + endY) / 2;

			const cp1x = midX + perpX * controlPointOffset;
			const cp1y = midY + perpY * controlPointOffset;

			// 곡선 그리기
			const linkColor =
				link.type === 'creates' ? '#64748b' : link.type === 'manages' ? '#f97316' : '#10b981';

			ctx!.beginPath();
			ctx!.moveTo(startX, startY);
			ctx!.quadraticCurveTo(cp1x, cp1y, endX, endY);
			ctx!.strokeStyle = linkColor;
			ctx!.lineWidth = 2;
			ctx!.stroke();

			// 화살표 그리기 (곡선 끝점의 접선 방향으로)
			const t = 0.95; // 곡선의 95% 지점
			const arrowStartX = (1-t)*(1-t)*startX + 2*(1-t)*t*cp1x + t*t*endX;
			const arrowStartY = (1-t)*(1-t)*startY + 2*(1-t)*t*cp1y + t*t*endY;

			const angle = Math.atan2(endY - arrowStartY, endX - arrowStartX);
			const arrowSize = 12;

			ctx!.beginPath();
			ctx!.moveTo(endX, endY);
			ctx!.lineTo(
				endX - arrowSize * Math.cos(angle - Math.PI / 6),
				endY - arrowSize * Math.sin(angle - Math.PI / 6)
			);
			ctx!.lineTo(
				endX - arrowSize * Math.cos(angle + Math.PI / 6),
				endY - arrowSize * Math.sin(angle + Math.PI / 6)
			);
			ctx!.closePath();
			ctx!.fillStyle = linkColor;
			ctx!.fill();

			if (link.label) {
				const labelT = 0.5;
				const labelX =
					(1 - labelT) * (1 - labelT) * startX + 2 * (1 - labelT) * labelT * cp1x + labelT * labelT * endX;
				const labelY =
					(1 - labelT) * (1 - labelT) * startY + 2 * (1 - labelT) * labelT * cp1y + labelT * labelT * endY;

				ctx!.save();
				const isDarkMode = canvas?.ownerDocument?.documentElement.classList.contains('dark');
				ctx!.fillStyle = isDarkMode ? 'rgba(226, 232, 240, 0.85)' : 'rgba(30, 41, 59, 0.75)';
				ctx!.font = '10px sans-serif';
				ctx!.textAlign = 'center';
				ctx!.fillText(link.label, labelX, labelY - 8);
				ctx!.restore();
			}
		});

		// 노드 그리기
		nodes.forEach((node) => {
			const isSelected = selectedNode?.id === node.id;
			const radius = 40;

			// 노드 원
			ctx!.beginPath();
			ctx!.arc(node.x, node.y, radius, 0, Math.PI * 2);
			ctx!.fillStyle = colors[node.type];
			ctx!.fill();
			ctx!.strokeStyle = isSelected ? '#fff' : 'transparent';
			ctx!.lineWidth = 3;
			ctx!.stroke();

			// 노드 레이블
			ctx!.fillStyle = '#fff';
			ctx!.font = 'bold 12px sans-serif';
			ctx!.textAlign = 'center';
			ctx!.textBaseline = 'middle';
			const maxWidth = radius * 1.8;
			const text = node.label.length > 10 ? node.label.substring(0, 10) + '...' : node.label;
			ctx!.fillText(text, node.x, node.y);

			// 타입 레이블
			ctx!.font = '10px sans-serif';
			ctx!.fillText(node.type, node.x, node.y + radius + 15);
		});
	}

	function handleMouseDown(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;

		const node = findNodeAt(x, y);
		if (node) {
			isDragging = true;
			dragNode = node;
			dragOffset = { x: x - node.x, y: y - node.y };
		}
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isDragging || !dragNode) return;

		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;

		dragNode.x = x - dragOffset.x;
		dragNode.y = y - dragOffset.y;

		render();
	}

	function handleMouseUp() {
		isDragging = false;
		dragNode = null;
	}

	function handleClick(e: MouseEvent) {
		if (isDragging) return;

		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;

		const node = findNodeAt(x, y);
		selectedNode = node;
		render();
	}

	function findNodeAt(x: number, y: number): Node | null {
		const radius = 40;
		return nodes.find((node) => {
			const dx = node.x - x;
			const dy = node.y - y;
			return Math.sqrt(dx * dx + dy * dy) <= radius;
		}) || null;
	}

	function resetLayout() {
		if (!canvas) return;

		const canvasWidth = canvas.offsetWidth;

		// 타입별로 그룹화
		const deviceNodes = nodes.filter(n => n.type === 'device');
		const libraryNodes = nodes.filter(n => n.type === 'library');
		const contactNodes = nodes.filter(n => n.type === 'contact');

		// 각 그룹을 세로로 배치
		const leftX = 150;
		const rightX = canvasWidth - 150;
		const centerX = canvasWidth / 2;

		// 디바이스는 왼쪽
		deviceNodes.forEach((node, i) => {
			node.x = leftX;
			node.y = 100 + i * 100;
		});

		// 라이브러리는 오른쪽
		libraryNodes.forEach((node, i) => {
			node.x = rightX;
			node.y = 100 + i * 100;
		});

		// 담당자는 중앙
		contactNodes.forEach((node, i) => {
			node.x = centerX;
			node.y = 100 + i * 100;
		});

		// 포스 기반 미세 조정
		const iterations = 50;
		const repulsion = 3000;

		for (let iter = 0; iter < iterations; iter++) {
			// 같은 타입끼리만 반발력 적용
			[deviceNodes, libraryNodes, contactNodes].forEach(group => {
				group.forEach((node1) => {
					let fy = 0;

					group.forEach((node2) => {
						if (node1.id === node2.id) return;

						const dy = node1.y - node2.y;
						const distance = Math.abs(dy) || 1;

						if (distance < 150) { // 너무 가까우면 밀어냄
							const force = repulsion / (distance * distance);
							fy += (dy / distance) * force;
						}
					});

					node1.y += fy * 0.01;
				});
			});

			// 연결된 노드들을 서로 당김
			links.forEach((link) => {
				const sourceNode = nodes.find((n) => n.id === link.source);
				const targetNode = nodes.find((n) => n.id === link.target);
				if (!sourceNode || !targetNode) return;

				const dy = targetNode.y - sourceNode.y;
				const distance = Math.abs(dy) || 1;

				if (distance > 100) { // 너무 멀면 당김
					const force = distance * 0.02;
					const fy = (dy / distance) * force;

					sourceNode.y += fy * 0.1;
					targetNode.y -= fy * 0.1;
				}
			});
		}

		render();
	}
</script>

<div class="min-h-screen bg-gray-50 p-6 dark:bg-gray-900">
	<div class="mx-auto max-w-7xl">
		<!-- Header -->
		<div class="mb-6 flex items-center justify-between">
			<div>
				<h1 class="flex items-center gap-2 text-3xl font-bold text-gray-900 dark:text-gray-100">
					<Network class="h-8 w-8" />
					데이터 관계
				</h1>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					디바이스, 라이브러리, 담당자 간의 관계를 시각화합니다
				</p>
			</div>
			<div class="flex gap-2">
				<Button
					variant="ghost"
					class="gap-2 rounded-full border border-orange-500 px-4 py-2 text-orange-600 transition hover:-translate-y-0.5 hover:bg-orange-50 hover:shadow-sm dark:border-orange-400 dark:text-orange-300 dark:hover:bg-orange-900/30"
					onclick={resetLayout}
				>
					레이아웃 재정렬
				</Button>
				<Button
					class="gap-2 rounded-full bg-gradient-to-r from-orange-500 via-orange-400 to-orange-500 px-4 py-2 text-white shadow-lg shadow-orange-500/30 transition hover:-translate-y-0.5 hover:shadow-xl dark:from-orange-400 dark:via-orange-300 dark:to-orange-400 dark:text-gray-900"
					onclick={loadData}
				>
					새로고침
				</Button>
			</div>
		</div>

		<!-- Legend -->
		<div class="mb-4 rounded-lg bg-white p-4 shadow-sm dark:bg-gray-800">
			<div class="mb-2 flex items-center justify-between">
				<h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">범례</h3>
				{#if dataTruncated}
					<span class="rounded-full bg-amber-100 px-3 py-1 text-xs font-semibold text-amber-700 dark:bg-amber-900/40 dark:text-amber-200">
						표시된 항목은 최대 200개까지입니다. 필터를 사용해 주세요.
					</span>
				{/if}
			</div>
			<div class="flex flex-wrap gap-6">
				<div class="flex items-center gap-2">
					<div class="h-4 w-4 rounded-full" style="background-color: {colors.device}"></div>
					<span class="text-sm text-gray-700 dark:text-gray-300">디바이스</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="h-4 w-4 rounded-full" style="background-color: {colors.library}"></div>
					<span class="text-sm text-gray-700 dark:text-gray-300">라이브러리</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="h-4 w-4 rounded-full" style="background-color: {colors.contact}"></div>
					<span class="text-sm text-gray-700 dark:text-gray-300">담당자</span>
				</div>
				<div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
					<span class="inline-flex h-2 w-4 rounded-full bg-[#f97316]"></span> 담당
					<span class="inline-flex h-2 w-4 rounded-full bg-[#10b981]"></span> 동기화
					<span class="inline-flex h-2 w-4 rounded-full bg-[#64748b]"></span> 구성
				</div>
			</div>
		</div>

		<!-- Canvas Container -->
		<div class="overflow-hidden rounded-lg bg-white shadow-sm dark:bg-gray-800">
			<div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">관계도</h2>
				<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
					노드를 드래그하여 이동할 수 있습니다. 클릭하면 상세 정보를 확인할 수 있습니다.
				</p>
			</div>

			{#if isLoading}
				<div class="flex items-center justify-center py-12">
					<div class="h-8 w-8 animate-spin rounded-full border-4 border-orange-600 border-t-transparent"></div>
				</div>
			{:else}
				<div class="relative bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800">
					<!-- 배경 그리드 -->
					<div class="pointer-events-none absolute inset-0" style="
						background-image:
							linear-gradient(rgba(100, 116, 139, 0.1) 1px, transparent 1px),
							linear-gradient(90deg, rgba(100, 116, 139, 0.1) 1px, transparent 1px);
						background-size: 50px 50px;
					"></div>
					<canvas
						bind:this={canvas}
						class="relative h-[700px] w-full cursor-move"
					></canvas>
				</div>
			{/if}
		</div>

		<!-- Selected Node Details -->
		{#if selectedNode}
			<div class="mt-4 rounded-lg bg-white p-6 shadow-sm dark:bg-gray-800">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-gray-100">상세 정보</h3>
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<span class="text-sm font-medium text-gray-500 dark:text-gray-400">타입</span>
						<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.type}</p>
					</div>
					<div>
						<span class="text-sm font-medium text-gray-500 dark:text-gray-400">이름</span>
						<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.label}</p>
					</div>
					{#if selectedNode.type === 'device'}
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">디바이스 타입</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.device_type || '-'}</p>
						</div>
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">상태</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.status || '-'}</p>
						</div>
					{:else if selectedNode.type === 'library'}
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">제조사</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.manufacturer || '-'}</p>
						</div>
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">모델</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.model || '-'}</p>
						</div>
					{:else if selectedNode.type === 'contact'}
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">이메일</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.email || '-'}</p>
						</div>
						<div>
							<span class="text-sm font-medium text-gray-500 dark:text-gray-400">부서</span>
							<p class="mt-1 text-gray-900 dark:text-gray-100">{selectedNode.data.department || '-'}</p>
						</div>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Statistics -->
		<div class="mt-4 grid gap-4 md:grid-cols-3">
			<div class="rounded-lg bg-white p-4 shadow-sm dark:bg-gray-800">
				<div class="flex items-center gap-3">
					<div class="rounded-lg p-2" style="background-color: {colors.device}20">
						<div class="h-5 w-5 rounded-full" style="background-color: {colors.device}"></div>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">디바이스</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">
							{nodes.filter((n) => n.type === 'device').length}
						</p>
					</div>
				</div>
			</div>
			<div class="rounded-lg bg-white p-4 shadow-sm dark:bg-gray-800">
				<div class="flex items-center gap-3">
					<div class="rounded-lg p-2" style="background-color: {colors.library}20">
						<div class="h-5 w-5 rounded-full" style="background-color: {colors.library}"></div>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">라이브러리</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">
							{nodes.filter((n) => n.type === 'library').length}
						</p>
					</div>
				</div>
			</div>
			<div class="rounded-lg bg-white p-4 shadow-sm dark:bg-gray-800">
				<div class="flex items-center gap-3">
					<div class="rounded-lg p-2" style="background-color: {colors.contact}20">
						<div class="h-5 w-5 rounded-full" style="background-color: {colors.contact}"></div>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">담당자</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-gray-100">
							{nodes.filter((n) => n.type === 'contact').length}
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
