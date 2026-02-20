<script lang="ts">
	import { appState } from '$lib';

	import InputCell from '$lib/components/InputCell.svelte';
	import ResultCell from '$lib/components/ResultCell.svelte';
	import { tick } from 'svelte';

	let scrollContainer: HTMLElement;

	// Optional: Auto-scroll to bottom when history grows
	$: if ($appState.data?.history && scrollContainer) {
		scrollToBottom();
	}

	async function scrollToBottom() {
		await tick();
		scrollContainer.scrollTo({
			top: scrollContainer.scrollHeight,
			behavior: 'smooth'
		});
	}
</script>

<div class="flex h-screen flex-col overflow-hidden bg-white">
	<div bind:this={scrollContainer} class="grow overflow-y-auto p-8">
		<div class="flex flex-col gap-4">
			{#each $appState.data?.history as entry}
				<ResultCell {entry} />
			{/each}
		</div>
	</div>

	<div class="sticky bottom-0 bg-white p-8 pt-4">
		<InputCell />
	</div>
</div>
