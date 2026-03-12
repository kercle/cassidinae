<script lang="ts">
	import { submitExpression } from '$lib';

	export let symbol: string;

	function handleClick() {
		submitExpression(`Help[${symbol}]`);
	}
</script>

<button
	type="button"
	class=" border-0 bg-transparent p-0 hover:brightness-105"
	on:click={handleClick}
>
	<span class="badge badge-accent cursor-pointer font-mono">{symbol}</span>
</button>
