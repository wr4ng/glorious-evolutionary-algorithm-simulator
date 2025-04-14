<script lang="ts">
	import { berlin52EUC2D } from "../lib/tsp";

	interface TaskCreateFormProps {
		onSubmit: (r: any) => void;
		error: string;
	}

	let { onSubmit, error }: TaskCreateFormProps = $props();

	const problemOptions = ["OneMax", "LeadingOnes", "TSP"];
	const algorithmOptions = [
		{ text: "(1+1) EA", value: "OnePlusOneEA" },
		{ text: "Simulated Annealing", value: "SimulatedAnnealing" },
	];
	const tspInstanceOptions = ["berlin52", "Custom"];

	let problem = $state("OneMax");
	let bitstringSize = $state(1000);
	let tspInstance = $state("berlin52");
	let customTspInstance = $state("");

	let algorithm = $state("OnePlusOneEA");

	let maxIterations = $state(1000000);
	let optimalFitness: number | undefined = $state(undefined);

	function isBitstringProblem(problem: string) {
		return problem == "OneMax" || problem == "LeadingOnes";
	}

	function isTSP(problem: string) {
		return problem == "TSP";
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
		{/if}
		{#if isTSP(problem) && tspInstance == "Custom"}
			<label class="flex flex-col">
				Custom TSP Instance (EUC2D Format):
				<textarea
					bind:value={customTspInstance}
					class="border rounded p-1"
					placeholder="Enter TSP instance..."
				></textarea>
			</label>
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
