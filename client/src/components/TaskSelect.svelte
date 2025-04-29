<script lang="ts">
	import type { Task, TaskResult } from "../types/task.ts";
	import Dashboard from "./Dashboard.svelte";
	import TaskCreateForm from "./TaskCreateForm.svelte";
	import TaskList from "./TaskList.svelte";
    import TaskResultList from "./TaskResultList.svelte";

	interface TaskSelectProps {
		serverURL: string;
	}

	let { serverURL }: TaskSelectProps = $props();

	let in_progress: Task[] = $state([]);
	let queued: Task[] = $state([]);
	let finished: TaskResult[] = $state([]);
	let selectedTask: Task | null = $state(null);

	let lastRequestBody: Object | null = $state(null);
	let createError = $state("");

	interface GetTasksResponse {
		in_progress: Task[];
		queued: Task[];
		finished: TaskResult[];
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
		} catch (error) {
			console.log(error);
		}
	}

	//TODO: Create type for requestBody
	async function createTask(requestBody: any) {
		//TODO: Validate requestInput
		try {
			lastRequestBody = requestBody;
			const response = await fetch(`${serverURL}/tasks`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(requestBody),
			});
			if (!response.ok) {
				const responseText = await response.text();
				createError = `Failed to create task: ${responseText}`;
				return;
			}
			selectedTask = await response.json();
		} catch (error) {
			console.log(error);
			createError = "Failed to send create task request...";
		}
	}

	async function rerunTask() {
		deselectTask();
		createTask(lastRequestBody);
	}

	function deselectTask() {
		selectedTask = null;
		getTasks();
	}

	getTasks();
</script>

{#if selectedTask}
	<Dashboard
		{serverURL}
		task={selectedTask}
		back={deselectTask}
		rerun={rerunTask}
	/>
{:else}
	<div class="flex h-screen p-4 gap-4">
		<div class="w-1/2">
			<h1 class="text-4xl font-extrabold">In-Progress Tasks</h1>
			<TaskList tasks={in_progress} onClick={selectTask} />
			<h1 class="mt-4 text-4xl font-extrabold">Completed Tasks</h1>
			<TaskResultList tasks={finished} onClick={async (_: Task) => {}} />
		</div>
		<div class="w-1/2">
			<TaskCreateForm onSubmit={createTask} error={createError} />
		</div>
	</div>
{/if}
