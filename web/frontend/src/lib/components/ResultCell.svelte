<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Math from './Math.svelte';
	import { tick } from 'svelte';

	export let entry: ServerMessage | undefined = undefined;

	const getInput = () => {
		if (entry) {
			if ('evalResult' in entry) {
				return entry.evalResult.input;
			} else if ('parseError' in entry) {
				return entry.parseError.input;
			}
		} else {
			return '';
		}
	};

	const highlightColor = () => {
		if (entry) {
			if ('parseError' in entry) {
				return 'red-200';
			}
		}

		return 'base-200';
	};
</script>

{#snippet menu()}
	<div
		class="absolute top-4 right-4 z-10 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
	>
		<ul class="menu menu-horizontal bg-base-200/50 rounded-box gap-1 p-0">
			<li>
				<button
					class="hover:bg-base-300 flex items-center justify-center p-2 transition-colors"
					title="Copy"
				>
					<i class="fa-solid fa-copy text-sm"></i>
				</button>
			</li>
			<li>
				<button
					class="hover:bg-error/20 hover:text-error flex items-center justify-center p-2 transition-colors"
					title="Delete"
				>
					<i class="fa-solid fa-trash text-sm"></i>
				</button>
			</li>
		</ul>
	</div>
{/snippet}

<div class="group w-full rounded-sm shadow-md" in:fly={{ y: 20, duration: 400, easing: cubicOut }}>
	<div class="bg-{highlightColor()} relative rounded-t-sm">
		{@render menu()}

		<div class="pt-2 pl-8">
			<Math expr={getInput()} />
		</div>
	</div>

	{#if entry}
		{#if 'evalResult' in entry}
			<div class="border-{highlightColor()} rounded-b-sm border pl-8">
				<Math expr={'=' + entry.evalResult.output} />
			</div>
		{:else if 'parseError' in entry}
			<div class="border-{highlightColor()} rounded-b-sm border p-2 pl-8">
				<b class="mr-2">Error:</b>{entry.parseError.msg}
			</div>
		{/if}
	{/if}
</div>
