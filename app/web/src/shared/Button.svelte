<script lang="ts">
	import type { Component } from 'svelte';
	import Loader from './Loader.svelte';
	import type { FunctionEvent } from './types';

	const {
		onclick,
		text,
		type,
		fill_width,
		href,
		Icon
	}: {
		onclick?: FunctionEvent;
		text?: string;
		fill_width?: boolean;
		Icon?: Component;
		type?: 'submit' | 'button' | 'reset';
		href?: string;
	} = $props();

	let debounce = $state(false);
	async function click(event: Event) {
		if (href) {
			window.location = href as string & Location;
		}

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
	class="{`base ${fill_width ? 'w-full' : ''}`} text-inherits flex min-h-13 cursor-pointer items-center justify-center gap-4 rounded-lg bg-blue-600 px-4 text-xl font-semibold text-white duration-200 not-dark:hover:bg-blue-800 dark:text-black dark:hover:bg-blue-500"
>
	{#if debounce === true}
		<span class="aspect-square h-1/2">
			<Loader />
		</span>
	{/if}
	{#if Icon && debounce == false}
		<span class="aspect-square h-1/2">
			<Icon />
		</span>
	{/if}

	<p class="font-semibold">{text}</p>
</button>
