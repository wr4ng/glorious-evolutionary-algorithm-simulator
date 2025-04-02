<script lang="ts">
	import { generateGaussPoints, generateGaussPath, mapPercentageToView, } from "../lib/onion";
	import type { Point } from "../types/types";

	interface OnionProps {
		pointData: Point[];
	}

	const { pointData }: OnionProps = $props();
	let pointDataView: Point[] = $state([]);

	const gaussPoints = generateGaussPoints();
	const gaussPath = generateGaussPath(gaussPoints);

	$effect(() => {
		pointDataView = pointData.map((p) => mapPercentageToView(p));
	});
</script>

<div style="width: 100%; height: 100%;">
	<svg
		width="100%"
		height="100%"
		viewBox="-5 -5 110 110"
		preserveAspectRatio="xMidYMid meet"
	>
		<line
			x1="50"
			y1="1"
			x2="50"
			y2="100"
			stroke="black"
			stroke-opacity="0.3"
			stroke-width="2"
			stroke-dasharray="5,5"
			vector-effect="non-scaling-stroke"
		/>
		<line
			x1="0"
			y1="50"
			x2="100"
			y2="50"
			stroke="black"
			stroke-width="2"
			stroke-opacity="0.3"
			stroke-dasharray="5,5"
			vector-effect="non-scaling-stroke"
		/>
		<path
			d={gaussPath}
			stroke="red"
			fill="none"
			stroke-width="3"
			vector-effect="non-scaling-stroke"
		/>
		{#each pointDataView as p}
			<circle cx={p.x} cy={p.y} r="1" class="node" />
		{/each}
	</svg>
</div>

<style>
	.node {
		fill: blue;
	}
</style>
