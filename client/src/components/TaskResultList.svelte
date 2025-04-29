<script lang="ts">
	import type { TaskResult } from "../types/task.ts";

	interface TaskListProps {
		tasks: TaskResult[];
		onClick: (task: TaskResult) => Promise<void>;
	}

	const { tasks, onClick }: TaskListProps = $props();
</script>

{#if tasks.length == 0}
	<p>No running tasks.</p>
{:else}
	<ul>
		{#each tasks as task}
			<li class="mb-2">
				<button
					onclick={() => onClick(task)}
					class="w-full text-left p-2 border border-gray-200 bg-white hover:bg-gray-100 rounded-lg shadow-sm"
				>
					<strong>{task.algorithm.type} - {task.problem.type}</strong>
					({task.id})
					{#if task.algorithm.cooling_schedule}
						<p>
							Cooling Scheduel: {task.algorithm.cooling_schedule
								.type}
						</p>
						<p>
							{#if task.algorithm.cooling_schedule.temperature}
								Temperature: {task.algorithm.cooling_schedule
									.temperature}
							{:else if task.algorithm.cooling_schedule.cooling_rate}
								Cooling rate (c): {task.algorithm
									.cooling_schedule.cooling_rate}
							{/if}
						</p>
					{/if}
					<p>Final fitness: {task.final_fitness}</p>
					<p>Final iterations: {task.final_iterations}</p>
				</button>
			</li>
		{/each}
	</ul>
{/if}
