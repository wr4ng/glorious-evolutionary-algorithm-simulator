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
	class={`border-solid border-1 rounded-lg p-2 font-bold ${disabled ? "bg-slate-300" : "bg-slate-500 hover:bg-slate-600 border-slate-500"} text-white ${extraClass}`}
	{disabled}
	{onclick}
>
	{text}
</button>
