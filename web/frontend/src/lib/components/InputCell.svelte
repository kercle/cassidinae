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

<div class="bg-base-200 w-full p-1">
	<label
		class="input input-ghost flex w-full items-center gap-2 focus-within:bg-transparent focus-within:outline-none"
	>
		<i class="fa-solid fa-angle-right text-info-content"></i>

		<input
			type="text"
			placeholder="Input expression"
			bind:value={inputValue}
			onkeydown={handleKeydown}
		/>
	</label>
</div>
