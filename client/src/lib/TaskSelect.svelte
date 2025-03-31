<script lang="ts">
	import type { Task } from "../types/task.ts";
    import Dashboard from "./Dashboard.svelte";

	interface TaskSelectProps {
		serverURL: string;
	}

	let { serverURL }: TaskSelectProps = $props();

	let tasks: Task[] = $state([]);
	let finished: Task[] = $state([]);
	let selectedTask: Task | null = $state(null);

	interface GetTasksResponse {
		queued: Task[];
		finished: Task[];
	}

	async function selectTask(task: Task) {
		selectedTask = task;
		console.log("selected: " + task.id);
	}

	async function getTasks() {
		try {
			const response = await fetch(`${serverURL}/tasks`);
			if (!response.ok) {
				throw new Error(`server responded with: ${response.status}`);
			}
			const resp = (await response.json()) as GetTasksResponse;
			tasks = resp.queued;
			finished = resp.finished;
		} catch (error) {
			console.log(error);
		}
	}

	getTasks();
</script>

{#if selectedTask}
	<Dashboard serverURL={serverURL} task={selectedTask} />
{:else}
	<div class="p-4">
		<h1 class="text-4xl font-extrabold">Tasks</h1>
		{#if tasks.length == 0}
			<p>No running tasks.</p>
		{:else}
			<ul>
				{#each tasks as task}
					<li class="mb-2">
						<button
							onclick={() => selectTask(task)}
							class="w-full text-left p-2 border border-gray-200 bg-white hover:bg-gray-100 rounded-lg shadow-sm"
						>
							{task.id}
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	</div>
{/if}
