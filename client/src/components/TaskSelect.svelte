<script lang="ts">
	import type {
		TaskSchedule,
		TaskScheduleRequest,
		TaskScheduleResult,
	} from "../types/task.ts";
	import Dashboard from "./Dashboard.svelte";
	import TaskCreateForm from "./TaskCreateForm.svelte";
	import TaskResultList from "./TaskResultList.svelte";

	interface TaskSelectProps {
		serverURL: string;
	}

	let { serverURL }: TaskSelectProps = $props();

	let results: TaskScheduleResult[] = $state([]);
	let selectedTaskSchedule: TaskSchedule | null = $state(null);

	let createError = $state("");

	async function getResults() {
		try {
			const response = await fetch(`${serverURL}/results`);
			if (!response.ok) {
				throw new Error(`server responded with: ${response.status}`);
			}
			const data = (await response.json()) as TaskScheduleResult[];
			results = data;
		} catch (error) {
			console.log(error);
		}
	}

	//TODO: Create type for requestBody
	async function createTaskSchedule(request: TaskScheduleRequest) {
		//TODO: Validate requestInput
		try {
			const response = await fetch(`${serverURL}/schedules`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(request),
			});
			if (!response.ok) {
				const responseText = await response.text();
				createError = `Failed to create task: ${responseText}`;
				return;
			}
			selectedTaskSchedule = await response.json();
		} catch (error) {
			console.log(error);
			createError = "Failed to send create task schedule request...";
		}
	}

	function deselectTaskSchedule() {
		selectedTaskSchedule = null;
		getResults();
	}

	getResults();
</script>

{#if selectedTaskSchedule}
	<Dashboard
		{serverURL}
		taskSchedule={selectedTaskSchedule}
		back={deselectTaskSchedule}
	/>
{:else}
	<div class="flex h-screen p-4 gap-4">
		<div class="w-1/2">
			<h1 class="mt-4 text-4xl font-extrabold">Results</h1>
			<TaskResultList {results} />
		</div>
		<div class="w-1/2">
			<h1 class="mt-4 text-4xl font-extrabold">Create Schedule</h1>
			<TaskCreateForm onSubmit={createTaskSchedule} error={createError} />
		</div>
	</div>
{/if}
