<script lang="ts">
	export let label: string;

	const COLORS: Record<string, string> = {
		A: '#15803d',
		P: '#1d4ed8',
		S: '#1d4ed8',
		F: '#b91c1c',
		I: '#475569',
		default: '#334155'
	};

	const BACKGROUNDS: Record<string, string> = {
		A: '#bbf7d0',
		P: '#bfdbfe',
		S: '#bfdbfe',
		F: '#fecaca',
		I: '#e2e8f0',
		default: '#e0f2fe'
	};

	$: background = BACKGROUNDS[label] ?? BACKGROUNDS.default;
	$: color = COLORS[label] ?? COLORS.default;
</script>

<span class="chip" style={`background:${background}; color:${color};`}>{label}</span>

<style>
	.chip {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 2px 6px;
		border-radius: 999px;
		font-size: 10px;
		line-height: 1.1;
		font-weight: 600;
		letter-spacing: 0.02em;
		text-transform: uppercase;
		box-shadow: inset 0 0 0 1px rgba(15, 23, 42, 0.08);
	}
</style>
