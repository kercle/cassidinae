<script lang="ts">
	import katex from 'katex';
	import ResultCell from '../ResultCell.svelte';
	import HelpLink from './HelpLink.svelte';

	export let title: string;
	export let patterns: Array<[string, string]>;
	export let examples: Array<[string, string]>;
	export let related: Array<string>;

	function renderMixed(text: string) {
		return text.replace(/\$([^$]+)\$/g, (_, math) => {
			return katex.renderToString(math, { throwOnError: false });
		});
	}
</script>

<div class="bg-base-200 mt-1 p-4 font-sans">
	<h1 class="bg-neutral-content p-2 text-2xl font-semibold">{title}</h1>
	<ul class="divide-base-300 ml-2 divide-y p-4">
		{#each patterns as pat}
			<li class="mt-2 pb-2">
				<div class="badge badge-accent font-mono">{pat[0]}</div>
				<div class="mt-2 ml-6">{@html renderMixed(pat[1])}</div>
			</li>
		{/each}
	</ul>
	{#if examples.length > 0}
		<h2 class="p-2 text-xl font-semibold">Examples</h2>
		<table class="my-4 ml-6 border-collapse border-spacing-2">
			<thead class="font-light">
				<tr>
					<td class="pb-2">Input</td>
					<td class="pb-2">Output</td>
				</tr>
			</thead>
			<tbody>
				{#each examples as ex}
					<tr class="border-base-300 border-t">
						<td class="mt-2 pt-2 pr-8 pb-2 font-mono">{ex[0]}</td>
						<td class="mt-2 pt-2 pb-2 font-mono">{ex[1]}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
	{#if related.length > 0}
		<h2 class="p-2 text-xl font-semibold">Related</h2>
		<div class="flex flex-row gap-2 pl-6">
			{#each related as rel}
				<HelpLink symbol={rel} />
			{/each}
		</div>
	{/if}
</div>
