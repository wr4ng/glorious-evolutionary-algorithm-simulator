<script lang="ts">
	import { berlin52EUC2D, parseEUC2D } from "../lib/tsp";
	import type { Task, TaskScheduleRequest } from "../types/task";
	import TaskText from "./TaskText.svelte";

	interface TaskCreateFormProps {
		onSubmit: (request: TaskScheduleRequest) => void;
		error: string;
	}

	let { onSubmit, error }: TaskCreateFormProps = $props();

	const problemOptions = ["OneMax", "LeadingOnes", "TSP"];
	const algorithmOptions = [
		{ text: "(1+1) EA", value: "OnePlusOneEA" },
		{ text: "Simulated Annealing", value: "SimulatedAnnealing" },
		{ text: "Ant Colony Optimization (ACO)", value: "ACO" },
	];
	const tspInstanceOptions = ["berlin52", "Custom"];
	const scheduleOptions = ["Static", "Exponential"];

	let problem = $state("OneMax");
	let bitstringSize = $state(1000);
	let tspInstance = $state("berlin52");
	let customTspInstance = $state("");
	let customTspInstanceError = $state("");
	let customTspInstanceValidated = $state(false);

	let algorithm = $state("OnePlusOneEA");

	let scheduleType = $state("Exponential");
	let coolingRate = $state(1.0);
	let staticTemperature = $state(0.0);

	let maxIterations = $state(1000000);
	let optimalFitness: number | undefined = $state(undefined);

	let tasks: Task[] = $state([]);

	function isBitstringProblem(problem: string) {
		return problem == "OneMax" || problem == "LeadingOnes";
	}

	function isTSP(problem: string) {
		return problem == "TSP";
	}

	function validateCustomTSP() {
		try {
			parseEUC2D(customTspInstance);
			customTspInstanceValidated = true;
		} catch (error) {
			console.log(error);
			customTspInstanceError = "Invalid EUC2D instance";
		}
	}

	async function handleSumbit(e: SubmitEvent) {
		e.preventDefault();
		if (tasks.length == 0) {
			return;
		}
		onSubmit({
			tasks: tasks,
		});
	}

	function addCurrentTask() {
		let task: Task = {
			problem: {
				type: problem,
				...(isBitstringProblem(problem) && {
					bitstring_size: bitstringSize,
				}),
				...(isTSP(problem) && {
					tsp_instance:
						tspInstance == "berlin52"
							? berlin52EUC2D
							: customTspInstance,
				}),
			},
			algorithm: {
				type: algorithm,
				...(algorithm == "SimulatedAnnealing" && {
					cooling_schedule: {
						type: scheduleType,
						...(scheduleType == "Static" && {
							temperature: staticTemperature,
						}),
						...(scheduleType == "Exponential" && {
							cooling_rate: coolingRate,
						}),
					},
				}),
			},
			stop_cond: {
				max_iterations: maxIterations,
				...(optimalFitness && {
					optimal_fitness: optimalFitness,
				}),
			},
		};
		tasks = [...tasks, task];
	}

	function clearSchedule() {
		tasks = [];
	}
</script>

<form onsubmit={handleSumbit} class="flex flex-col space-y-4">
	<div class="flex flex-col space-y-2">
		<h1 class="text-xl font-bold">Problem</h1>
		<label class="flex flex-col">
			Problem:
			<select required bind:value={problem} class="border rounded">
				{#each problemOptions as option}
					<option value={option}>{option}</option>
				{/each}
			</select>
		</label>
		{#if isBitstringProblem(problem)}
			<label class="flex flex-col">
				Bitstring Size:
				<input
					type="number"
					step="1"
					bind:value={bitstringSize}
					required
					class="border rounded px-1"
				/>
			</label>
		{/if}
		{#if isTSP(problem)}
			<label class="flex flex-col">
				TSP Instance:
				<select
					required
					bind:value={tspInstance}
					class="border rounded"
				>
					{#each tspInstanceOptions as option}
						<option value={option}>{option}</option>
					{/each}
				</select>
			</label>
			{#if tspInstance == "Custom"}
				<label class="flex flex-col">
					Custom TSP Instance (EUC2D Format):
					<textarea
						bind:value={customTspInstance}
						oninput={() => {
							customTspInstanceError = "";
							customTspInstanceValidated = false;
						}}
						class="border rounded p-1"
						placeholder="Enter TSP instance..."
					></textarea>
				</label>
				{#if customTspInstanceError}
					<span class="text-red-500 font-bold"
						>{customTspInstanceError}</span
					>
				{/if}
				{#if customTspInstanceValidated}
					<span class="text-green-500 font-bold">Valid</span>
				{/if}
				<button
					type="button"
					onclick={validateCustomTSP}
					class="border rounded-lg py-2 font-bold"
				>
					Validate TSP Instance
				</button>
			{/if}
		{/if}
	</div>
	<div class="flex flex-col space-y-2">
		<h1 class="text-xl font-bold">Algoritm</h1>
		<label class="flex flex-col">
			Algorithm:
			<select bind:value={algorithm} class="border rounded">
				{#each algorithmOptions as option}
					<option value={option.value}>{option.text}</option>
				{/each}
			</select>
		</label>
		{#if algorithm == "SimulatedAnnealing"}
			<label class="flex flex-col">
				Cooling Schedule Type:
				<select bind:value={scheduleType} class="border rounded">
					{#each scheduleOptions as option}
						<option value={option}>{option}</option>
					{/each}
				</select>
			</label>
			{#if scheduleType == "Static"}
				<label class="flex flex-col">
					Static Temperature:
					<input
						type="number"
						step="any"
						min="0"
						required
						bind:value={staticTemperature}
						class="border rounded px-1"
					/>
				</label>
			{/if}
			{#if scheduleType == "Exponential"}
				<label class="flex flex-col">
					Cooling rate (c):
					<input
						type="number"
						step="any"
						min="0"
						required
						bind:value={coolingRate}
						class="border rounded px-1"
					/>
				</label>
			{/if}
		{/if}
	</div>
	<div class="flex flex-col space-y-2">
		<h1 class="text-xl font-bold">Stop Condition</h1>
		<label class="flex flex-col">
			Max Iterations:
			<input
				type="number"
				min="1"
				step="1"
				bind:value={maxIterations}
				required
				class="border rounded px-1"
			/>
		</label>
		<label class="flex flex-col">
			Optimal Fitness (optional):
			<input
				type="number"
				step="any"
				bind:value={optimalFitness}
				class="border rounded px-1"
			/>
		</label>
	</div>
	{#if error}
		<span class="text-red-500 font-bold">{error}</span>
	{/if}
	<button
		type="button"
		class="border rounded-lg py-2 font-bold"
		onclick={addCurrentTask}
	>
		Add Task
	</button>
	{#if tasks.length > 0}
		<button
			type="button"
			class="border rounded-lg p-2 font-bold"
			onclick={clearSchedule}
		>
			Clear Schedule
		</button>
		<button type="submit" class="border rounded-lg p-2 font-bold">
			Create Schedule
		</button>
	{/if}
	<h1 class="text-xl font-bold">Current Schedule</h1>
	{#each tasks as task}
		<TaskText {task} />
	{/each}
</form>
