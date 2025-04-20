<script lang="ts">
	import { berlin52EUC2D, parseEUC2D } from "../lib/tsp";

	interface TaskCreateFormProps {
		onSubmit: (r: any) => void;
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
	const tspMutatorOptions = [
		{ text: "2-opt", value: "TwoOpt" },
		{ text: "3-opt", value: "ThreeOpt" },
	];

	let problem = $state("OneMax");
	let bitstringSize = $state(1000);
	let tspInstance = $state("berlin52");
	let customTspInstance = $state("");
	let customTspInstanceError = $state("");
	let customTspInstanceValidated = $state(false);
	let tspMutator = $state("TwoOpt");

	let algorithm = $state("OnePlusOneEA");
	let coolingRate = $state(1.0);

	let maxIterations = $state(1000000);
	let optimalFitness: number | undefined = $state(undefined);

	function isBitstringProblem(problem: string) {
		return problem == "OneMax" || problem == "LeadingOnes";
	}

	function isTSP(problem: string) {
		return problem == "TSP";
	}

	function needMutator(algorithm: string) {
		return algorithm == "OnePlusOneEA" || algorithm == "SimulatedAnnealing";
	}

	function validateCustomTSP() {
		try {
			const _ = parseEUC2D(customTspInstance);
			customTspInstanceValidated = true;
		} catch (error) {
			console.log(error);
			customTspInstanceError = "Invalid EUC2D instance";
		}
	}

	async function handleSumbit(e: SubmitEvent) {
		e.preventDefault();
		//TODO: Validate object
		const requestBody = {
			problem: problem,
			algorithm: algorithm,
			stop_cond: {
				max_iterations: maxIterations,
				...(optimalFitness && {
					optimal_fitness: optimalFitness,
				}),
			},
			...(isBitstringProblem(problem) && {
				bitstring_size: bitstringSize,
			}),
			...(isTSP(problem) && {
				tsp_instance:
					tspInstance == "berlin52"
						? berlin52EUC2D
						: customTspInstance,
				...(needMutator(algorithm) && {
					tsp_mutator: tspMutator,
				}),
			}),
			...(algorithm == "SimulatedAnnealing" && {
				cooling_rate: coolingRate,
			}),
		};
		console.log(requestBody);
		onSubmit(requestBody);
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
		{#if isTSP(problem) && needMutator(algorithm)}
			<label class="flex flex-col">
				TSP Mutator:
				<select required bind:value={tspMutator} class="border rounded">
					{#each tspMutatorOptions as option}
						<option value={option.value}>{option.text}</option>
					{/each}
				</select>
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
				Cooling rate (c):
				<input
					type="number"
					step="any"
					required
					bind:value={coolingRate}
					class="border rounded px-1"
				/>
			</label>
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
	<button type="submit" class="border rounded-lg py-2 font-bold">
		Create Task
	</button>
</form>
