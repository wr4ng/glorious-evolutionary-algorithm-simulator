<script lang="ts">
	interface ButtonProps {
		text: string;
		disabled?: boolean;
		type?: "button" | "submit";
		onclick?: () => void;
		extraClass?: string;
	}

	let {
		text,
		disabled = false,
		type = "button",
		onclick,
		extraClass = "",
	}: ButtonProps = $props();
</script>

<button
	{type}
	class={`border rounded-lg p-2 font-bold ${disabled ? "bg-gray-100" : "bg-gray-200 hover:bg-gray-300"} text-gray-800 ${extraClass}`}
	{disabled}
	{onclick}
>
	{text}
</button>
