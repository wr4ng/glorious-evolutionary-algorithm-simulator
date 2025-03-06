<script lang="ts">
	import Chart from "./lib/Chart.svelte";
	import Graph from "./lib/Graph.svelte";
	//import Main from "./lib/Main.svelte";

	let dataPoints = $state([
		{ iteration: 0, fitness: 100 },
		{ iteration: 1000, fitness: 120 },
		{ iteration: 2000, fitness: 125 },
	]);

	let nodes = [
		{ id: "A", x: 500, y: 100 },
		{ id: "B", x: 105, y: 120 },
		{ id: "C", x: 35, y: 300 },
		{ id: "D", x: 24, y: 50 },
	];

	let edges = [
		{ source: "A", target: "B" },
		{ source: "A", target: "C" },
		{ source: "B", target: "C" },
		{ source: "D", target: "C" },
	];
</script>

<div class="grid grid-cols-2">
	<div class="bg-red-100 max-h-120">
		<Chart {dataPoints} />
	</div>
	<div class="bg-blue-100 max-h-120 p-4">
		<Graph {nodes} {edges} />
	</div>
	<div class="bg-green-100">Info</div>
	<div class="bg-orange-100">Buttons...</div>
</div>
