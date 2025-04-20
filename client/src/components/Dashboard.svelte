<script lang="ts">
	import Chart from "./Chart.svelte";
	import Graph from "./Graph.svelte";
	import Onion from "./Onion.svelte";
	import type { Task } from "../types/task";
	import type { Node, Edge, Point } from "../types/types";
	import type { Series } from "../types/chart";
	import { bitstringToOnionCoords } from "../lib/onion";
	import { parsePermutation } from "../lib/graph";
	import { parseEUC2D } from "../lib/tsp";

	interface DashboardProps {
		serverURL: string;
		task: Task;
		back: () => void;
		rerun: () => void;
	}

	let { serverURL, task, back, rerun }: DashboardProps = $props();
	var socket: WebSocket;
	var status = $state("Disconnected...");

	let onionPoints: Point[] = $state([]);
	let nodes: Node[] = $state([]);
	let edges: Edge[] = $state([]);

	let iterations: number[] = $state([]);
	let fitness: number[] = $state([]);
	let temperature: number[] = $state([]);

	interface SimulationUpdate {
		iterations: number;
		current_fitness: number;
		current_solution: string;
		temperature: number | undefined;
	}

	function buildSeries(): Series[] {
		let series: Series[] = [
			{
				data: [...fitness],
				label: "Fitness",
				color: "blue",
				yAxisID: "yfit",
			},
		];
		if (hasTemp) {
			series.push({
				data: [...temperature],
				label: "Temperature",
				color: "red",
				yAxisID: "ytemp",
			});
		}
		return series;
	}

	async function setupWebsocket() {
		const wsURL = `${serverURL}/ws/${task.id}`;
		socket = new WebSocket(wsURL);

		socket.onopen = (event) => {
			status = "Connected";
			//TODO: Show loading before connection opens
			console.log(event);
		};

		socket.onclose = (event) => {
			status = "Disconnected...";
			//TODO: Show simulation is completed
			console.log(event);
		};

		socket.onerror = (event) => {
			status = "Disconnected...";
			//TODO: Handle error
			console.log(event);
		};

		socket.onmessage = (event) => {
			try {
				const message = JSON.parse(event.data) as SimulationUpdate;

				iterations = [...iterations, message.iterations];
				fitness = [...fitness, message.current_fitness];

				if (message.temperature) {
					temperature = [...temperature, message.temperature];
				}

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

	function handleBack() {
		if (socket) {
			socket.close();
		}
		back();
	}

	const isBitstringProblem = ["OneMax", "LeadingOnes"].includes(task.problem);
	const isPermutationProblem = ["TSP"].includes(task.problem);
	const hasTemp = task.algorithm == "SimulatedAnnealing";

	if (task.problem == "TSP" && task.tsp_instance) {
		nodes = parseEUC2D(task.tsp_instance);
	}

	setupWebsocket();
</script>

<div class="flex flex-col p-2 space-y-4">
	<div>
		<h1 class="text-2xl font-bold">Stats</h1>
		<p>Task ID: {task.id}</p>
		<p>Status: {status}</p>
		<p>Iteration: {iterations[iterations.length - 1]}</p>
		<p>Fitness: {fitness[fitness.length - 1]}</p>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Visualizations</h1>
		<div class="grid grid-cols-2 gap-2">
			<div class="border rounded-lg max-h-120">
				<Chart labels={[...iterations]} series={buildSeries()} />
			</div>
			<div class="border rounded-lg max-h-120">
				{#if isBitstringProblem}
					<Onion pointData={onionPoints} />
				{:else if isPermutationProblem}
					<Graph {nodes} {edges} />
				{:else}
					<p>Invalid problem. No visualization to show.</p>
				{/if}
			</div>
		</div>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Controls</h1>
		{#if status == "Disconnected..."}
			<button
				onclick={rerun}
				class="border rounded-lg px-4 py-2 font-bold"
			>
				Rerun Task
			</button>
		{/if}
		<button
			onclick={handleBack}
			class="border rounded-lg px-4 py-2 font-bold"
		>
			Back
		</button>
	</div>
</div>
