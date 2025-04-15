<script lang="ts">
	import type { Component, SvelteComponent } from 'svelte';
	import Loader from './Loader.svelte';
	import type { FunctionEvent } from './types';
	import { render } from 'svelte/server';

	const {
		onclick,
		text,
		type,
		href,
		icon
	}: {
		onclick?: FunctionEvent;
		text?: string;
		icon?: any;
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

<button
	onclick={click}
	{type}
	class="flex items-center justify-center w-full gap-4 px-4 text-xl font-semibold text-white duration-200 bg-blue-600 rounded-lg cursor-pointer h-13 text-inherits not-dark:hover:bg-blue-800 dark:text-black dark:hover:bg-blue-500"
>
	{#if debounce === true}
		<div class="h-6">
			<Loader />
		</div>
	{/if}

	<p class="font-semibold">{text}</p>

	<!-- {#if icon}
			<div class="h-full aspect-square">
				{@render icon()}
			</div>
		{/if} -->
</button>
