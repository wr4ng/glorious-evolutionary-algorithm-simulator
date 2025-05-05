<script lang="ts">
	import { mapAlgorithmName } from "../lib/algorithm";
	import type { Task } from "../types/task";

	interface TaskTextProps {
		task: Task;
	}

	let { task }: TaskTextProps = $props();
</script>

<div>
	<strong
		>{mapAlgorithmName(task.algorithm.type)} - {task.problem.type}</strong
	>
	{#if task.algorithm.cooling_schedule}
		<p>
			Cooling Scheduel: {task.algorithm.cooling_schedule.type}
		</p>
		<p>
			{#if task.algorithm.cooling_schedule.temperature}
				Temperature: {task.algorithm.cooling_schedule.temperature}
			{:else if task.algorithm.cooling_schedule.cooling_rate}
				Cooling rate (c): {task.algorithm.cooling_schedule.cooling_rate}
			{/if}
		</p>
	{/if}
	<p>Stop Condition:</p>
	<p>Max iterations: {task.stop_cond.max_iterations}</p>
	{#if task.stop_cond.optimal_fitness}
		<p>Optimal fitness: {task.stop_cond.optimal_fitness}</p>
	{/if}
</div>
