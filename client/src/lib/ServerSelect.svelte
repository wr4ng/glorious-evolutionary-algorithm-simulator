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

<form onsubmit={handleSumbit}>
	<label>
		Server URL:
		<input type="text" bind:value={url} required />
	</label>
	<button type="submit" disabled={loading}>
		{loading ? "Loading..." : "Select"}
	</button>
</form>
{#if error}
	<p>{error}</p>
{/if}
