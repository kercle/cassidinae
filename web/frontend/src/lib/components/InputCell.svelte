<script lang="ts">
	import { appState } from '$lib';

	let inputValue = $state('');

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();

			if (inputValue.trim()) {
				submitExpression(inputValue);
				inputValue = '';
			}
		}
	}

	function submitExpression(expr: string) {
		const msg = {
			eval: expr
		};
		appState.send(JSON.stringify(msg));
	}
</script>

<div class="bg-base-200 w-full rounded-sm p-3 shadow-md">
	<label class="input input-ghost flex w-full items-center gap-2">
		<i class="fa-solid fa-angle-right"></i>
		<input
			type="text"
			class="grow"
			placeholder="Input expression"
			bind:value={inputValue}
			onkeydown={handleKeydown}
		/>
	</label>
</div>
