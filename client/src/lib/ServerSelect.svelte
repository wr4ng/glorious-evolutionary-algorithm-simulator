<script lang="ts">
	interface ServerSelectProps {
		setServerURL: (url: string) => Promise<void>,
		error: string,
	}
	let { setServerURL, error }: ServerSelectProps = $props();

	let url = $state("http://localhost:3000");
	let loading = $state(false);

	async function handleSumbit(e: SubmitEvent) {
		e.preventDefault();
		if (!url) return;

		loading = true;
		await setServerURL(url);
		loading = false;
	}
</script>

<form onsubmit={handleSumbit} class="max-w-sm mx-auto flex flex-col p-4 gap-2">
	<label for="serverURL">Server URL: </label>
	<input
		type="text"
		id="serverURL"
		bind:value={url}
		required
		class="bg-gray-50 border rounded-lg p-2.5"
	/>
	<button type="submit" disabled={loading} class="border rounded-lg">
		{loading ? "Loading..." : "Select"}
	</button>
	{#if error}
		<p class="text-red-500 font-bold">{error}</p>
	{/if}
</form>
