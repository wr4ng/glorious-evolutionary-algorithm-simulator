<script lang="ts">
	import type { Task } from "../types/task.ts";

	interface TaskSelectProps {
		tasks: Task[];
		selectTask: (id: string) => void;
	}

	let { tasks, selectTask }: TaskSelectProps = $props();
</script>

<div class="p-4">
	<h1 class="text-4xl font-extrabold">Tasks</h1>
	{#if tasks.length == 0}
		<p>No running tasks.</p>
	{:else}
		<ul>
			{#each tasks as task}
				<li class="mb-2">
					<button
						onclick={() => selectTask(task.id)}
						class="w-full text-left p-2 border border-gray-200 bg-white hover:bg-gray-100 rounded-lg shadow-sm"
					>
						{task.id}
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
