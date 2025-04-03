<script lang="ts">
	import Chart from "./Chart.svelte";
	import Graph from "./Graph.svelte";
	import Onion from "./Onion.svelte";
	import { berlinNodes, berlinEdges } from "../example/berlin52"; //TODO: Handle permutation
	import { bitstringToOnionCoords } from "../lib/onion";
	import { parsePermutation } from "../lib/graph";
	import type { Task } from "../types/task";
	import type { DataPoint } from "../types/chart";
	import type { Point } from "../types/types";

	interface DashboardProps {
		serverURL: string;
		task: Task;
	}

	let { serverURL, task }: DashboardProps = $props();
	var socket: WebSocket;

	let dataPoints: DataPoint[] = $state([]);
	let onionPoints: Point[] = $state([]);
	let edges = $state(berlinEdges);

	interface SimulationUpdate {
		iterations: number;
		current_fitness: number;
		current_solution: string;
	}

	async function setupWebsocket() {
		const wsURL = `${serverURL}/ws/${task.id}`;
		socket = new WebSocket(wsURL);

		socket.onopen = (event) => {
			//TODO: Show loading before connection opens
			console.log(event);
		};

		socket.onclose = (event) => {
			//TODO: Show simulation is completed
			console.log(event);
		};

		socket.onerror = (event) => {
			//TODO: Handle error
			console.log(event);
		};

		socket.onmessage = (event) => {
			try {
				const message = JSON.parse(event.data) as SimulationUpdate;
				dataPoints = [
					...dataPoints,
					{
						iteration: message.iterations,
						fitness: message.current_fitness,
					},
				];
				if (isBitstringProblem) {
					const p = bitstringToOnionCoords(message.current_solution);
					onionPoints = [...onionPoints, p];
				} else if (isPermutationProblem) {
					edges = parsePermutation(message.current_solution);
				}
				//TODO: Handle permutation
			} catch (error) {
				//TODO: Handle error
				console.log(error);
			}
		};
	}

	setupWebsocket();

	const isBitstringProblem = ["OneMax", "LeadingOnes"].includes(task.problem);
	const isPermutationProblem = ["TSP"].includes(task.problem);
</script>

<p>Task ID: {task.id}</p>
<div class="grid grid-cols-2">
	<div class="bg-red-100 max-h-120">
		<Chart {dataPoints} />
	</div>
	<div class="bg-blue-100 max-h-120">
		{#if isBitstringProblem}
			<Onion pointData={onionPoints} />
		{:else if isPermutationProblem}
			<Graph nodes={berlinNodes} {edges} />
		{:else}
			<p>Invalid problem. No visualization to show.</p>
		{/if}
	</div>
</div>
