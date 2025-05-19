<script lang="ts">
	import Chart from "./Chart.svelte";
	import Graph from "./Graph.svelte";
	import Onion from "./Onion.svelte";
	import type {
		Algorithm,
		Problem,
		Task,
		TaskResult,
		TaskSchedule,
	} from "../types/task";
	import type { Node, Edge } from "../types/types";
	import type { Series } from "../types/chart";
	import { bitstringToOnionCoords } from "../lib/onion";
	import { parsePermutation } from "../lib/graph";
	import { parseEUC2D } from "../lib/tsp";
	import { taskToText } from "../lib/task";
	import { downloadCSV } from "../lib/download";
	import Button from "./ui/Button.svelte";
    import type { OnionPoint } from "../types/onion";

	interface DashboardProps {
		serverURL: string;
		taskSchedule: TaskSchedule;
		back: () => void;
	}

	let { serverURL, taskSchedule, back }: DashboardProps = $props();
	var socket: WebSocket;
	var status = $state("Disconnected");

	let showTemperature: boolean = $state(true);

	let tasks: Task[] = $state([]);
	let currentTaskIndex: number = $state(0);

	let results: TaskResult[] = $state([]);

	let iterations: number[][] = $state([]);
	let fitness: number[][] = $state([]);
	let temperature: number[][] = $state([]);
	let onionPoints: OnionPoint[][] = $state([]);
	let nodes: Node[][] = $state([]);
	let edges: Edge[][] = $state([]);

	// Message types
	interface Message {
		messageType: string;
		task: Task | null;
		data: SimulationUpdate | null;
		result: TaskResult | null;
	}

	interface SimulationUpdate {
		iterations: number;
		current_fitness: number;
		current_solution: string;
		temperature: number | undefined;
	}

	function buildSeries(i: number): Series[] {
		let series: Series[] = [
			{
				data: [...fitness[i]],
				label: "Fitness",
				color: "blue",
				yAxisID: "yfit",
			},
		];
		if (showTemperature && hasTemp(tasks[i].algorithm)) {
			series.push({
				data: [...temperature[i]],
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
			status = "Disconnected";
			//TODO: Show simulation is completed
			console.log(event);
		};

		socket.onerror = (event) => {
			status = "Disconnected";
			//TODO: Handle error
			console.log(event);
		};

		socket.onmessage = (event) => {
			try {
				const message = JSON.parse(event.data) as Message;

				if (message.messageType == "setTask" && message.task) {
					tasks = [...tasks, message.task];
					iterations = [...iterations, []];
					fitness = [...fitness, []];
					temperature = [...temperature, []];
					onionPoints = [...onionPoints, []];
					nodes = [...nodes, []];
					edges = [...edges, []];
					currentTaskIndex = tasks.length - 1;
					if (
						message.task.problem.type == "TSP" &&
						message.task.problem.tsp_instance
					) {
						nodes[currentTaskIndex] = parseEUC2D(
							message.task.problem.tsp_instance,
						);
					}
					return;
				}

				if (message.messageType == "dataUpdate" && message.data) {
					iterations[currentTaskIndex] = [
						...iterations[currentTaskIndex],
						message.data.iterations,
					];
					fitness[currentTaskIndex] = [
						...fitness[currentTaskIndex],
						message.data.current_fitness,
					];

					if (message.data.temperature) {
						temperature[currentTaskIndex] = [
							...temperature[currentTaskIndex],
							message.data.temperature,
						];
					}

					if (isBitstringProblem(tasks[currentTaskIndex].problem)) {
						const p = bitstringToOnionCoords(
							message.data.current_solution,
							`Iteration: ${message.data.iterations}, Fitness: ${message.data.current_fitness}`
						);
						onionPoints[currentTaskIndex] = [
							...onionPoints[currentTaskIndex],
							p,
						];
					} else if (
						isPermutationProblem(tasks[currentTaskIndex].problem)
					) {
						edges[currentTaskIndex] = parsePermutation(
							message.data.current_solution,
						);
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

	const isBitstringProblem = (problem: Problem) =>
		["OneMax", "LeadingOnes"].includes(problem.type);
	const isPermutationProblem = (problem: Problem) =>
		["TSP"].includes(problem.type);
	const hasTemp = (algorithm: Algorithm) =>
		algorithm.type == "SimulatedAnnealing";

	function downloadResults() {
		const header = "task, final_fitness, final_iterations\n";
		const content = results
			.map((taskResult) => {
				return `${taskToText(taskResult.task)}, ${taskResult.fitness}, ${taskResult.iterations}\n`;
			})
			.join("");

		downloadCSV(header + content, `data-${taskSchedule.id}`);
	}

	function downloadCurrentTaskData() {
		const currentTask = tasks[currentTaskIndex];
		let header = "";
		let content = "";

		if (hasTemp(currentTask.algorithm)) {
			header = "iteration, fitness, temperature\n";
			for (let i = 0; i < iterations[currentTaskIndex].length; i++) {
				content += `${iterations[currentTaskIndex][i]}, ${fitness[currentTaskIndex][i]}, ${temperature[currentTaskIndex][i]}\n`;
			}
		} else {
			header = "iteration, fitness\n";
			for (let i = 0; i < iterations[currentTaskIndex].length; i++) {
				content += `${iterations[currentTaskIndex][i]}, ${fitness[currentTaskIndex][i]}\n`;
			}
		}
		downloadCSV(
			header + content,
			`data-${taskSchedule.id}-task-${currentTaskIndex + 1}`,
		);
	}

	function updateTaskIndex(delta: number) {
		const newIndex = currentTaskIndex + delta;
		if (newIndex < 0) {
			currentTaskIndex = 0;
		} else if (newIndex >= tasks.length) {
			currentTaskIndex = tasks.length - 1;
		} else {
			currentTaskIndex = newIndex;
		}
	}

	setupWebsocket();
</script>

<div class="flex flex-col p-2 space-y-4">
	<div>
		<h1 class="text-2xl font-bold">Schedule</h1>
		<p><strong>Schedule ID:</strong> {taskSchedule.id}</p>
		<p><strong>Connection status:</strong> {status}</p>
	</div>
	{#if tasks.length > 0}
		<div class="flex flex-col gap-2">
			<h1 class="text-2xl font-bold">Current Task</h1>
			{#if status == "Disconnected"}
				<div class="flex gap-2 items-center">
					<Button
						text="<"
						disabled={currentTaskIndex == 0}
						onclick={() => updateTaskIndex(-1)}
					/>
					<p>Current task: {currentTaskIndex + 1}/{tasks.length}</p>
					<Button
						text=">"
						disabled={currentTaskIndex == tasks.length - 1}
						onclick={() => updateTaskIndex(1)}
					/>
				</div>
			{/if}
			<div>
				<p>
					Iteration: {iterations[currentTaskIndex][
						iterations[currentTaskIndex].length - 1
					]}
				</p>
				<p>
					Fitness: {fitness[currentTaskIndex][
						fitness[currentTaskIndex].length - 1
					]}
				</p>
			</div>
			{#if hasTemp(tasks[currentTaskIndex].algorithm)}
				<label class="flex items-center gap-2">
					Show temperature:
					<input
						type="checkbox"
						bind:checked={showTemperature}
						class="w-4 h-4"
					/>
				</label>
			{/if}
			{#if status == "Disconnected"}
				<div>
					<Button
						text="Download task data"
						type="button"
						onclick={downloadCurrentTaskData}
					/>
				</div>
			{/if}
			<div class="grid grid-cols-2 gap-2">
				<div class="border rounded-lg h-120">
					<Chart
						labels={[...iterations[currentTaskIndex]]}
						series={buildSeries(currentTaskIndex)}
					/>
				</div>
				<div class="h-120 border rounded-lg p-2">
					<p class="h-6 font-bold text-xl">Instance</p>
					<div class="h-110">
						{#if isBitstringProblem(tasks[currentTaskIndex].problem)}
							<Onion pointData={onionPoints[currentTaskIndex]} />
						{:else if isPermutationProblem(tasks[currentTaskIndex].problem)}
							<Graph
								nodes={nodes[currentTaskIndex]}
								edges={edges[currentTaskIndex]}
							/>
						{:else}
							<p>Invalid problem. No visualization to show.</p>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
	<div class="flex flex-col gap-2">
		<h1 class="text-2xl font-bold">Results</h1>
		<div>
			{#if results.length > 0}
				<Button
					text="Download Results"
					type="button"
					onclick={downloadResults}
				/>
			{/if}
		</div>
		<div
			class="text-gray-700 shadow-md bg-white border rounded-xl overflow-hidden"
		>
			<table class="w-full text-left table-auto">
				<thead>
					<tr class="bg-gray-100">
						<th class="p-2 border-b border-blue-gray-100">#</th>
						<th class="p-2 border-b border-blue-gray-100">Task</th>
						<th class="p-2 border-b border-blue-gray-100"
							>Final Fitness</th
						>
						<th class="p-2 border-b border-blue-gray-100"
							>Final Iterations</th
						>
						<th class="border-b border-blue-gray-100"></th>
					</tr>
				</thead>
				<tbody>
					{#each results as result, i}
						{#if i == currentTaskIndex}
							<tr class="bg-gray-200">
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									<strong>{i + 1}</strong>
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									<strong>{taskToText(result.task)}</strong>
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									{result.fitness}
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									{result.iterations}
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								></td>
							</tr>
						{:else}
							<tr class={i % 2 === 1 ? "bg-gray-50" : "bg-white"}>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									<strong>{i + 1}</strong>
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									<strong>{taskToText(result.task)}</strong>
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									{result.fitness}
								</td>
								<td
									class={"p-2 " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
								>
									{result.iterations}
								</td>
								<td
									class={"p-2 text-right shrink " +
										(i === results.length - 1
											? ""
											: "border-b border-blue-gray-50")}
									><Button
										text="Select"
										onclick={() => {
											currentTaskIndex = i;
										}}
										extraClass="py-0"
									/></td
								>
							</tr>
						{/if}
					{/each}
				</tbody>
			</table>
		</div>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Controls</h1>
		<Button text="Back" onclick={handleBack} extraClass="px-4" />
	</div>
</div>
