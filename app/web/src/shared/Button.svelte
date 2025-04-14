<script lang="ts">
	import Loader from './Loader.svelte';
	import type { FunctionEvent } from './types';

	const {
		onclick,
		text,
		type,
		href
	}: {
		onclick?: FunctionEvent;
		text: string;
		type?: 'submit' | 'button' | 'reset';
		href?: string;
	} = $props();

	let debounce = $state(false);
	async function click(event: Event) {
		if (!onclick) return;
		if (debounce) return;
		debounce = true;

		try {
			await onclick(event);
		} catch (e) {
			console.error(e);
		}

		debounce = false;
	}
</script>

<!-- {#if href}
	<a
		{href}
		onclick={click}
		class="flex items-center justify-center gap-4 px-4 py-4 text-xl font-semibold text-white duration-200 bg-blue-600 rounded-lg cursor-pointer text-inherits not-dark:hover:bg-blue-800 size-full dark:text-black dark:hover:bg-blue-500"
	>
		<p>{text}</p>
	</a>
{:else} -->
	<button
		onclick={click}
		{type}
		class="flex items-center justify-center gap-4 px-4 py-4 text-xl font-semibold text-white duration-200 bg-blue-600 rounded-lg cursor-pointer text-inherits not-dark:hover:bg-blue-800 size-full dark:text-black dark:hover:bg-blue-500"
	>
		<p>{text}</p>
		{#if debounce === true}
			<div class="h-full">
				<Loader />
			</div>
		{/if}
	</button>
<!-- {/if} -->
