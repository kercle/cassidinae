<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Math from './Math.svelte';

	export let input_expr = '';
	export let output_expr = '';
</script>

<div class="group w-full rounded-sm shadow-md" in:fly={{ y: 20, duration: 400, easing: cubicOut }}>
	<div class="bg-base-200 relative rounded-t-sm">
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

		<div class="pt-2 pl-8">
			<Math expr={input_expr} />
		</div>
	</div>

	<div class="border-base-200 rounded-b-sm border pl-8">
		<Math expr={'=' + output_expr} />
	</div>
</div>
