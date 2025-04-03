<script lang="ts">
	import { onMount } from "svelte";
	import Chart, { type ChartConfiguration } from "chart.js/auto";
	import type { DataPoint } from "../types/chart.ts";

	let chart: Chart;
	let canvas: HTMLCanvasElement;

	interface ChartProps {
		dataPoints: DataPoint[];
	}

	let { dataPoints }: ChartProps = $props();

	const updateChart = () => {
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		const config: ChartConfiguration = {
			type: "line",
			data: {
				labels: dataPoints.map((d) => d.iteration.toString()),
				datasets: [
					{
						label: "Fitness",
						data: dataPoints.map((d) => d.fitness),
						borderColor: "blue",
						borderWidth: 2,
						fill: false,
					},
				],
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
				animation: false,
				scales: {
					x: {
						title: { display: true, text: "Iteration" },
					},
					y: {
						title: { display: true, text: "Fitness" },
					},
				},
			},
		};

		chart = new Chart(ctx, config);
	};

	onMount(() => {
		updateChart();
	});

	$effect(() => {
		if (chart && dataPoints) {
			chart.data.labels = dataPoints.map((d) => d.iteration.toString());
			chart.data.datasets[0].data = dataPoints.map((d) => d.fitness);
			chart.update();
		}
	});

</script>

<canvas bind:this={canvas}></canvas>
