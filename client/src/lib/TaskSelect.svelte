<script lang="ts">
	import type { Task } from "../types/task.ts"


	interface TaskSelectProps {
		tasks: Task[];
		selectTask: (id: string) => void;
	}

	let { tasks, selectTask }: TaskSelectProps = $props();
</script>

<div>
	{#if tasks.length == 0}
		<p>No running tasks.</p>
	{:else}
		<ul>
			{#each tasks as task}
				<li>
					<button onclick={() => selectTask(task.id)}>
						{task.id}
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
