<script lang="ts">
    import Navbar from "./Navbar.svelte";
	import ServerSelect from "./ServerSelect.svelte";
	import ScheduleOverview from "./ScheduleOverview.svelte";

	let serverURL = $state("");
	let serverError = $state("");

	async function pingServer(url: string) {
		if (!url) {
			return;
		}
		try {
			const pingResponse = await fetch(`${url}/ping`);
			if (!pingResponse.ok) {
				throw new Error(
					`server responded with status: ${pingResponse.status}`,
				);
			}
			const reponseText = await pingResponse.text();
			if (reponseText != "pong") {
				throw new Error(`server responded with: ${reponseText}`);
			}
		} catch (error) {
			console.log(error);
			serverError = "Failed to reach server";
			return;
		}
		const newURL = new URL(window.location.href);
		newURL.searchParams.set("serverURL", url);
		window.history.pushState({}, "", newURL);

		serverURL = url;
	}

	const url =
		new URLSearchParams(window.location.search).get("serverURL") || "";
	pingServer(url);
</script>

<main>
	{#if !serverURL}
		<ServerSelect setServerURL={pingServer} error={serverError} />
	{:else}
		<Navbar />
		<ScheduleOverview {serverURL} />
	{/if}
</main>
 
