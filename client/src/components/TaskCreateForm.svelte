<script lang="ts">
	import { berlin52EUC2D, parseEUC2D } from "../lib/tsp";
	import type { Task } from "../types/task";
	import Button from "./ui/Button.svelte";

	interface TaskCreateFormProps {
		addTask: (t: Task) => void;
	}

	let { addTask }: TaskCreateFormProps = $props();

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

	let autoOptimalFitness: boolean = $state(true);

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
		addCurrentTask();
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
		addTask(task);
	}

	function checkAutoOptimalFitness() {
		if (autoOptimalFitness) {
			if (problem == "OneMax" || problem == "LeadingOnes") {
				optimalFitness = bitstringSize;
			} else if (problem == "TSP" && tspInstance == "berlin52") {
				optimalFitness = 7542;
			} else {
				optimalFitness = undefined;
			}
		}
	}
	checkAutoOptimalFitness();
</script>

<h1 class="mt-4 text-4xl font-extrabold">Create Task</h1>
<form
	oninput={checkAutoOptimalFitness}
	onchange={checkAutoOptimalFitness}
	onsubmit={handleSumbit}
	class="flex flex-col space-y-4"
>
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
				<Button
					text="Validate TSP Instance"
					type="button"
					onclick={validateCustomTSP}
				/>
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
		<p>Task stops when one of the following conditions are met:</p>
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
			<div class="flex gap-2">
				<input
					type="number"
					step="any"
					disabled={autoOptimalFitness}
					bind:value={optimalFitness}
					class="border rounded px-1 grow disabled:bg-gray-200"
				/>
				<label class="flex items-center gap-2">
					Auto Optimal Fitness:
					<input
						type="checkbox"
						bind:checked={autoOptimalFitness}
						class="w-4 h-4"
					/>
				</label>
			</div>
		</label>
	</div>
	<Button text="Add Task" type="submit" />
</form>
