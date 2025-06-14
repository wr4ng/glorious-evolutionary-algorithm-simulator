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
	let previousTask: Task | null = $state(null);
	let repeatCount: number = $state(1);
	let updateRate: number = $state(5000);
	let seed: number = $state(0);
	let createError = $state("");

	async function submitSchedule(e: SubmitEvent) {
		e.preventDefault();

		try {
			let request: TaskScheduleRequest = {
				tasks: tasks,
				repeat_count: repeatCount,
				update_rate: updateRate,
				seed: seed,
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
			console.log($state.snapshot(selectedTaskSchedule));
		} catch (error) {
			console.log(error);
			createError = "Failed to send create task schedule request...";
		}
	}

	function setRandomSeed() {
		seed = Math.floor(Math.random() * 9007199254740991); // 2^53 - 1, which is largest integer to be correctly represented
	}

	function removeTask(i: number) {
		console.log(i);
		tasks.splice(i, 1);
	}

	function addTask(t: Task) {
		tasks = [...tasks, t];
		previousTask = t;
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
			<TaskCreateForm {addTask} {previousTask}/>
		</div>
		<div class="w-1/2 flex flex-col gap-4">
			<ScheduleList schedule={tasks} {removeTask} />
			{#if createError}
				<span class="text-red-500 font-bold">{createError}</span>
			{/if}
			{#if tasks.length > 0}
				<form onsubmit={submitSchedule} class="flex flex-col space-y-4">
					<label class="flex items-center gap-2">
						Repeat Count:
						<input
							type="number"
							step="1"
							min="1"
							max="100"
							required
							bind:value={repeatCount}
							class="border rounded px-1"
						/>
					</label>
					<label class="flex items-center gap-2">
						Update rate:
						<input
							type="number"
							step="1"
							min="100"
							max="50000"
							required
							bind:value={updateRate}
							class="border rounded px-1"
						/>
					</label>
					<label class="flex items-center gap-2">
						Seed:
						<input
							type="number"
							step="1"
							min="0"
							required
							bind:value={seed}
							class="border rounded px-1"
						/>
						<Button text="Randomize" onclick={setRandomSeed} />
					</label>
					<div class="flex gap-2">
						<Button
							text="Create Schedule"
							type="submit"
							extraClass="grow"
						/>
						<Button
							text="Clear Schedule"
							type="button"
							onclick={clearSchedule}
							extraClass="grow"
						/>
					</div>
				</form>
			{/if}
		</div>
	</div>
{/if}
