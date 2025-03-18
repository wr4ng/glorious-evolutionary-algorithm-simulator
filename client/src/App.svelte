<script lang="ts">
	import Chart from "./lib/Chart.svelte";
	import Graph from "./lib/Graph.svelte";
	import Onion from "./lib/Onion.svelte";
	import { nodes, edges } from "./example/berlin52.ts";
	//import Main from "./lib/Main.svelte";

	let dataPoints = $state([
		{ iteration: 0, fitness: 100 },
		{ iteration: 1000, fitness: 120 },
		{ iteration: 2000, fitness: 125 },
	]);

	const pointData = [
		{ x: 0.5, y: 0.5 },
		{ x: 0.2, y: 0.6 },
		{ x: 0.8, y: 0.7 },
		{ x: 1, y: 0.7 },
		{ x: 1, y: 1 },
		{ x: 1, y: 0 },
	];

</script>

<div class="grid grid-cols-2">
	<div class="bg-red-100 max-h-120">
		<Chart {dataPoints} />
	</div>
	<div class="bg-blue-100 max-h-120 p-4">
		<Graph {nodes} {edges} />
	</div>
	<div class="bg-green-100 max-h-120">
		<Onion {pointData} />
	</div>
	<div class="bg-orange-100">Buttons...</div>
</div>
