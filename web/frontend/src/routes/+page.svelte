<script lang="ts">
	import { appState } from '$lib';

	import InputCell from '$lib/components/InputCell.svelte';
	import ResultCell from '$lib/components/ResultCell.svelte';
</script>

<div class="flex flex-col gap-4 p-8">
	{#each $appState.data?.history as entry}
		<ResultCell input_expr={entry.evalResult.input} output_expr={entry.evalResult.output} />
	{/each}

	<InputCell />
</div>
