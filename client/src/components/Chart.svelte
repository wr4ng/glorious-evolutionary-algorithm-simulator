<script lang="ts">
	import { onMount } from "svelte";
	import Chart, { type ChartConfiguration } from "chart.js/auto";
	import type { Series } from "../types/chart";

	let chart: Chart;
	let canvas: HTMLCanvasElement;

	interface ChartProps {
		labels: number[];
		series: Series[];
	}

	let { labels, series }: ChartProps = $props();

	const updateChart = () => {
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		const config: ChartConfiguration = {
			type: "line",
			data: {
				labels: labels,
				datasets: series.map((s) => ({
					label: s.label,
					data: s.data,
					yAxisID: s.yAxisID,
					borderColor: s.color,
					borderWidth: 2,
					fill: false,
					pointRadius: 1,
				})),
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
				animation: false,
				scales: {
					x: {
						title: { display: true, text: "Iteration" },
					},
					yfit: {
						title: { display: true, text: "Fitness" },
					},
					ytemp: {
						title: { display: true, text: "Temperature" },
						position: "right",
						display: "auto",
						type: "logarithmic",
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
		if (chart && series) {
			chart.data.labels = labels;
			chart.data.datasets = series.map((s) => ({
				label: s.label,
				data: s.data,
				yAxisID: s.yAxisID,
				borderColor: s.color,
				borderWidth: 2,
				fill: false,
				pointRadius: 1,
			}));
			chart.update();
		}
	});
</script>

<canvas bind:this={canvas}></canvas>
