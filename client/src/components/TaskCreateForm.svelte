<script lang="ts">
	const problemOptions = ["OneMax", "LeadingOnes", "TSP"];
	const algorithmOptions = [
		{ text: "(1+1) EA", value: "OnePlusOneEA" },
		{ text: "Simulated Annealing", value: "SimulatedAnnealing" },
	];

	let problem = $state("OneMax");
	let bitstringSize = $state(1000);
	let tspInstance = $state("");

	let algoritm = $state("OnePlusOneEA");

	let maxIterations = $state(1000000);

	function isBitstringProblem(problem: string) {
		return problem == "OneMax" || problem == "LeadingOnes";
	}

	function isTSP(problem: string) {
		return problem == "TSP";
	}

	async function handleSumbit(e: SubmitEvent) {
		e.preventDefault();
		//TODO: Validate object + send request
		console.log({
			problem: problem,
			algoritm: algoritm,
			stop_condition: {
				max_iterations: maxIterations,
			},
			...(isBitstringProblem(problem) && { bistring_size: bitstringSize })
		});
	}
</script>

<form onsubmit={handleSumbit} class="flex flex-col p-4 gap-2">
	<h1 class="text-xl font-bold">Problem</h1>
	<label>
		Problem:
		<select bind:value={problem} class="border rounded">
			{#each problemOptions as option}
				<option value={option} selected={option == "OneMax"}>{option}</option>
			{/each}
		</select>
	</label>
	{#if isBitstringProblem(problem)}
		<label>
			Bitstring Size:
			<input
				type="number"
				step="1"
				bind:value={bitstringSize}
				required
				class="border rounded"
			/>
		</label>
	{/if}
	{#if isTSP(problem)}
		<label class="flex flex-col">
			TSP Instance (EUC2D Format):
			<textarea
				bind:value={tspInstance}
				class="border rounded"
				placeholder="Enter TSP instance..."
			></textarea>
		</label>
	{/if}
	<h1 class="text-xl font-bold">Algoritm</h1>
	<label>
		Algorithm:
		<select bind:value={algoritm} class="border rounded">
			{#each algorithmOptions as option}
				<option value={option.value}>{option.text}</option>
			{/each}
		</select>
	</label>
	<h1 class="text-xl font-bold">Stop Condition</h1>
	<label>
		Max Iterations:
		<input
			type="number"
			step="1"
			bind:value={maxIterations}
			required
			class="border rounded"
		/>
	</label>
	<button type="submit" class="border rounded-lg py-2 font-bold">
		Submit
	</button>
</form>
