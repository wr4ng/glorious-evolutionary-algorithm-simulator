<script lang="ts">
	import type { TaskScheduleResult } from "../types/task.ts";

	interface TaskListProps {
		results: TaskScheduleResult[];
		onClick: (task: TaskScheduleResult) => Promise<void>;
	}

	const { results, onClick }: TaskListProps = $props();
</script>

<ul>
	{#each results as result}
		<li class="mb-2">
			<button
				onclick={() => onClick(result)}
				class="w-full text-left p-2 border border-gray-200 bg-white hover:bg-gray-100 rounded-lg shadow-sm"
			>
				{#each result.results as taskResult}
					<strong
						>{taskResult.task.algorithm.type} - {taskResult.task.problem
							.type}</strong
					>
					{#if taskResult.task.algorithm.cooling_schedule}
						<p>
							Cooling Scheduel: {taskResult.task.algorithm.cooling_schedule
								.type}
						</p>
						<p>
							{#if taskResult.task.algorithm.cooling_schedule.temperature}
								Temperature: {taskResult.task.algorithm.cooling_schedule
									.temperature}
							{:else if taskResult.task.algorithm.cooling_schedule.cooling_rate}
								Cooling rate (c): {taskResult.task.algorithm
									.cooling_schedule.cooling_rate}
							{/if}
						</p>
					{/if}
					<p>Final fitness: {taskResult.fitness}</p>
					<p>Final iterations: {taskResult.iterations}</p>
				{/each}
			</button>
		</li>
	{/each}
</ul>
