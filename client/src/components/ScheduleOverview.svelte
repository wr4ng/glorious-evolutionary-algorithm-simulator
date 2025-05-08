<script lang="ts">
	import type {
		Task,
		TaskSchedule,
		TaskScheduleRequest,
	} from "../types/task.ts";
	import Dashboard from "./Dashboard.svelte";
	import ScheduleList from "./ScheduleList.svelte";
	import TaskCreateForm from "./TaskCreateForm.svelte";
	import Button from "./ui/Button.svelte";

	interface TaskSelectProps {
		serverURL: string;
	}

	let { serverURL }: TaskSelectProps = $props();

	let selectedTaskSchedule: TaskSchedule | null = $state(null);

	let tasks: Task[] = $state([]);
	let createError = $state("");

	async function submitSchedule() {
		try {
			let request: TaskScheduleRequest = {
				tasks: tasks,
			};
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

	function removeTask(i: number) {
		console.log(i);
		tasks.splice(i, 1);
	}

	function addTask(t: Task) {
		tasks = [...tasks, t];
	}

	function clearSchedule() {
		tasks = [];
	}

	function deselectTaskSchedule() {
		selectedTaskSchedule = null;
	}
</script>

{#if selectedTaskSchedule}
	<Dashboard
		{serverURL}
		taskSchedule={selectedTaskSchedule}
		back={deselectTaskSchedule}
	/>
{:else}
	<div class="p-4 flex gap-4">
		<div class="w-1/2">
			<TaskCreateForm {addTask} />
		</div>
		<div class="w-1/2 flex flex-col gap-4">
			<ScheduleList schedule={tasks} {removeTask} />
			{#if createError}
				<span class="text-red-500 font-bold">{createError}</span>
			{/if}
			{#if tasks.length > 0}
				<div class="flex gap-2">
					<Button
						text="Create Schedule"
						type="button"
						onclick={submitSchedule}
						extraClass="grow"
					/>
					<Button
						text="Clear Schedule"
						type="button"
						onclick={clearSchedule}
						extraClass="grow"
					/>
				</div>
			{/if}
		</div>
	</div>
{/if}
