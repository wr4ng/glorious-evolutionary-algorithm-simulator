<script lang="ts">
	import Chart from "./Chart.svelte";
	import Graph from "./Graph.svelte";
	import Onion from "./Onion.svelte";
	import { nodes, edges } from "../example/berlin52.ts";
	import type { Task } from "../types/task";
	import type { DataPoint } from "../types/chart.ts";

	interface DashboardProps {
		serverURL: string;
		task: Task;
	}

	let { serverURL, task }: DashboardProps = $props();
	var socket: WebSocket;

	let dataPoints: DataPoint[] = $state([]);

	interface Point {
		x: number;
		y: number;
	}

	let pointData: Point[] = $state([]);

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
				const onionPoint = bitstringToOnion(message.current_solution);
				pointData = [...pointData, onionPoint];
			} catch (error) {
				//TODO: Handle error
				console.log(error);
			}
		};
	}

	function bitstringToOnion(bitstring: string) {
		const numOnes = (bitstring.match(/1/g) || []).length;
		if (numOnes == bitstring.length) {
			return { x: 1, y: 1 };
		} else if (numOnes == 0) {
			return { x: 0, y: 0 };
		}
		const vertical = numOnes / bitstring.length;

		let averageOneIndex = 0;
		for (let i = 0; i < bitstring.length; i++) {
			if (bitstring[i] == "1") {
				averageOneIndex += bitstring.length - 1 - i;
			}
		}
		//TODO: Can simplify this using n(n+1)/2
		let minAverage = 0;
		let maxAverage = 0;
		for (let i = 0; i < numOnes; i++) {
			minAverage += i;
			maxAverage += bitstring.length - 1 - i;
		}
		// Map averageOneIndex from [minAverage; maxAverage] to [0; 1]
		let horizontal =
			(averageOneIndex - minAverage) / (maxAverage - minAverage);

		return { x: horizontal, y: vertical };
	}

	setupWebsocket();
</script>

<p>Task ID: {task.id}</p>
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
