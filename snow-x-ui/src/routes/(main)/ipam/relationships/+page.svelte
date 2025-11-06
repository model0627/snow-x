<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { goto } from '$app/navigation';
import { Network, Search } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { deviceApi, deviceLibraryApi, contactApi } from '$lib/api/office';
	import type { Device, DeviceLibrary, Contact, DeviceContact } from '$lib/api/office';

	type NodeType = 'device' | 'library' | 'contact';

	interface BaseNode {
		id: string;
		type: NodeType;
		label: string;
		data: Device | DeviceLibrary | Contact;
	}

	interface PositionedNode extends BaseNode {
		x: number;
		y: number;
		angle: number;
		radius: number;
		ring: 'inner' | 'outer';
	}

	interface Link {
		source: string;
		target: string;
		type: 'creates' | 'manages' | 'syncs';
		label?: string;
	}

	interface BackgroundArc {
		id: string;
		path: string;
		color: string;
		label: string;
		textX: number;
		textY: number;
	}

	const colors: Record<NodeType, string> = {
		device: '#3b82f6',
		library: '#8b5cf6',
		contact: '#10b981'
	};

	const linkColors: Record<Link['type'], string> = {
		creates: '#64748b',
		manages: '#f97316',
		syncs: '#10b981'
	};

	const typeLabels: Record<NodeType, string> = {
		device: '디바이스',
		library: '라이브러리',
		contact: '담당자'
	};

const statsConfig: Array<{ type: NodeType; label: string; color: string }> = [
	{ type: 'device', label: '디바이스', color: colors.device },
	{ type: 'library', label: '라이브러리', color: colors.library },
	{ type: 'contact', label: '담당자', color: colors.contact }
];

const MAX_NODES_PER_GROUP = 30;

let nodes = $state<BaseNode[]>([]);
let positionedNodes = $state<PositionedNode[]>([]);
let links = $state<Link[]>([]);
let backgroundArcs = $state<BackgroundArc[]>([]);
let innerHalo = $state<{ radius: number; color: string } | null>(null);
let isLoading = $state(true);
let dataTruncated = $state(false);

let searchQuery = $state('');
const normalizedSearch = $derived.by(() => searchQuery.trim().toLowerCase());

let connectionFilterMode = $state<'all' | 'connected' | 'disconnected'>('all');

const nodesMatchingSearch = $derived.by<BaseNode[]>(() => {
	const query = normalizedSearch;
	if (!query) return nodes;

	const matchingIds = new Set<string>();
	nodes.forEach((node) => {
		if (node.label.toLowerCase().includes(query)) {
			matchingIds.add(node.id);
		}
	});

	if (matchingIds.size === 0) {
		return [];
	}

	const expandedIds = new Set(matchingIds);
	links.forEach((link) => {
		if (expandedIds.has(link.source) || expandedIds.has(link.target)) {
			expandedIds.add(link.source);
			expandedIds.add(link.target);
		}
	});

	return nodes.filter((node) => expandedIds.has(node.id));
});

const connectedNodeIds = $derived.by<Set<string>>(() => {
	const ids = new Set<string>();
	links.forEach((link) => {
		ids.add(link.source);
		ids.add(link.target);
	});
	return ids;
});

const visibleNodes = $derived.by<BaseNode[]>(() => {
	const base = nodesMatchingSearch;
	if (connectionFilterMode === 'connected') {
		return base.filter((node) => connectedNodeIds.has(node.id));
	}
	if (connectionFilterMode === 'disconnected') {
		return base.filter((node) => !connectedNodeIds.has(node.id));
	}
	return base;
});

const visibleLinks = $derived.by<Link[]>(() => {
	const idSet = new Set(visibleNodes.map((node) => node.id));
	if (idSet.size === 0) return [];
	return links.filter((link) => idSet.has(link.source) && idSet.has(link.target));
});

const searchActive = $derived.by(
	() => normalizedSearch.length > 0 || connectionFilterMode !== 'all'
);

let hoveredNodeId = $state<string | null>(null);
let selectedNodeId = $state<string | null>(null);
const hoveredNode = $derived.by<PositionedNode | null>(() => {
	return hoveredNodeId ? positionedNodes.find((node) => node.id === hoveredNodeId) ?? null : null;
});

const selectedNode = $derived.by<PositionedNode | null>(() => {
	return selectedNodeId ? positionedNodes.find((node) => node.id === selectedNodeId) ?? null : null;
});

const selectedConnections = $derived.by<PositionedNode[]>(() => {
	const node = selectedNode;
	return node ? getConnectedNodes(node.id) : [];
});

let container: HTMLDivElement | null = null;
let resizeObserver: ResizeObserver | null = null;
let currentObservedElement: HTMLDivElement | null = null;

let viewport = $state({ width: 1200, height: 720 });
let mapCenter = $state({ x: 600, y: 360 });

onMount(async () => {
	await loadData();
});

onDestroy(() => {
	resizeObserver?.disconnect();
	resizeObserver = null;
	currentObservedElement = null;
});

$effect(() => {
	if (container) {
		setupResizeObserver();
		return () => {
			resizeObserver?.disconnect();
			resizeObserver = null;
			currentObservedElement = null;
		};
	}

	if (resizeObserver) {
		resizeObserver.disconnect();
		resizeObserver = null;
		currentObservedElement = null;
	}
});

$effect(() => {
	visibleLinks;
	const currentNodes = visibleNodes;
	const idSet = new Set(currentNodes.map((node) => node.id));

	if (selectedNodeId && !idSet.has(selectedNodeId)) {
		selectedNodeId = null;
	}

	if (hoveredNodeId && !idSet.has(hoveredNodeId)) {
		hoveredNodeId = null;
	}

	updateLayout();
});

async function loadData() {
	isLoading = true;

	try {
			const [devicesResponse, librariesResponse, contactsResponse] = await Promise.all([
				deviceApi.getDevices({ limit: 200 }),
				deviceLibraryApi.getDeviceLibraries({ limit: 200 }),
				contactApi.getContacts({ limit: 200 })
			]);

			const deviceList = devicesResponse.devices ?? [];
			const libraryList = librariesResponse.libraries ?? [];
			const contactList = contactsResponse.contacts ?? [];

			let devices = deviceList.slice(0, MAX_NODES_PER_GROUP);
			let libraries = libraryList.slice(0, MAX_NODES_PER_GROUP);
			let contacts = contactList.slice(0, MAX_NODES_PER_GROUP);

			dataTruncated =
				devicesResponse.total > deviceList.length ||
				librariesResponse.total > libraryList.length ||
				contactsResponse.total > contactList.length ||
				deviceList.length > devices.length ||
				libraryList.length > libraries.length ||
				contactList.length > contacts.length;

			devices.sort((a, b) => a.name.localeCompare(b.name));
			libraries.sort((a, b) => a.name.localeCompare(b.name));
			contacts.sort((a, b) => a.name.localeCompare(b.name));

			const contactAssignments = await fetchDeviceContactAssignments(deviceList);
			const contactIdToName = new Map<string, string>();
			const requiredDeviceIds = new Set<string>();
			const requiredContactIds = new Set<string>();

			contactAssignments.forEach((mappings, deviceId) => {
				requiredDeviceIds.add(deviceId);
				mappings.forEach((mapping) => {
					requiredContactIds.add(mapping.contact_id);
					if (mapping.contact_name) {
						contactIdToName.set(mapping.contact_id, mapping.contact_name);
					}
				});
			});

			const currentDeviceIds = new Set(devices.map((device) => device.id));
			requiredDeviceIds.forEach((deviceId) => {
				if (!currentDeviceIds.has(deviceId)) {
					const device = deviceList.find((d) => d.id === deviceId);
					if (device) {
						devices.push(device);
						currentDeviceIds.add(deviceId);
					}
				}
			});

			const currentContactIds = new Set(contacts.map((contact) => contact.id));
			requiredContactIds.forEach((contactId) => {
				if (!currentContactIds.has(contactId)) {
					const contact = contactList.find((c) => c.id === contactId);
					if (contact) {
						contacts.push(contact);
						currentContactIds.add(contactId);
					} else {
						const fallbackName = contactIdToName.get(contactId) ?? '미등록 담당자';
						const placeholder: Contact = {
							id: contactId,
							name: fallbackName,
							created_by: '00000000-0000-0000-0000-000000000000',
							created_at: new Date(0).toISOString(),
							updated_at: new Date(0).toISOString(),
							is_active: true,
							source_type: 'manual',
							title: undefined,
							department: undefined,
							phone: undefined,
							mobile: undefined,
							email: undefined,
							office_location: undefined,
							responsibilities: undefined,
							external_api_connection_id: undefined
						};
						contacts.push(placeholder);
						currentContactIds.add(contactId);
					}
				}
			});

			devices.sort((a, b) => a.name.localeCompare(b.name));
			libraries.sort((a, b) => a.name.localeCompare(b.name));
			contacts.sort((a, b) => a.name.localeCompare(b.name));

			const deviceNodes: BaseNode[] = devices.map((device) => ({
				id: `device-${device.id}`,
				type: 'device',
				label: device.name,
				data: device
			}));

			const libraryNodes: BaseNode[] = libraries.map((library) => ({
				id: `library-${library.id}`,
				type: 'library',
				label: library.name,
				data: library
			}));

			const contactNodes: BaseNode[] = contacts.map((contact) => ({
				id: `contact-${contact.id}`,
				type: 'contact',
				label: contact.name,
				data: contact
			}));

			nodes = [...deviceNodes, ...libraryNodes, ...contactNodes];
			links = buildLinks(devices, libraries, contacts, contactAssignments);
			selectedNodeId = null;
			hoveredNodeId = null;

			updateLayout();
		} catch (error) {
			console.error('Failed to load relationships data:', error);
		} finally {
			isLoading = false;
		}
	}

	async function fetchDeviceContactAssignments(devices: Device[]): Promise<Map<string, DeviceContact[]>> {
		const assignments = new Map<string, DeviceContact[]>();
		await Promise.all(
			devices.map(async (device) => {
				try {
					const response = await deviceApi.getAssignedContacts(device.id);
					const mappings = response.mappings ?? [];
					const deviceMappings = mappings.filter((mapping) => mapping.resource_type === 'device');
					if (deviceMappings.length) {
						assignments.set(device.id, deviceMappings);
					}
				} catch (error) {
					console.error('Failed to load contacts for device:', device.id, error);
				}
			})
		);
		return assignments;
	}

	function buildLinks(
		devices: Device[],
		libraries: DeviceLibrary[],
		contacts: Contact[],
		contactAssignments: Map<string, DeviceContact[]>
	): Link[] {
		const result: Link[] = [];
		const linkKeys = new Set<string>();

		const deviceIds = new Set(devices.map((device) => device.id));
		const deviceMap = new Map(devices.map((device) => [device.id, device]));
		const contactIds = new Set(contacts.map((contact) => contact.id));

		const directDeviceAssignments = new Map<string, Set<string>>();
		const contactDeviceRoles = new Map<string, Map<string, string | undefined>>();

		contactAssignments.forEach((mappings, deviceId) => {
			if (!deviceIds.has(deviceId)) return;

			mappings.forEach((mapping) => {
				if (mapping.resource_type !== 'device') return;
				const contactId = mapping.contact_id;
				if (!contactIds.has(contactId)) return;

				let deviceSet = directDeviceAssignments.get(contactId);
				if (!deviceSet) {
					deviceSet = new Set<string>();
					directDeviceAssignments.set(contactId, deviceSet);
				}
				deviceSet.add(deviceId);

				let roleMap = contactDeviceRoles.get(contactId);
				if (!roleMap) {
					roleMap = new Map<string, string | undefined>();
					contactDeviceRoles.set(contactId, roleMap);
				}
				roleMap.set(deviceId, mapping.role || undefined);
			});
		});

		const addLink = (link: Link) => {
			const key = `${link.source}-${link.target}-${link.type}`;
			if (linkKeys.has(key)) return;
			linkKeys.add(key);
			result.push(link);
		};

		libraries.forEach((library) => {
			if (!library.device_id) return;
			if (!deviceIds.has(library.device_id)) return;

			addLink({
				source: `library-${library.id}`,
				target: `device-${library.device_id}`,
				type: 'creates',
				label: '구성 요소'
			});
		});

		contacts.forEach((contact) => {
			const contactNodeId = `contact-${contact.id}`;
			const matchedDeviceIds = new Set<string>();
			const directAssignments = directDeviceAssignments.get(contact.id);
			if (directAssignments) {
				directAssignments.forEach((deviceId) => matchedDeviceIds.add(deviceId));
			}

			if (contact.external_api_connection_id) {
				devices.forEach((device) => {
					if (
						device.external_api_connection_id &&
						device.external_api_connection_id === contact.external_api_connection_id
					) {
						matchedDeviceIds.add(device.id);
					}
				});
			}

			const responsibilityText = (contact.responsibilities || '').toLowerCase();

			if (responsibilityText) {
				devices.forEach((device) => {
					if (!matchedDeviceIds.has(device.id) && responsibilityText.includes(device.name.toLowerCase())) {
						matchedDeviceIds.add(device.id);
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

			matchedDeviceIds.forEach((deviceId) => {
				const device = deviceMap.get(deviceId);
				if (!device) return;
				const isSynced =
					contact.external_api_connection_id &&
					device.external_api_connection_id &&
					contact.external_api_connection_id === device.external_api_connection_id;
				const role = contactDeviceRoles.get(contact.id)?.get(device.id);

				addLink({
					source: contactNodeId,
					target: `device-${device.id}`,
					type: isSynced ? 'syncs' : 'manages',
					label: role || (isSynced ? '동기화' : '담당 장비')
				});
			});
		});

		return result;
	}

	function updateLayout() {
		const currentNodes = visibleNodes;

		if (!currentNodes.length) {
			positionedNodes = [];
			backgroundArcs = [];
			innerHalo = null;
			return;
		}

		const width = viewport.width;
		const height = viewport.height;
		const center = { x: width / 2, y: height / 2 };
		const baseRadius = Math.min(width, height) * 0.28;
		const outerRadius = baseRadius * 1.7;
		const innerRadius = baseRadius * 1.05;

		const deviceNodes = currentNodes.filter((node) => node.type === 'device');
		const libraryNodes = currentNodes.filter((node) => node.type === 'library');
		const contactNodes = currentNodes.filter((node) => node.type === 'contact');

		const positioned: PositionedNode[] = [
			...distributeArc(deviceNodes, center, outerRadius, 135, 255, 'outer'),
			...distributeArc(libraryNodes, center, outerRadius, -75, 45, 'outer'),
			...distributeArc(contactNodes, center, innerRadius, 210, 510, 'inner')
		];

		positionedNodes = positioned;
		mapCenter = center;

		backgroundArcs = [
			buildBackgroundArc('device-band', center, outerRadius + 46, 135, 255, 64, '디바이스', 'rgba(59, 130, 246, 0.12)'),
			buildBackgroundArc('library-band', center, outerRadius + 46, -75, 45, 64, '라이브러리', 'rgba(139, 92, 246, 0.12)')
		];

		innerHalo = {
			radius: innerRadius + 24,
			color: 'rgba(16, 185, 129, 0.12)'
		};
	}

	function distributeArc(
		groupNodes: BaseNode[],
		center: { x: number; y: number },
		radius: number,
		startAngleDeg: number,
		endAngleDeg: number,
		ring: 'inner' | 'outer'
	): PositionedNode[] {
		if (!groupNodes.length) return [];

		const start = toRadians(startAngleDeg);
		const end = toRadians(endAngleDeg);
		const total = groupNodes.length;
		const angleStep = total === 1 ? 0 : (end - start) / (total - 1);

		return groupNodes.map((node, index) => {
			const angle = total === 1 ? start + (end - start) / 2 : start + angleStep * index;
			const x = center.x + radius * Math.cos(angle);
			const y = center.y + radius * Math.sin(angle);

			return {
				...node,
				x,
				y,
				angle,
				radius,
				ring
			};
		});
	}

	function buildBackgroundArc(
		id: string,
		center: { x: number; y: number },
		radius: number,
		startAngle: number,
		endAngle: number,
		thickness: number,
		label: string,
		color: string
	): BackgroundArc {
		const path = describeArc(center.x, center.y, radius, startAngle, endAngle, thickness);
		const midAngle = toRadians((startAngle + endAngle) / 2);
		const labelRadius = radius - thickness / 2;
		const textX = center.x + labelRadius * Math.cos(midAngle);
		const textY = center.y + labelRadius * Math.sin(midAngle);

		return {
			id,
			path,
			color,
			label,
			textX,
			textY
		};
	}

	function describeArc(
		centerX: number,
		centerY: number,
		radius: number,
		startAngle: number,
		endAngle: number,
		thickness: number
	): string {
		const innerRadius = Math.max(0, radius - thickness);
		const start = polarToCartesian(centerX, centerY, radius, endAngle);
		const end = polarToCartesian(centerX, centerY, radius, startAngle);
		const innerStart = polarToCartesian(centerX, centerY, innerRadius, startAngle);
		const innerEnd = polarToCartesian(centerX, centerY, innerRadius, endAngle);
		const largeArcFlag = Math.abs(endAngle - startAngle) <= 180 ? '0' : '1';

		return [
			'M',
			start.x,
			start.y,
			'A',
			radius,
			radius,
			0,
			largeArcFlag,
			0,
			end.x,
			end.y,
			'L',
			innerStart.x,
			innerStart.y,
			'A',
			innerRadius,
			innerRadius,
			0,
			largeArcFlag,
			1,
			innerEnd.x,
			innerEnd.y,
			'Z'
		].join(' ');
	}

	function polarToCartesian(centerX: number, centerY: number, radius: number, angleInDegrees: number) {
		const angleInRadians = toRadians(angleInDegrees);
		return {
			x: centerX + radius * Math.cos(angleInRadians),
			y: centerY + radius * Math.sin(angleInRadians)
		};
	}

	function toRadians(degrees: number) {
		return (degrees * Math.PI) / 180;
	}

	function setupResizeObserver() {
		if (!container || container === currentObservedElement) return;

		resizeObserver?.disconnect();

		resizeObserver = new ResizeObserver((entries) => {
			if (!entries.length) return;
			const { width } = entries[0].contentRect;
			applyViewportSize(width);
		});

		resizeObserver.observe(container);
		currentObservedElement = container;
		applyViewportSize(container.clientWidth || viewport.width);
	}

	function applyViewportSize(width: number) {
		const nextWidth = Math.max(720, Math.round(width));
		const nextHeight = Math.max(640, Math.round(nextWidth * 0.6));

		if (viewport.width !== nextWidth || viewport.height !== nextHeight) {
			viewport = { width: nextWidth, height: nextHeight };
			mapCenter = { x: nextWidth / 2, y: nextHeight / 2 };
			updateLayout();
		}
	}

	function getLinkPath(link: Link): string {
		const source = positionedNodes.find((node) => node.id === link.source);
		const target = positionedNodes.find((node) => node.id === link.target);
		if (!source || !target) return '';

		const dx = target.x - source.x;
		const dy = target.y - source.y;
		const distance = Math.sqrt(dx * dx + dy * dy) || 1;
		const midX = (source.x + target.x) / 2;
		const midY = (source.y + target.y) / 2;
		const perpendicularX = -dy / distance;
		const perpendicularY = dx / distance;
		const curvature =
			link.type === 'syncs' ? distance * 0.38 : link.type === 'manages' ? distance * 0.32 : distance * 0.24;
		const biasTowardsCenter = 0.12;

		const controlX = midX + perpendicularX * curvature + (mapCenter.x - midX) * biasTowardsCenter;
		const controlY = midY + perpendicularY * curvature + (mapCenter.y - midY) * biasTowardsCenter;

		return `M ${source.x} ${source.y} Q ${controlX} ${controlY} ${target.x} ${target.y}`;
	}

	function getLinkColor(link: Link) {
		return linkColors[link.type];
	}

	function getLinkWidth(link: Link) {
		return isLinkActive(link) ? 3.2 : 1.6;
	}

	function getLinkOpacity(link: Link) {
		if (isLinkActive(link)) return 0.9;
		if (hoveredNodeId || selectedNodeId) {
			return isLinkConnectedToFocus(link) ? 0.35 : 0.12;
		}
		return 0.28;
	}

	function isLinkActive(link: Link) {
		if (hoveredNodeId && (link.source === hoveredNodeId || link.target === hoveredNodeId)) return true;
		if (selectedNodeId && (link.source === selectedNodeId || link.target === selectedNodeId)) return true;
		return false;
	}

	function isLinkConnectedToFocus(link: Link) {
		if (hoveredNodeId && (link.source === hoveredNodeId || link.target === hoveredNodeId)) return true;
		if (selectedNodeId && (link.source === selectedNodeId || link.target === selectedNodeId)) return true;
		return false;
	}

	function isNodeHighlighted(nodeId: string) {
		if (hoveredNodeId === nodeId || selectedNodeId === nodeId) return true;
		if (hoveredNodeId && areNodesConnected(hoveredNodeId, nodeId)) return true;
		if (selectedNodeId && areNodesConnected(selectedNodeId, nodeId)) return true;
		return false;
	}

	function isNodeDimmed(nodeId: string) {
		if (!hoveredNodeId && !selectedNodeId) return false;
		return !isNodeHighlighted(nodeId);
	}

	function areNodesConnected(a: string, b: string) {
		return visibleLinks.some(
			(link) => (link.source === a && link.target === b) || (link.source === b && link.target === a)
		);
	}

	function getNodeRadius(node: PositionedNode) {
		const base = node.ring === 'inner' ? 32 : 38;
		return isNodeHighlighted(node.id) ? base * 1.1 : base;
	}

	function getNodeStroke(node: PositionedNode) {
		if (selectedNodeId === node.id) return 'rgba(248, 113, 113, 0.45)';
		if (hoveredNodeId === node.id) return 'rgba(251, 191, 36, 0.45)';
		if (isNodeHighlighted(node.id)) return 'rgba(148, 163, 184, 0.3)';
		return 'rgba(15, 23, 42, 0.12)';
	}

	function getNodeFillOpacity(node: PositionedNode) {
		return isNodeDimmed(node.id) ? 0.35 : 0.95;
	}

	function getNodeShadow(node: PositionedNode) {
		if (isNodeHighlighted(node.id)) return 'url(#glow)';
		return undefined;
	}

	function handleNodeClick(nodeId: string) {
		selectedNodeId = selectedNodeId === nodeId ? null : nodeId;
	}

	function handleNodeEnter(nodeId: string) {
		hoveredNodeId = nodeId;
	}

	function handleNodeLeave(nodeId: string) {
		if (hoveredNodeId === nodeId) {
			hoveredNodeId = null;
		}
	}

function handleRefresh() {
	loadData();
}

function setConnectionFilter(mode: 'all' | 'connected' | 'disconnected') {
	connectionFilterMode = mode;
}

	function truncateLabel(value: string, limit = 14) {
		if (!value) return '';
		return value.length <= limit ? value : `${value.slice(0, limit - 3)}...`;
	}

	function toPercent(value: number, total: number) {
		if (!total) return '0';
		return ((value / total) * 100).toFixed(2);
	}

	function withAlpha(hexColor: string, alpha: number) {
		const sanitized = hexColor.replace('#', '');
		const bigint = parseInt(sanitized, 16);
		const r = (bigint >> 16) & 255;
		const g = (bigint >> 8) & 255;
		const b = bigint & 255;
		return `rgba(${r}, ${g}, ${b}, ${alpha})`;
	}

	function getStats(type: NodeType) {
		return visibleNodes.filter((node) => node.type === type).length;
	}

	function getConnectedNodes(nodeId: string): PositionedNode[] {
		if (!nodeId) return [];
		const connectionIds = new Set<string>();
		visibleLinks.forEach((link) => {
			if (link.source === nodeId) connectionIds.add(link.target);
			if (link.target === nodeId) connectionIds.add(link.source);
		});
		return positionedNodes.filter((node) => connectionIds.has(node.id));
	}

	function isDeviceNode(node: PositionedNode): node is PositionedNode & { data: Device } {
		return node.type === 'device';
	}

	function isLibraryNode(node: PositionedNode): node is PositionedNode & { data: DeviceLibrary } {
		return node.type === 'library';
	}

	function isContactNode(node: PositionedNode): node is PositionedNode & { data: Contact } {
		return node.type === 'contact';
	}
</script>

<div class="flex flex-col gap-6">
	<div class="rounded-xl bg-white p-6 shadow-sm dark:bg-slate-900">
		<div class="flex flex-wrap items-start justify-between gap-4">
			<div class="flex items-center gap-4">
				<div class="rounded-full bg-orange-100 p-2 text-orange-600 dark:bg-orange-950/40 dark:text-orange-300">
					<Network class="h-6 w-6" />
				</div>
				<div>
					<h1 class="text-xl font-semibold text-slate-900 dark:text-slate-100">연결 인사이트</h1>
					<p class="mt-1 max-w-3xl text-sm text-slate-600 dark:text-slate-400">
						디바이스, 라이브러리, 담당자 간 관계를 Wine &amp; Cheese Map 스타일의 반경형 뷰로 시각화했습니다.
						핸드오버가 잦은 팀에서도 한눈에 흐름을 파악할 수 있어요.
					</p>
				</div>
				</div>
				<div class="flex flex-wrap items-center gap-2">
					<div class="relative">
						<Search class="pointer-events-none absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
						<input
						type="search"
						bind:value={searchQuery}
						placeholder="노드 검색..."
						autocomplete="off"
						on:keydown={(event) => {
							if (event.key === 'Escape') {
								searchQuery = '';
							}
						}}
							class="w-48 rounded-lg border border-slate-200 bg-white py-2 pl-9 pr-3 text-sm text-slate-700 shadow-sm transition focus:border-orange-400 focus:outline-none focus:ring-2 focus:ring-orange-200 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-200 dark:focus:border-orange-500 dark:focus:ring-orange-500/30"
						/>
					</div>
					<div class="flex items-center gap-1">
						<Button
							size="sm"
							on:click={() => setConnectionFilter('all')}
							variant={connectionFilterMode === 'all' ? 'default' : 'outline'}
							type="button"
						>
							전체
						</Button>
						<Button
							size="sm"
							on:click={() => setConnectionFilter('connected')}
							variant={connectionFilterMode === 'connected' ? 'default' : 'outline'}
							type="button"
						>
							연결됨
						</Button>
						<Button
							size="sm"
							on:click={() => setConnectionFilter('disconnected')}
							variant={connectionFilterMode === 'disconnected' ? 'default' : 'outline'}
							type="button"
						>
							미연결
						</Button>
					</div>
					{#if searchActive}
						<Button
							size="sm"
							variant="ghost"
							type="button"
							on:click={() => {
								searchQuery = '';
								connectionFilterMode = 'all';
							}}
						>
							조건 초기화
						</Button>
					{/if}
					<Button size="sm" on:click={handleRefresh} disabled={isLoading}>
					{#if isLoading}
						<span class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-orange-500 border-t-transparent"></span>
					{/if}
					데이터 새로고침
				</Button>
				<Button size="sm" variant="outline" on:click={updateLayout} disabled={isLoading}>
					레이아웃 재정렬
				</Button>
			</div>
		</div>

		{#if dataTruncated}
			<div class="mt-4 rounded-lg border border-dashed border-orange-300 bg-orange-50/60 px-4 py-3 text-xs text-orange-700 dark:border-orange-500/50 dark:bg-orange-500/10 dark:text-orange-200">
				표시 가능한 최대 노드 수를 초과한 데이터는 맵에서 생략되었습니다. 필터를 조정해 주세요.
			</div>
		{/if}
	</div>

	<div class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm dark:border-slate-700 dark:bg-slate-900">
		<div class="flex flex-col gap-1 border-b border-slate-100 px-6 py-4 dark:border-slate-700">
			<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">관계 맵</h2>
			<p class="text-sm text-slate-500 dark:text-slate-400">
				왼쪽은 디바이스, 오른쪽은 라이브러리를 배열하고 중앙에 담당자 링을 배치했습니다. 노드를 클릭하면 상세 정보와 연결을 확인할 수 있습니다.
			</p>
		</div>

		{#if isLoading}
			<div class="flex h-[520px] items-center justify-center">
				<div class="h-10 w-10 animate-spin rounded-full border-4 border-orange-500 border-t-transparent"></div>
			</div>
	{:else}
		<div class="relative" bind:this={container}>
			{#if visibleNodes.length === 0}
				<div class="flex h-[520px] flex-col items-center justify-center gap-3 text-slate-500 dark:text-slate-400">
					<Network class="h-10 w-10 text-orange-500 dark:text-orange-300" />
					<p class="text-sm">
						{searchActive
							? '검색 결과가 없습니다. 검색어를 변경하거나 초기화해 보세요.'
							: '표시할 관계가 없습니다. 데이터가 충분한지 확인해 주세요.'}
					</p>
					{#if searchActive}
						<Button size="sm" variant="outline" on:click={() => (searchQuery = '')}>검색 초기화</Button>
					{/if}
				</div>
			{:else}
				<svg
					class="block w-full"
					style={`height: ${viewport.height}px`}
					viewBox={`0 0 ${viewport.width} ${viewport.height}`}
					preserveAspectRatio="xMidYMid meet"
				>
					<defs>
						<radialGradient id="backgroundGlow" cx="50%" cy="50%" r="50%">
							<stop offset="0%" stop-color="rgba(15, 23, 42, 0.4)" stop-opacity="0.12" />
							<stop offset="100%" stop-color="rgba(15, 23, 42, 0)" stop-opacity="0" />
						</radialGradient>
						<filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
							<feGaussianBlur stdDeviation="12" result="coloredBlur" />
							<feMerge>
								<feMergeNode in="coloredBlur" />
								<feMergeNode in="SourceGraphic" />
							</feMerge>
						</filter>
					</defs>

					<rect x="0" y="0" width={viewport.width} height={viewport.height} fill="url(#backgroundGlow)" opacity="0.35" />

					{#if innerHalo}
						<circle cx={mapCenter.x} cy={mapCenter.y} r={innerHalo.radius} fill={innerHalo.color} opacity="0.6" />
					{/if}

					{#each backgroundArcs as arc (arc.id)}
						<g>
							<path d={arc.path} fill={arc.color} opacity="0.85" />
							<text
								x={arc.textX}
								y={arc.textY}
								fill="currentColor"
								class="text-[11px] font-medium uppercase tracking-[0.35em] text-slate-500 dark:text-slate-300"
								text-anchor="middle"
							>
								{arc.label}
							</text>
						</g>
					{/each}

					{#each visibleLinks as link, index (link.source + link.target + link.type + index)}
						{@const path = getLinkPath(link)}
						{#if path}
							<path
								d={path}
								fill="none"
								stroke={getLinkColor(link)}
								stroke-width={getLinkWidth(link)}
								stroke-linecap="round"
								stroke-linejoin="round"
								opacity={getLinkOpacity(link)}
								class="transition-all duration-200 ease-out"
							/>
						{/if}
					{/each}

					{#each positionedNodes as node (node.id)}
						<g
							transform={`translate(${node.x}, ${node.y})`}
							class="transition-all duration-200 ease-out"
							opacity={isNodeDimmed(node.id) ? 0.28 : 1}
							on:mouseenter={() => handleNodeEnter(node.id)}
							on:mouseleave={() => handleNodeLeave(node.id)}
							on:click={() => handleNodeClick(node.id)}
						>
							<circle
								r={getNodeRadius(node)}
								fill={colors[node.type]}
								fill-opacity={getNodeFillOpacity(node)}
								stroke={getNodeStroke(node)}
								stroke-width={isNodeHighlighted(node.id) ? 4 : 2}
								filter={getNodeShadow(node)}
							/>
							<text
								y="-2"
								text-anchor="middle"
								font-size="12"
								font-weight="600"
								fill="white"
							>
								{truncateLabel(node.label)}
							</text>
							<text
								y="14"
								text-anchor="middle"
								font-size="10"
								fill="rgba(255, 255, 255, 0.85)"
							>
								{typeLabels[node.type]}
							</text>
						</g>
					{/each}
				</svg>

				{#if hoveredNode}
					<div
						class="pointer-events-none absolute rounded-lg bg-white px-3 py-2 text-xs text-slate-700 shadow-lg ring-1 ring-slate-200 dark:bg-slate-900 dark:text-slate-200 dark:ring-slate-700"
						style={`left: calc(${toPercent(hoveredNode.x, viewport.width)}% - 60px); top: calc(${toPercent(hoveredNode.y, viewport.height)}% - 70px);`}
					>
						<p class="font-semibold">{truncateLabel(hoveredNode.label, 18)}</p>
						<p class="mt-0.5 text-[11px] text-slate-500 dark:text-slate-400">{typeLabels[hoveredNode.type]}</p>
					</div>
				{/if}
			{/if}
		</div>
	{/if}
	</div>

	{#if selectedNode}
		<div class="rounded-xl bg-white p-6 shadow-sm dark:bg-slate-900">
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">선택한 노드</h3>
				<p class="text-sm text-slate-500 dark:text-slate-400">연결된 항목을 확인하고 필요한 액션을 이어가세요.</p>
			</div>
			<div class="flex items-center gap-2">
				{#if isDeviceNode(selectedNode)}
					<Button
						size="sm"
						variant="outline"
						type="button"
						on:click={() => goto(`/ipam/device/${selectedNode.data.id}`)}
					>
						디바이스 상세 보기
					</Button>
				{/if}
				<Button variant="ghost" size="sm" on:click={() => (selectedNodeId = null)}>
					선택 해제
				</Button>
			</div>
		</div>

			<div class="mt-4 grid gap-4 md:grid-cols-3">
				<div>
					<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">타입</p>
					<p class="mt-1 text-base font-medium text-slate-900 dark:text-slate-100">{typeLabels[selectedNode.type]}</p>
				</div>
				<div>
					<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">이름</p>
					<p class="mt-1 text-base font-medium text-slate-900 dark:text-slate-100">{selectedNode.label}</p>
				</div>
				<div>
					<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">연결된 노드 수</p>
					<p class="mt-1 text-base font-medium text-slate-900 dark:text-slate-100">{selectedConnections.length}</p>
				</div>
			</div>

			{#if isDeviceNode(selectedNode)}
				<div class="mt-4 grid gap-4 md:grid-cols-3">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">디바이스 타입</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.device_type || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">제조사</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.manufacturer || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">모델</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.model || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">상태</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.status || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">소스</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.source_type || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">외부 연동</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">
							{selectedNode.data.external_api_connection_id ?? '-'}
						</p>
					</div>
				</div>
			{:else if isLibraryNode(selectedNode)}
				<div class="mt-4 grid gap-4 md:grid-cols-3">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">디바이스 타입</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.device_type || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">제조사</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.manufacturer || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">모델</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.model || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">기본 랙 크기</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">
							{selectedNode.data.default_rack_size ?? '-'}
						</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">기본 전력</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">
							{selectedNode.data.default_power_consumption ?? '-'}
						</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">소스</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.source_type || '-'}</p>
					</div>
				</div>
			{:else if isContactNode(selectedNode)}
				<div class="mt-4 grid gap-4 md:grid-cols-3">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">직함</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.title || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">부서</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.department || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">이메일</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.email || '-'}</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">전화</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">
							{selectedNode.data.phone || selectedNode.data.mobile || '-'}
						</p>
					</div>
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">소스</p>
						<p class="mt-1 text-base text-slate-800 dark:text-slate-200">{selectedNode.data.source_type || '-'}</p>
					</div>
				</div>

				{#if selectedNode.data.responsibilities}
					<div class="mt-4 rounded-lg bg-slate-50 p-4 text-sm leading-relaxed text-slate-600 dark:bg-slate-800 dark:text-slate-300">
						<span class="block text-xs font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">
							Responsibilities
						</span>
						<p class="mt-1">{selectedNode.data.responsibilities}</p>
					</div>
				{/if}
			{/if}

			<div class="mt-6">
				<p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">연결된 노드</p>
				{#if selectedConnections.length}
					<div class="mt-3 grid gap-2 sm:grid-cols-2">
						{#each selectedConnections as connection}
							<div class="flex items-center justify-between rounded-lg border border-slate-200 bg-slate-50 px-3 py-2 text-xs text-slate-600 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-300">
								<span class="font-medium">{truncateLabel(connection.label, 26)}</span>
								<span class="rounded-md bg-white/60 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:bg-slate-900/60 dark:text-slate-400">
									{typeLabels[connection.type]}
								</span>
							</div>
						{/each}
					</div>
				{:else}
					<p class="mt-3 text-sm text-slate-500 dark:text-slate-400">연결된 노드가 없습니다.</p>
				{/if}
			</div>
		</div>
	{/if}

	<div class="grid gap-4 md:grid-cols-3">
		{#each statsConfig as stat}
			<div class="rounded-xl bg-white p-4 shadow-sm dark:bg-slate-900">
				<div class="flex items-center gap-3">
					<div class="rounded-lg p-2" style={`background-color: ${withAlpha(stat.color, 0.15)}`}>
						<div class="h-5 w-5 rounded-full" style={`background-color: ${stat.color}`}></div>
					</div>
					<div>
						<p class="text-sm text-slate-500 dark:text-slate-400">{stat.label}</p>
						<p class="text-2xl font-semibold text-slate-900 dark:text-slate-100">{getStats(stat.type)}</p>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
