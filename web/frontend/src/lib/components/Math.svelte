<script lang="ts">
	import katex from 'katex';
	export let expr = '';
</script>

<div class="math-container flex w-full">
	{@html katex.renderToString(expr, {
		throwOnError: false,
		displayMode: true // This is the magic switch
	})}
</div>
