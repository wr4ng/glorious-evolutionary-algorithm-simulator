<script lang="ts">
	import { berlin52EUC2D, bier127EUC2D, parseEUC2D } from "../lib/tsp";
	import type { Task } from "../types/task";
	import Button from "./ui/Button.svelte";

	interface TaskCreateFormProps {
		addTask: (t: Task) => void;
		previousTask: Task | null;
	}

	let { addTask, previousTask }: TaskCreateFormProps = $props();

	const problemOptions = ["OneMax", "LeadingOnes", "TSP"];
	const algorithmOptions = [
		{ text: "(1+1) EA", value: "OnePlusOneEA" },
		{ text: "Simulated Annealing", value: "SimulatedAnnealing" },
		{ text: "Ant Colony Optimization (ACO)", value: "ACO" },
	];
	const tspInstanceOptions = ["berlin52", "bier127", "Custom"];
	const scheduleOptions = ["Static", "Exponential"];
	const strategyOptions = ["BestSoFar", "GenerationBest", "AllAnts"];

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

	let alpha = $state(1.0);
	let beta = $state(1.0);
	let evap_factor = $state(0.5);
	let ants = $state(1);
	let fit_based = $state(false);
	let fit_based_update = $state(false);
	let p_best = $state(0.05);
	let q = $state(0.0);
	let nearest_neighbor: boolean = $state(false);
	let updateStrategy: string = $state(strategyOptions[0]);

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

	function getTspInstance() {
		if (tspInstance == "berlin52") {
			return berlin52EUC2D;
		}
		if (tspInstance == "bier127") {
			return bier127EUC2D;
		}
		return customTspInstance;
	}

	function addCurrentTask() {
		let task: Task = {
			problem: {
				type: problem,
				...(isBitstringProblem(problem) && {
					bitstring_size: bitstringSize,
				}),
				...(isTSP(problem) && {
					tsp_instance: getTspInstance(),
					tsp_name: tspInstance,
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
				...(algorithm == "ACO" && {
					alpha: alpha,
					beta: beta,
					evap_factor: evap_factor,
					ants: ants,
					...(fit_based && {
						p_best: p_best,
					}),
					...(fit_based_update && {
						q: q,
					}),
					nn: nearest_neighbor,
					update_strategy: updateStrategy,
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

	function loadPreviousTask() {
		if (!previousTask) return;
		problem = previousTask.problem.type;

		if (previousTask.problem.bitstring_size) {
			bitstringSize = previousTask.problem.bitstring_size;
		}
		if (previousTask.problem.tsp_name) {
			tspInstance = previousTask.problem.tsp_name;
			if (tspInstance == "Custom") {
				customTspInstance = previousTask.problem.tsp_instance
					? previousTask.problem.tsp_instance
					: "";
			}
		}

		algorithm = previousTask.algorithm.type;
		if (previousTask.algorithm.cooling_schedule) {
			scheduleType = previousTask.algorithm.cooling_schedule.type;
			if (previousTask.algorithm.cooling_schedule.temperature) {
				staticTemperature =
					previousTask.algorithm.cooling_schedule.temperature;
			}
			if (previousTask.algorithm.cooling_schedule.cooling_rate) {
				coolingRate =
					previousTask.algorithm.cooling_schedule.cooling_rate;
			}
		}

		if (previousTask.algorithm.alpha) {
			alpha = previousTask.algorithm.alpha;
		}
		if (previousTask.algorithm.beta) {
			beta = previousTask.algorithm.beta;
		}
		if (previousTask.algorithm.evap_factor) {
			evap_factor = previousTask.algorithm.evap_factor;
		}
		if (previousTask.algorithm.ants) {
			ants = previousTask.algorithm.ants;
		}
		if (previousTask.algorithm.p_best) {
			fit_based = true;
			p_best = previousTask.algorithm.p_best;
		}
		if (previousTask.algorithm.q) {
			fit_based_update = true;
			q = previousTask.algorithm.q;
		}
		if (previousTask.algorithm.nn) {
			nearest_neighbor = previousTask.algorithm.nn;
		}
		if (previousTask.algorithm.update_strategy) {
			updateStrategy = previousTask.algorithm.update_strategy;
		}

		maxIterations = previousTask.stop_cond.max_iterations;
		if (previousTask.stop_cond.optimal_fitness) {
			autoOptimalFitness = false;
			optimalFitness = previousTask.stop_cond.optimal_fitness;
		}
	}

	function checkAutoOptimalFitness() {
		if (autoOptimalFitness) {
			if (problem == "OneMax" || problem == "LeadingOnes") {
				optimalFitness = bitstringSize;
			} else if (problem == "TSP" && tspInstance == "berlin52") {
				optimalFitness = 7542;
			} else if (problem == "TSP" && tspInstance == "bier127") {
				optimalFitness = 118282;
			} else {
				optimalFitness = undefined;
			}
		}
	}

	loadPreviousTask();
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
		{#if algorithm == "ACO"}
			<div class="flex flex-wrap gap-2">
				<label class="flex flex-col flex-grow">
					Alpha (α):
					<input
						type="number"
						step="any"
						min="0"
						required
						bind:value={alpha}
						class="border rounded px-1"
					/>
				</label>
				{#if isTSP(problem)}
					<label class="flex flex-col flex-grow">
						Beta (β):
						<input
							type="number"
							step="any"
							min="0"
							required
							bind:value={beta}
							class="border rounded px-1"
						/>
					</label>
				{/if}
				<label class="flex flex-col flex-grow">
					Evaporation factor (ρ):
					<input
						type="number"
						step="any"
						min="0"
						required
						bind:value={evap_factor}
						class="border rounded px-1"
					/>
				</label>
				<label class="flex flex-col flex-grow">
					Amount of ants:
					<input
						type="number"
						step="any"
						min="0"
						required
						bind:value={ants}
						class="border rounded px-1"
					/>
				</label>
			</div>
			{#if isTSP(problem)}
				<label class="flex items-center gap-2">
					Pheromone update strategy:
					<select bind:value={updateStrategy} class="border rounded">
						{#each strategyOptions as option}
							<option value={option}>{option}</option>
						{/each}
					</select>
				</label>
				<div class="flex gap-2">
					<label class="flex items-center gap-2">
						Fitness based borders:
						<input
							type="checkbox"
							bind:checked={fit_based}
							class="w-4 h-4"
						/>
					</label>
					{#if fit_based}
						<label class="flex items-center gap-2">
							p_best:
							<input
								type="number"
								step="any"
								min="0"
								max="1"
								required
								bind:value={p_best}
								class="border rounded px-1"
							/>
						</label>
					{/if}
				</div>
				<div class="flex gap-2">
					<label class="flex items-center gap-2">
						Fitness based update:
						<input
							type="checkbox"
							bind:checked={fit_based_update}
							class="w-4 h-4"
						/>
					</label>
					{#if fit_based_update}
						<label class="flex items-center gap-2">
							Q:
							<input
								type="number"
								step="any"
								min="1"
								required
								bind:value={q}
								class="border rounded px-1"
							/>
						</label>
					{/if}
				</div>
				<label class="flex flex-col">
					<label class="flex items-center gap-2">
						Nearest neighbor start:
						<input
							type="checkbox"
							bind:checked={nearest_neighbor}
							class="w-4 h-4"
						/>
					</label>
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
