<svelte:options runes={false} />

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
import cytoscape from 'cytoscape';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Activity, Pause, Play, RefreshCcw, Server, Database, Globe, Wifi } from '@lucide/svelte';

	type NodeType = 'server' | 'database' | 'external' | 'client';

	interface RelationshipNode {
		id: string;
		type: NodeType;
		name: string;
		ip: string;
		position?: { x: number; y: number };
	}

	interface RelationshipEdge {
		id: string;
		source: string;
		target: string;
		bandwidth: number;
		traffic: number;
	}

	const sampleNodes: RelationshipNode[] = [
		{ id: 'lb', type: 'server', name: 'Load Balancer', ip: '192.168.1.5', position: { x: 100, y: 200 } },
		{ id: 'web', type: 'server', name: 'Web Server', ip: '192.168.1.10', position: { x: 320, y: 160 } },
		{ id: 'api', type: 'server', name: 'API Server', ip: '192.168.1.20', position: { x: 600, y: 120 } },
		{ id: 'db', type: 'database', name: 'PostgreSQL DB', ip: '192.168.1.30', position: { x: 860, y: 180 } },
		{ id: 'cache', type: 'database', name: 'Redis Cache', ip: '192.168.1.40', position: { x: 620, y: 320 } },
		{ id: 'external', type: 'external', name: 'Payment Gateway', ip: '203.0.113.15', position: { x: 320, y: 340 } },
		{ id: 'client', type: 'client', name: 'Client Apps', ip: '0.0.0.0/0', position: { x: 80, y: 360 } }
	];

	const sampleEdges: RelationshipEdge[] = [
		{ id: 'lb-web', source: 'lb', target: 'web', bandwidth: 1200, traffic: 0 },
		{ id: 'web-api', source: 'web', target: 'api', bandwidth: 800, traffic: 0 },
		{ id: 'api-db', source: 'api', target: 'db', bandwidth: 400, traffic: 0 },
		{ id: 'api-cache', source: 'api', target: 'cache', bandwidth: 500, traffic: 0 },
		{ id: 'web-external', source: 'web', target: 'external', bandwidth: 150, traffic: 0 },
		{ id: 'client-lb', source: 'client', target: 'lb', bandwidth: 900, traffic: 0 }
	];

	const MAX_BANDWIDTH = Math.max(...sampleEdges.map((edge) => edge.bandwidth));

	let container: HTMLDivElement | null = null;
let cy: cytoscape.Core | null = null;
let simulationTimer: ReturnType<typeof setTimeout> | null = null;

	let isPlaying = true;
	let simulationSpeed = 60;
	let totalThroughput = 0;
	let lastUpdated = new Date();

	const chipStyles: Record<NodeType, { label: string; icon: typeof Server; badgeClass: string }> = {
		server: { label: '서버', icon: Server, badgeClass: 'bg-sky-100 text-sky-700' },
		database: { label: '데이터베이스', icon: Database, badgeClass: 'bg-emerald-100 text-emerald-700' },
		external: { label: '외부', icon: Globe, badgeClass: 'bg-purple-100 text-purple-700' },
		client: { label: '클라이언트', icon: Wifi, badgeClass: 'bg-orange-100 text-orange-700' }
	};

	onMount(() => {
		initCytoscape();
		startSimulation();

		return () => {
			stopSimulation();
			if (cy) {
				cy.destroy();
				cy = null;
			}
		};
	});

	onDestroy(() => {
		stopSimulation();
		cy?.destroy();
		cy = null;
	});

	function initCytoscape() {
		if (!container) return;

		cy = cytoscape({
			container,
			elements: buildElements(),
			style: createStyles(),
			layout: { name: 'preset' },
			wheelSensitivity: 0.2
		});

		cy.on('tap', 'node', (evt) => {
			const data = evt.target.data();
			cy?.nodes().removeClass('selected');
			evt.target.addClass('selected');
			console.info(`노드 ${data.label} (${data.ip}) 선택`);
		});

		cy.on('tap', () => {
			cy?.nodes().removeClass('selected');
		});

		cy.fit(undefined, 50);
	}

	function buildElements() {
		const nodeElements = sampleNodes.map((node) => ({
			data: {
				id: node.id,
				label: node.name,
				type: node.type,
				ip: node.ip
			},
			position: node.position,
			classes: node.type
		}));

		const edgeElements = sampleEdges.map((edge) => ({
			data: {
				id: edge.id,
				source: edge.source,
				target: edge.target,
				bandwidth: edge.bandwidth,
				traffic: edge.traffic,
				label: `${edge.traffic} Mbps`
			}
		}));

		return [...nodeElements, ...edgeElements];
	}

	function createStyles() {
		return [
			{
				selector: 'node',
				style: {
					width: 70,
					height: 70,
					'label': 'data(label)',
					'font-size': 12,
					'font-weight': 600,
					'color': '#0f172a',
					'text-wrap': 'wrap',
					'text-valign': 'center',
					'text-halign': 'center',
					'background-color': '#38bdf8',
					'border-width': 2,
					'border-color': '#0ea5e9',
					'shadow-blur': 12,
					'shadow-opacity': 0.18,
					'shadow-color': '#0f172a'
				}
			},
			{
				selector: 'node.server',
				style: {
					'background-color': '#38bdf8',
					'border-color': '#0ea5e9'
				}
			},
			{
				selector: 'node.database',
				style: {
					'background-color': '#34d399',
					'border-color': '#059669'
				}
			},
			{
				selector: 'node.external',
				style: {
					'background-color': '#a855f7',
					'border-color': '#7c3aed'
				}
			},
			{
				selector: 'node.client',
				style: {
					'background-color': '#fb923c',
					'border-color': '#f97316'
				}
			},
			{
				selector: 'node.selected',
				style: {
					'shadow-color': '#facc15',
					'shadow-opacity': 0.45,
					'shadow-blur': 20
				}
			},
			{
				selector: 'edge',
				style: {
					'curve-style': 'bezier',
					'target-arrow-shape': 'triangle',
					'target-arrow-color': '#94a3b8',
					'line-color': '#94a3b8',
					'width': `mapData(traffic, 0, ${MAX_BANDWIDTH}, 1.5, 12)`,
					'label': 'data(label)',
					'font-size': 9,
					'color': '#1e293b',
					'text-background-color': '#e2e8f0',
					'text-background-opacity': 0.85,
					'text-background-padding': 2,
					'text-border-opacity': 0,
					'text-rotation': 'autorotate'
				}
			},
			{
				selector: 'edge.active',
				style: {
					'line-color': '#38bdf8',
					'target-arrow-color': '#38bdf8'
				}
			}
		];
	}

	function startSimulation() {
		stopSimulation();

		const tick = () => {
			if (isPlaying) {
				updateTraffic();
			}
			simulationTimer = setTimeout(tick, Math.max(40, 220 - simulationSpeed));
		};

		tick();
	}

	function stopSimulation() {
		if (simulationTimer) {
			clearTimeout(simulationTimer);
			simulationTimer = null;
		}
	}

	function updateTraffic() {
		if (!cy) return;

		let runningTotal = 0;
		sampleEdges.forEach((edge) => {
			const change = Math.random() * edge.bandwidth * 0.25;
			const direction = Math.random() > 0.5 ? 1 : -1;
			const nextTraffic = Math.max(0, Math.min(edge.bandwidth, edge.traffic + direction * change));
			edge.traffic = nextTraffic;
			runningTotal += nextTraffic;

			const cyEdge = cy.getElementById(edge.id);
			if (cyEdge) {
				cyEdge.data('traffic', nextTraffic);
				cyEdge.data('label', `${Math.round(nextTraffic)} Mbps`);
				cyEdge.toggleClass('active', nextTraffic > edge.bandwidth * 0.6);
			}
		});

		totalThroughput = Math.round(runningTotal);
		lastUpdated = new Date();
	}

	function toggleSimulation() {
		isPlaying = !isPlaying;
	}

	function resetTraffic() {
		totalThroughput = 0;
		lastUpdated = new Date();
		sampleEdges.forEach((edge) => {
			edge.traffic = 0;
			const cyEdge = cy?.getElementById(edge.id);
			if (cyEdge) {
				cyEdge.data('traffic', 0);
				cyEdge.data('label', '0 Mbps');
				cyEdge.removeClass('active');
			}
		});
	}

</script>

<div class="page">
	<section class="intro">
		<div>
			<h1>서비스 관계도</h1>
			<p>내부 서비스와 외부 의존성 간 실시간 트래픽 흐름을 시각화합니다.</p>
		</div>
		<div class="controls">
			<Button variant="outline" size="sm" on:click={toggleSimulation}>
				{#if isPlaying}
					<Pause class="icon" /> 일시 정지
				{:else}
					<Play class="icon" /> 재생
				{/if}
			</Button>
			<Button variant="outline" size="sm" on:click={resetTraffic}>
				<RefreshCcw class="icon" /> 초기화
			</Button>
		</div>
	</section>

	<section class="stats">
		<div class="card">
			<span class="label">전체 노드</span>
			<strong>{sampleNodes.length}</strong>
		</div>
		<div class="card">
			<span class="label">연결 수</span>
			<strong>{sampleEdges.length}</strong>
		</div>
		<div class="card">
			<span class="label">총 처리량</span>
			<strong>{totalThroughput} Mbps</strong>
			<span class="sub">{lastUpdated.toLocaleTimeString()}</span>
		</div>
		<div class="card slider">
			<label for="speed">시뮬레이션 속도</label>
			<input
				id="speed"
				type="range"
				min="20"
				max="180"
				step="10"
				bind:value={simulationSpeed}
				on:input={(event) => (simulationSpeed = Number((event.currentTarget as HTMLInputElement).value))}
			/>
		</div>
	</section>

	<section class="legend">
		{#each Object.entries(chipStyles) as [type, info]}
			<Badge class={`chip ${info.badgeClass}`}>
				<svelte:component this={info.icon} class="icon" /> {info.label}
			</Badge>
		{/each}
	</section>

	<section class="graph">
		<div class="graph-header">
			<h2><Activity class="icon" /> 네트워크 트래픽</h2>
			<span>최근 갱신: {lastUpdated.toLocaleTimeString()}</span>
		</div>
		<div class="graph-container" bind:this={container}></div>
	</section>
</div>

<style>
	.page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.intro {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 1rem;
	}

	.intro h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #0f172a;
	}

	.intro p {
		margin: 0.25rem 0 0;
		color: #475569;
	}

	.controls {
		display: flex;
		gap: 0.5rem;
	}

	.stats {
		display: grid;
		gap: 1rem;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
	}

	.card {
		padding: 1rem 1.25rem;
		border-radius: 0.75rem;
		background: white;
		box-shadow: 0 10px 18px -12px rgba(15, 23, 42, 0.35);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	:global(body.dark) .card {
		background: #1f2937;
		color: #e2e8f0;
	}

	.card .label {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
	}

	.card strong {
		font-size: 1.4rem;
		font-weight: 700;
	}

	.card .sub {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.card.slider {
		gap: 0.75rem;
	}

	.card.slider input[type='range'] {
		width: 100%;
	}

	.legend {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.legend .chip {
		display: inline-flex;
		align-items: center;
		gap: 0.35rem;
		font-weight: 600;
		border-radius: 999px;
		padding: 0.35rem 0.75rem;
	}

	.legend .chip .icon {
		width: 14px;
		height: 14px;
	}

	.graph {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.graph-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		color: #475569;
	}

	.graph-header h2 {
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: #0f172a;
	}

	.graph-container {
		height: 560px;
		border-radius: 1rem;
		overflow: hidden;
		border: 1px solid rgba(148, 163, 184, 0.35);
	}

	.icon {
		width: 16px;
		height: 16px;
	}
</style>
