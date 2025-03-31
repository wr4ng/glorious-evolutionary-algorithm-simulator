<script lang="ts">
	import type { Task } from "../types/task.ts";
	import Dashboard from "./Dashboard.svelte";
	import TaskList from "./TaskList.svelte";

	interface TaskSelectProps {
		serverURL: string;
	}

	let { serverURL }: TaskSelectProps = $props();

	let in_progress: Task[] = $state([]);
	let queued: Task[] = $state([]);
	let finished: Task[] = $state([]);
	let selectedTask: Task | null = $state(null);

	interface GetTasksResponse {
		in_progress: Task[];
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
			in_progress = resp.in_progress;
			queued = resp.queued;
			finished = resp.finished;
			console.log(resp.in_progress);
		} catch (error) {
			console.log(error);
		}
	}

	getTasks();
</script>

{#if selectedTask}
	<Dashboard {serverURL} task={selectedTask} />
{:else}
	<div class="p-4">
		<h1 class="text-4xl font-extrabold">In-Progress Tasks</h1>
		<TaskList tasks={in_progress} onClick={selectTask} />
		<h1 class="mt-4 text-4xl font-extrabold">Completed Tasks</h1>
		<TaskList tasks={finished} onClick={async (t: Task) => {}} />
	</div>
{/if}
