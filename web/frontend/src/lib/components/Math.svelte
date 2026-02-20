<script lang="ts">
	import katex from 'katex';
	export let expr = '';
</script>

<div class="math-container flex w-full" style="font-size: 0.9rem;">
	{@html katex.renderToString(expr, {
		throwOnError: false,
		displayMode: true
	})}
</div>
