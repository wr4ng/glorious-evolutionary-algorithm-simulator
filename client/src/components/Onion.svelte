<script lang="ts">
	import {
		generateGaussPoints,
		generateGaussPath,
		mapPercentageToView,
	} from "../lib/onion";
	import type { OnionPoint } from "../types/onion";

	interface OnionProps {
		pointData: OnionPoint[];
	}

	const { pointData }: OnionProps = $props();
	let pointDataView: OnionPoint[] = $state([]);

	const gaussPoints = generateGaussPoints();
	const gaussPath = generateGaussPath(gaussPoints);

	$effect(() => {
		pointDataView = pointData.map((p) => mapPercentageToView(p));
	});

	let hoveredPoint: OnionPoint | null = $state(null);
	let tooltipX = $state(0);
	let tooltipY = $state(0);

	function showTooltip(event: MouseEvent, point: OnionPoint) {
		hoveredPoint = point;
		const svg = (event.currentTarget as SVGCircleElement).ownerSVGElement;
		if (svg) {
			const pt = svg.createSVGPoint();
			pt.x = event.clientX;
			pt.y = event.clientY;
			const screenCTM = svg.getScreenCTM();
			if (screenCTM) {
				tooltipX = event.clientX + 20;
				tooltipY = event.clientY;
			}
		}
	}

	function hideTooltip() {
		hoveredPoint = null;
	}
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
			<circle
				cx={p.x}
				cy={p.y}
				r="1.25"
				class="node"
				role="tooltip"
				onmouseenter={(e) => showTooltip(e, p)}
				onmousemove={(e) => showTooltip(e, p)}
				onmouseleave={hideTooltip}
			/>
		{/each}
	</svg>
	{#if hoveredPoint}
		<div
			class="tooltip"
			style="position: absolute; left: {tooltipX}px; top: {tooltipY}px;"
		>
			{hoveredPoint.tooltip}
		</div>
	{/if}
</div>

<style>
	.node {
		fill: blue;
		cursor: pointer;
	}

	.tooltip {
		background-color: rgba(0, 0, 0, 0.75);
		color: white;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 16px;
		pointer-events: none;
		white-space: nowrap;
	}
</style>
