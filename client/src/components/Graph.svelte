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

	let width = $state(0);
	let height = $state(0);
	let viewBox = $state("0 0 100 100");

	function pheromone_width(value: number){
		let top = value - t_min;
		let bot = t_max - t_min;
		let division = top / bot;
		let result = 1 + division * 9
		return (result)
	}

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
</script>

<div class="graph-container" use:updateDimensions>
	<svg {viewBox} preserveAspectRatio="xMidYMid meet">
		{#each edges as edge}
			<line 
				x1={nodes[edge.source].x}
				y1={nodes[edge.source].y}
				x2={nodes[edge.target].x}
				y2={nodes[edge.target].y}
				class="edge"
			/>
		{/each}
		{#each pheromones as phero,i}
			{#each phero as value, j}
				{#if j > i}
					<line 
						x1={nodes[i].x}
						y1={nodes[i].y}
						x2={nodes[j].x}
						y2={nodes[j].y}
						stroke-opacity="{20 + (value-t_min)/(t_max-t_min)*60}%"
						style="stroke:red;stroke-width:{pheromone_width(value)};"
					/>
				{/if}
			{/each}
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
		display: inline-block;
	}

	.edge {
		stroke: #666;
		stroke-width: 2;
	}

	.node {
		fill: #0066cc;
	}

</style>
