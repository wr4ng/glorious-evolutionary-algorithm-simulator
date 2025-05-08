<script lang="ts">
	import { taskToText } from "../lib/task";
	import type { Task } from "../types/task";
	import Button from "./ui/Button.svelte";

	interface ScheduleListProps {
		schedule: Task[];
		removeTask: (i: number) => void;
	}

	let { schedule, removeTask }: ScheduleListProps = $props();
</script>

<h1 class="mt-4 text-4xl font-extrabold">Current Schedule</h1>

<div class="text-gray-700 shadow-md bg-white border rounded-xl overflow-hidden">
	<table class="w-full text-left table-auto">
		<thead>
			<tr class="bg-gray-100">
				<th
					class={"p-4 text-center " +
						(schedule.length > 0
							? "border-b border-blue-gray-100"
							: "")}>#</th
				>
				<th
					class={"p-4 " +
						(schedule.length > 0
							? "border-b border-blue-gray-100"
							: "")}>Task</th
				>
				<th
					class={"p-4 " +
						(schedule.length > 0
							? "border-b border-blue-gray-100"
							: "")}>Stop Condition</th
				>
				<th
					class={"p-4 " +
						(schedule.length > 0
							? "border-b border-blue-gray-100"
							: "")}
				></th>
			</tr>
		</thead>
		<tbody>
			{#each schedule as task, i}
				<tr class={i % 2 === 1 ? "bg-gray-50" : "bg-white"}>
					<td
						class={"p-2 text-center " +
							(i === schedule.length - 1
								? ""
								: "border-b border-blue-gray-50")}
					>
						<p>{i + 1}</p>
					</td>

					<td
						class={"p-2 " +
							(i === schedule.length - 1
								? ""
								: "border-b border-blue-gray-50")}
					>
						<strong>{taskToText(task)}</strong>
					</td>
					<td
						class={"p-2 " +
							(i === schedule.length - 1
								? ""
								: "border-b border-blue-gray-50")}
					>
						<p>Max iterations: {task.stop_cond.max_iterations}</p>
						{#if task.stop_cond.optimal_fitness}
							<p>
								Optimal fitness: {task.stop_cond
									.optimal_fitness}
							</p>
						{/if}
					</td>
					<td
						class={"p-2 text-right shrink " +
							(i === schedule.length - 1
								? ""
								: "border-b border-blue-gray-50")}
					>
						<Button text="Remove" onclick={() => removeTask(i)} />
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
