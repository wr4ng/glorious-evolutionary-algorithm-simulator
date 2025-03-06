<script lang="ts">
	interface Node {
		id: string;
		x: number;
		y: number;
	}

	interface Edge {
		source: string;
		target: string;
	}

	interface GraphProps {
		nodes: Node[];
		edges: Edge[];
	}

	let { nodes, edges }: GraphProps = $props();

	let width = $state(0);
	let height = $state(0);
	let viewBox = $state("0 0 100 100");

	function updateDimensions(node: HTMLElement) {
		const resizeObserver = new ResizeObserver((entries) => {
			const entry = entries[0];
			width = entry.contentRect.width;
			height = entry.contentRect.height;

			// Calculate viewBox based on nodes
			if (nodes.length > 0) {
				const padding = 10;
				const minX = Math.min(...nodes.map((n) => n.x)) - padding;
				const maxX = Math.max(...nodes.map((n) => n.x)) + padding;
				const minY = Math.min(...nodes.map((n) => n.y)) - padding;
				const maxY = Math.max(...nodes.map((n) => n.y)) + padding;
				viewBox = `${minX} ${minY} ${maxX - minX} ${maxY - minY}`;
			}
		});

		resizeObserver.observe(node);
		return {
			destroy() {
				resizeObserver.unobserve(node);
			},
		};
	}

	function getNode(id: string): Node | undefined {
		return nodes.find((n) => n.id === id);
	}
</script>

<div class="graph-container" use:updateDimensions>
	<svg {viewBox} preserveAspectRatio="xMidYMid meet">
		{#each edges as edge}
			{#if getNode(edge.source) && getNode(edge.target)}
				<line
					x1={getNode(edge.source)!.x}
					y1={getNode(edge.source)!.y}
					x2={getNode(edge.target)!.x}
					y2={getNode(edge.target)!.y}
					class="edge"
				/>
			{/if}
		{/each}
		{#each nodes as node}
			<circle cx={node.x} cy={node.y} r="5" class="node" />
		{/each}
	</svg>
</div>

<style>
	.graph-container {
		width: 100%;
		height: 100%;
	}

	svg {
		width: 100%;
		height: 100%;
	}

	.edge {
		stroke: #666;
		stroke-width: 2;
	}

	.node {
		fill: #0066cc;
	}
</style>
