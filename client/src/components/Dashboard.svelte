<script lang="ts">
	import Chart from "./Chart.svelte";
	import Graph from "./Graph.svelte";
	import Onion from "./Onion.svelte";
	import type { Algorithm, Problem, Task, TaskSchedule } from "../types/task";
	import type { Node, Edge, Point } from "../types/types";
	import type { Series } from "../types/chart";
	import { bitstringToOnionCoords } from "../lib/onion";
	import { parsePermutation } from "../lib/graph";
	import { parseEUC2D } from "../lib/tsp";

	interface DashboardProps {
		serverURL: string;
		taskSchedule: TaskSchedule;
		back: () => void;
	}

	let { serverURL, taskSchedule, back }: DashboardProps = $props();
	var socket: WebSocket;
	var status = $state("Disconnected...");

	let currentTask: Task | null = $state(null);
	let results: Result[] = $state([]);

	let onionPoints: Point[] = $state([]);
	let nodes: Node[] = $state([]);
	let edges: Edge[] = $state([]);

	let iterations: number[] = $state([]);
	let fitness: number[] = $state([]);
	let temperature: number[] = $state([]);

	// Message types
	interface Message {
		messageType: string;
		task: Task | null;
		data: SimulationUpdate | null;
		result: Result | null;
	}

	interface SimulationUpdate {
		iterations: number;
		current_fitness: number;
		current_solution: string;
		temperature: number | undefined;
	}

	interface Result {
		task: Task;
		fitness: number;
		iterations: number;
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
		if (hasTemp(currentTask!.algorithm)) {
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
		const wsURL = `${serverURL}/ws/${taskSchedule.id}`;
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
				const message = JSON.parse(event.data) as Message;

				if (message.messageType == "setTask" && message.task) {
					currentTask = message.task;
					clearData();
					if (
						currentTask.problem.type == "TSP" &&
						currentTask.problem.tsp_instance
					) {
						nodes = parseEUC2D(currentTask.problem.tsp_instance);
					}
					return;
				}

				if (message.messageType == "dataUpdate" && message.data) {
					iterations = [...iterations, message.data.iterations];
					fitness = [...fitness, message.data.current_fitness];

					if (message.data.temperature) {
						temperature = [
							...temperature,
							message.data.temperature,
						];
					}

					if (isBitstringProblem(currentTask!.problem)) {
						const p = bitstringToOnionCoords(
							message.data.current_solution,
						);
						onionPoints = [...onionPoints, p];
					} else if (isPermutationProblem(currentTask!.problem)) {
						edges = parsePermutation(message.data.current_solution);
					}
					return;
				}

				if (message.messageType == "result" && message.result) {
					results = [...results, message.result];
					return;
				}
				console.log("invalid message"); //TODO: Throw error
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

	function clearData() {
		iterations = [];
		fitness = [];
		temperature = [];
		onionPoints = [];
		nodes = [];
		edges = [];
	}

	const isBitstringProblem = (problem: Problem) =>
		["OneMax", "LeadingOnes"].includes(problem.type);
	const isPermutationProblem = (problem: Problem) =>
		["TSP"].includes(problem.type);
	const hasTemp = (algorithm: Algorithm) =>
		algorithm.type == "SimulatedAnnealing";

	setupWebsocket();
</script>

<div class="flex flex-col p-2 space-y-4">
	<div>
		<h1 class="text-2xl font-bold">Stats</h1>
		<p>Task ID: {taskSchedule.id}</p>
		<p>Status: {status}</p>
	</div>
	{#if currentTask}
		<div>
			<h1 class="text-2xl font-bold">Current Task</h1>
			<p>Iteration: {iterations[iterations.length - 1]}</p>
			<p>Fitness: {fitness[fitness.length - 1]}</p>
		</div>
		<div>
			<h1 class="text-2xl font-bold">Visualizations</h1>
			<div class="grid grid-cols-2 gap-2">
				<div class="border rounded-lg h-120">
					<Chart labels={[...iterations]} series={buildSeries()} />
				</div>
				<div class="h-120 border rounded-lg p-2">
					<p class="h-6 font-bold text-xl">Instance</p>
					<div class="h-110">
						{#if isBitstringProblem(currentTask.problem)}
							<Onion pointData={onionPoints} />
						{:else if isPermutationProblem(currentTask.problem)}
							<Graph {nodes} {edges} />
						{:else}
							<p>Invalid problem. No visualization to show.</p>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
	<div>
		<h1 class="text-2xl font-bold">Results</h1>
		{#each results as result}
			<p>
				{result.task.algorithm.type} - {result.iterations} - {result.fitness}
			</p>
		{/each}
	</div>
	<div>
		<h1 class="text-2xl font-bold">Controls</h1>
		<button
			onclick={handleBack}
			class="border rounded-lg px-4 py-2 font-bold"
		>
			Back
		</button>
	</div>
</div>
