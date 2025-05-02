<script lang="ts">
	import type { TaskScheduleResult } from "../types/task.ts";
    import TaskText from "./TaskText.svelte";

	interface TaskListProps {
		results: TaskScheduleResult[];
	}

	const { results }: TaskListProps = $props();
</script>

<ul>
	{#each results as result}
		<li class="mb-2">
			<div
				class="w-full text-left p-2 border border-gray-200 bg-white hover:bg-gray-100 rounded-lg shadow-sm"
			>
				{#each result.results as taskResult}
					<TaskText task={taskResult.task} />
					<p>Final fitness: {taskResult.fitness}</p>
					<p>Final iterations: {taskResult.iterations}</p>
				{/each}
			</div>
		</li>
	{/each}
</ul>
