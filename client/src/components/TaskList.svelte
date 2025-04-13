<script lang="ts">
	import type { Task } from "../types/task.ts";

	interface TaskListProps {
		tasks: Task[];
		onClick: (task: Task) => Promise<void>;
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
					<strong>{task.algorithm} - {task.problem}</strong>
					({task.id})
				</button>
			</li>
		{/each}
	</ul>
{/if}
