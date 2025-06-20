<script lang="ts">
	import type { Node, Edge } from "../types/types";

	interface GraphProps {
		nodes: Node[];
		edges: Edge[];
		pheromones: number[][];
		t_max: number;
		t_min: number;
	}

	let { nodes, edges, pheromones, t_max, t_min }: GraphProps = $props();

	let viewBox = $state("0 0 100 100");
	let minY = $state(0);
	let maxY = $state(100);

	function updateDimensions(node: HTMLElement) {
		const resizeObserver = new ResizeObserver(() => calculateViewbox());
		resizeObserver.observe(node);
		return {
			destroy() {
				resizeObserver.unobserve(node);
			},
		};
	}

	function calculateViewbox() {
		// Calculate viewBox based on nodes
		if (nodes.length > 0) {
			const minX = Math.min(...nodes.map((n) => n.x));
			const maxX = Math.max(...nodes.map((n) => n.x));
			minY = Math.min(...nodes.map((n) => n.y));
			maxY = Math.max(...nodes.map((n) => n.y));

			const padding = Math.max(maxX - minX, maxY - minY) * 0.05;
			viewBox = `${minX - padding} ${minY - padding} ${maxX - minX + 2 * padding} ${maxY - minY + 2 * padding}`;
		}
	}

	function mapRange(
		value: number,
		fromMin: number,
		fromMax: number,
		toMin: number,
		toMax: number,
	) {
		return (
			((value - fromMin) / (fromMax - fromMin)) * (toMax - toMin) + toMin
		);
	}

	function flipY(y: number) {
		return maxY - (y - minY);
	}

	$effect(() => {
		// Recalculate viewbox when nodes/edges are updated (switching current task)
		calculateViewbox();
	});
</script>

<div class="graph-container" use:updateDimensions>
	<svg {viewBox} preserveAspectRatio="xMidYMid meet">
		{#each edges as edge}
			<line
				x1={nodes[edge.source].x}
				y1={flipY(nodes[edge.source].y)}
				x2={nodes[edge.target].x}
				y2={flipY(nodes[edge.target].y)}
				class="edge"
				vector-effect="non-scaling-stroke"
				stroke-width="1.5"
			/>
		{/each}
		{#each pheromones as phero, i}
			{#each phero as value, j}
				{#if j > i}
					<line
						x1={nodes[i].x}
						y1={flipY(nodes[i].y)}
						x2={nodes[j].x}
						y2={flipY(nodes[j].y)}
						stroke-opacity="{mapRange(
							value,
							t_min,
							t_max,
							10,
							50,
						)}%"
						vector-effect="non-scaling-stroke"
						stroke="red"
						stroke-width={mapRange(value, t_min, t_max, 0.2, 1.5)}
					/>
				{/if}
			{/each}
		{/each}
		{#each nodes as node}
			<circle cx={node.x} cy={flipY(node.y)} r="0.5%" class="node" />
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
		display: inline-block;
	}

	.edge {
		stroke: #666;
	}

	.node {
		fill: #0066cc;
	}
</style>
