<!-- 관계도 페이지 - 네트워크 트래픽 시각화 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { GitBranch, Server, Database, Globe, Wifi, Activity, Play, Pause, RotateCcw } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	let container: HTMLDivElement;
	let editor: any;
	let isPlaying = $state(true);
	let simulationSpeed = $state([50]);
	let trafficData = $state({
		nodes: [],
		connections: [],
		lastUpdate: Date.now()
	});

	// 네트워크 노드 타입 정의
	const nodeTypes = {
		server: {
			icon: Server,
			color: 'bg-blue-500',
			label: '서버',
			textColor: 'text-blue-700'
		},
		database: {
			icon: Database,
			color: 'bg-green-500',
			label: '데이터베이스',
			textColor: 'text-green-700'
		},
		external: {
			icon: Globe,
			color: 'bg-purple-500',
			label: '외부 API',
			textColor: 'text-purple-700'
		},
		client: {
			icon: Wifi,
			color: 'bg-orange-500',
			label: '클라이언트',
			textColor: 'text-orange-700'
		}
	};

	// 샘플 노드 데이터
	const sampleNodes = [
		{
			id: 'web-server',
			type: 'server',
			name: 'Web Server',
			ip: '192.168.1.10',
			position: { x: 300, y: 200 },
			traffic: { in: 0, out: 0 }
		},
		{
			id: 'api-server',
			type: 'server',
			name: 'API Server',
			ip: '192.168.1.20',
			position: { x: 600, y: 150 },
			traffic: { in: 0, out: 0 }
		},
		{
			id: 'database',
			type: 'database',
			name: 'PostgreSQL DB',
			ip: '192.168.1.30',
			position: { x: 900, y: 200 },
			traffic: { in: 0, out: 0 }
		},
		{
			id: 'redis',
			type: 'database',
			name: 'Redis Cache',
			ip: '192.168.1.40',
			position: { x: 600, y: 350 },
			traffic: { in: 0, out: 0 }
		},
		{
			id: 'external-api',
			type: 'external',
			name: 'Payment Gateway',
			ip: '203.0.113.15',
			position: { x: 300, y: 400 },
			traffic: { in: 0, out: 0 }
		},
		{
			id: 'load-balancer',
			type: 'server',
			name: 'Load Balancer',
			ip: '192.168.1.5',
			position: { x: 100, y: 200 },
			traffic: { in: 0, out: 0 }
		}
	];

	// 연결 정의
	const connections = [
		{ from: 'load-balancer', to: 'web-server', bandwidth: 1000, currentTraffic: 0 },
		{ from: 'web-server', to: 'api-server', bandwidth: 500, currentTraffic: 0 },
		{ from: 'api-server', to: 'database', bandwidth: 200, currentTraffic: 0 },
		{ from: 'api-server', to: 'redis', bandwidth: 300, currentTraffic: 0 },
		{ from: 'web-server', to: 'external-api', bandwidth: 100, currentTraffic: 0 }
	];

	let animationFrameId: number;

	onMount(async () => {
		await initializeReteEditor();
		startSimulation();

		return () => {
			if (animationFrameId) {
				cancelAnimationFrame(animationFrameId);
			}
		};
	});

	async function initializeReteEditor() {
		try {
			// Rete.js 초기화는 나중에 구현
			// 현재는 캔버스 기반 구현으로 대체
			renderNetworkVisualization();
		} catch (error) {
			console.error('Failed to initialize Rete editor:', error);
		}
	}

	function renderNetworkVisualization() {
		if (!container) return;

		// SVG 생성
		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.style.width = '100%';
		svg.style.height = '600px';
		svg.style.background = '#f8fafc';
		svg.style.border = '1px solid #e2e8f0';
		svg.style.borderRadius = '12px';

		container.innerHTML = '';
		container.appendChild(svg);

		renderConnections(svg);
		renderNodes(svg);
	}

	function renderConnections(svg: SVGSVGElement) {
		connections.forEach((conn) => {
			const fromNode = sampleNodes.find((n) => n.id === conn.from);
			const toNode = sampleNodes.find((n) => n.id === conn.to);

			if (!fromNode || !toNode) return;

			// 연결선 그리기
			const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
			line.setAttribute('x1', String(fromNode.position.x + 60));
			line.setAttribute('y1', String(fromNode.position.y + 40));
			line.setAttribute('x2', String(toNode.position.x + 60));
			line.setAttribute('y2', String(toNode.position.y + 40));
			line.setAttribute('stroke', '#94a3b8');
			line.setAttribute('stroke-width', String(Math.max(2, conn.currentTraffic / 10)));
			line.setAttribute('opacity', String(0.6 + (conn.currentTraffic / conn.bandwidth) * 0.4));

			svg.appendChild(line);

			// 트래픽 표시 (흐르는 점들)
			if (conn.currentTraffic > 0) {
				const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
				circle.setAttribute('r', '4');
				circle.setAttribute('fill', '#3b82f6');

				const animateMotion = document.createElementNS('http://www.w3.org/2000/svg', 'animateMotion');
				animateMotion.setAttribute('dur', `${2000 / simulationSpeed[0]}ms`);
				animateMotion.setAttribute('repeatCount', 'indefinite');

				const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
				path.setAttribute(
					'd',
					`M${fromNode.position.x + 60},${fromNode.position.y + 40} L${toNode.position.x + 60},${toNode.position.y + 40}`
				);

				animateMotion.appendChild(path);
				circle.appendChild(animateMotion);
				svg.appendChild(circle);
			}
		});
	}

	function renderNodes(svg: SVGSVGElement) {
		sampleNodes.forEach((node) => {
			const group = document.createElementNS('http://www.w3.org/2000/svg', 'g');
			group.setAttribute('transform', `translate(${node.position.x}, ${node.position.y})`);

			// 노드 배경
			const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
			rect.setAttribute('width', '120');
			rect.setAttribute('height', '80');
			rect.setAttribute('rx', '8');
			rect.setAttribute('fill', 'white');
			rect.setAttribute('stroke', nodeTypes[node.type].color.replace('bg-', '#'));
			rect.setAttribute('stroke-width', '2');
			rect.setAttribute('filter', 'drop-shadow(0 4px 6px rgb(0 0 0 / 0.1))');

			group.appendChild(rect);

			// 노드 제목
			const title = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			title.setAttribute('x', '60');
			title.setAttribute('y', '20');
			title.setAttribute('text-anchor', 'middle');
			title.setAttribute('font-size', '12');
			title.setAttribute('font-weight', 'bold');
			title.textContent = node.name;

			group.appendChild(title);

			// IP 주소
			const ip = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			ip.setAttribute('x', '60');
			ip.setAttribute('y', '35');
			ip.setAttribute('text-anchor', 'middle');
			ip.setAttribute('font-size', '10');
			ip.setAttribute('fill', '#64748b');
			ip.textContent = node.ip;

			group.appendChild(ip);

			// 트래픽 정보
			const traffic = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			traffic.setAttribute('x', '60');
			traffic.setAttribute('y', '50');
			traffic.setAttribute('text-anchor', 'middle');
			traffic.setAttribute('font-size', '9');
			traffic.setAttribute('fill', '#059669');
			traffic.textContent = `↓${node.traffic.in} MB/s ↑${node.traffic.out} MB/s`;

			group.appendChild(traffic);

			// 타입 표시
			const type = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			type.setAttribute('x', '60');
			type.setAttribute('y', '65');
			type.setAttribute('text-anchor', 'middle');
			type.setAttribute('font-size', '8');
			type.setAttribute('fill', '#6b7280');
			type.textContent = nodeTypes[node.type].label;

			group.appendChild(type);

			svg.appendChild(group);
		});
	}

	function generateTrafficData() {
		// 실제 트래픽 시뮬레이션
		connections.forEach((conn) => {
			const variation = (Math.random() - 0.5) * 0.3;
			const baseTraffic = conn.bandwidth * 0.3;
			conn.currentTraffic = Math.max(0, baseTraffic + baseTraffic * variation);
		});

		// 노드별 트래픽 계산
		sampleNodes.forEach((node) => {
			const incomingTraffic = connections.filter((c) => c.to === node.id).reduce((sum, c) => sum + c.currentTraffic, 0);

			const outgoingTraffic = connections
				.filter((c) => c.from === node.id)
				.reduce((sum, c) => sum + c.currentTraffic, 0);

			node.traffic.in = Math.round(incomingTraffic);
			node.traffic.out = Math.round(outgoingTraffic);
		});

		trafficData.lastUpdate = Date.now();
	}

	function startSimulation() {
		if (!isPlaying) return;

		generateTrafficData();
		renderNetworkVisualization();

		animationFrameId = requestAnimationFrame(() => {
			setTimeout(startSimulation, 1000 / (simulationSpeed[0] / 10));
		});
	}

	function togglePlayPause() {
		isPlaying = !isPlaying;
		if (isPlaying) {
			startSimulation();
		} else {
			if (animationFrameId) {
				cancelAnimationFrame(animationFrameId);
			}
		}
	}

	function resetSimulation() {
		connections.forEach((conn) => {
			conn.currentTraffic = 0;
		});
		sampleNodes.forEach((node) => {
			node.traffic.in = 0;
			node.traffic.out = 0;
		});
		renderNetworkVisualization();
	}

	function onSpeedChange(event: Event) {
		const target = event.target as HTMLInputElement;
		simulationSpeed = [parseInt(target.value)];
	}
</script>

<svelte:head>
	<title>네트워크 관계도 - Snow-X</title>
	<meta name="description" content="서버 간 네트워크 트래픽과 관계를 실시간으로 시각화합니다." />
</svelte:head>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="mb-4 flex items-center gap-3">
				<GitBranch class="h-8 w-8 text-gray-700 dark:text-gray-300" />
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white">네트워크 관계도</h1>
			</div>
			<p class="text-lg text-gray-600 dark:text-gray-400">
				서버 간의 실시간 네트워크 트래픽과 데이터 흐름을 시각화합니다.
			</p>
		</div>

		<div class="grid grid-cols-1 gap-6 xl:grid-cols-4">
			<!-- 컨트롤 패널 -->
			<div class="xl:col-span-1">
				<div class="rounded-xl border border-gray-200 bg-white p-6 shadow-lg dark:border-gray-700 dark:bg-gray-800">
					<div class="mb-6 flex items-center gap-2">
						<Activity class="h-5 w-5" />
						<h3 class="text-lg font-semibold">시뮬레이션 제어</h3>
					</div>
					<div class="space-y-4">
						<!-- 재생/일시정지 -->
						<div class="flex gap-2">
							<Button onclick={togglePlayPause} variant={isPlaying ? 'default' : 'outline'} class="flex-1">
								{#if isPlaying}
									<Pause class="mr-2 h-4 w-4" />
									일시정지
								{:else}
									<Play class="mr-2 h-4 w-4" />
									재생
								{/if}
							</Button>
							<Button onclick={resetSimulation} variant="outline">
								<RotateCcw class="h-4 w-4" />
							</Button>
						</div>

						<!-- 속도 조절 -->
						<div>
							<label class="mb-2 block text-sm font-medium">
								시뮬레이션 속도: {simulationSpeed[0]}%
							</label>
							<input
								type="range"
								min="10"
								max="200"
								step="10"
								value={simulationSpeed[0]}
								onchange={onSpeedChange}
								class="slider h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200 dark:bg-gray-700"
							/>
						</div>

						<!-- 범례 -->
						<div>
							<h4 class="mb-3 text-sm font-medium">노드 타입</h4>
							<div class="space-y-2">
								{#each Object.entries(nodeTypes) as [key, type]}
									<div class="flex items-center gap-2">
										<div class="h-3 w-3 rounded {type.color}"></div>
										<span class="text-sm">{type.label}</span>
									</div>
								{/each}
							</div>
						</div>

						<!-- 통계 -->
						<div>
							<h4 class="mb-3 text-sm font-medium">네트워크 통계</h4>
							<div class="space-y-2 text-sm">
								<div class="flex justify-between">
									<span>총 노드:</span>
									<Badge variant="secondary">{sampleNodes.length}</Badge>
								</div>
								<div class="flex justify-between">
									<span>총 연결:</span>
									<Badge variant="secondary">{connections.length}</Badge>
								</div>
								<div class="flex justify-between">
									<span>활성 트래픽:</span>
									<Badge variant="secondary">
										{connections.filter((c) => c.currentTraffic > 0).length}
									</Badge>
								</div>
								<div class="flex justify-between">
									<span>최근 업데이트:</span>
									<span class="text-xs text-gray-500">
										{new Date(trafficData.lastUpdate).toLocaleTimeString()}
									</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- 네트워크 시각화 -->
			<div class="xl:col-span-3">
				<div class="rounded-xl border border-gray-200 bg-white p-6 shadow-lg dark:border-gray-700 dark:bg-gray-800">
					<div class="mb-6 flex items-center justify-between">
						<h3 class="text-lg font-semibold">네트워크 토폴로지</h3>
						<div class="flex items-center gap-2">
							<div class="h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
							<span class="text-sm text-gray-500">실시간</span>
						</div>
					</div>
					<div bind:this={container} class="h-[600px] w-full overflow-hidden">
						<!-- Rete.js 에디터가 여기에 마운트됩니다 -->
					</div>

					<!-- 트래픽 정보 -->
					<div class="mt-4 rounded-lg bg-gray-50 p-4 dark:bg-gray-800">
						<h4 class="mb-2 text-sm font-medium">실시간 트래픽</h4>
						<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
							{#each connections as conn}
								<div class="text-xs">
									<div class="font-medium">
										{sampleNodes.find((n) => n.id === conn.from)?.name}
										→
										{sampleNodes.find((n) => n.id === conn.to)?.name}
									</div>
									<div class="text-gray-600 dark:text-gray-400">
										{Math.round(conn.currentTraffic)} MB/s / {conn.bandwidth} MB/s
									</div>
									<div class="mt-1 h-1.5 w-full rounded-full bg-gray-200">
										<div
											class="h-1.5 rounded-full bg-blue-600 transition-all duration-300"
											style="width: {(conn.currentTraffic / conn.bandwidth) * 100}%"
										></div>
									</div>
								</div>
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
