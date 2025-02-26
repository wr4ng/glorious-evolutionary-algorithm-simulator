<script lang="ts">
	import { onMount } from "svelte";
	import ServerSelect from "./lib/ServerSelect.svelte";
	import TaskSelect from "./lib/TaskSelect.svelte";
	//TODO: Fix this
	import type { Task } from "./types/task";

	let serverURL = $state(
		new URLSearchParams(window.location.search).get("serverURL") || "",
	);
	let serverError = $state("");
	let taskID = $state(
		new URLSearchParams(window.location.search).get("taskID") || "",
	);

	let tasks: Task[] = $state([]);

	async function checkServerURL(url: string) {
		try {
			const response = await fetch(`${url}/tasks`);
			if (!response.ok) {
				throw new Error(`server responded with: ${response.status}`);
			}
			tasks = (await response.json()) as Task[];

			const newURL = new URL(window.location.href);
			newURL.searchParams.set("serverURL", url);
			window.history.pushState({}, "", newURL);

			serverURL = url;
		} catch (error) {
			console.log(error);
			serverError = "Failed to connect to server";
		}
	}

	function selectTask(id: string) {
		const newURL = new URL(window.location.href);
		newURL.searchParams.set("serverURL", serverURL);
		newURL.searchParams.set("taskID", id);
		window.history.pushState({}, "", newURL);

		taskID = id;
	}

	// Handle browser back/forward navigation
	onMount(() => {
		const handlePopState = () => {
			const params = new URLSearchParams(window.location.search);
			serverURL = params.get("serverURL") || "";
			if (serverURL) {
				checkServerURL(serverURL);
			}
		};

		window.addEventListener("popstate", handlePopState);

		return () => {
			window.removeEventListener("popstate", handlePopState);
		};
	});
</script>

<main>
	{#if !serverURL}
		<ServerSelect setServerURL={checkServerURL} error={serverError} />
	{:else if !taskID}
		<TaskSelect {tasks} {selectTask} />
	{:else}
		<p>{serverURL}</p>
		<p>{taskID}</p>
	{/if}
</main>
