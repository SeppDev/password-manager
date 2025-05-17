<script lang="ts">
    import type { Component } from "svelte";
    import Loader from "./Loader.svelte";
    import type { FunctionEvent } from "./types";

    const {
        onclick,
        text,
        type = "button",
        compact = false,
        fill_width = false,
        prevent_default = false,
        theme = "primary",
        href,
        Icon,
    }: {
        onclick?: FunctionEvent;
        text?: string;
        fill_width?: boolean;
        prevent_default?: boolean;
        compact?: boolean;
        Icon?: Component;
        theme?: "primary" | "secondary";
        type?: "submit" | "button" | "reset";
        href?: string;
    } = $props();

    let debounce = $state(false);
    async function click(event: Event) {
        if (prevent_default) event.preventDefault();
        if (href) window.location = href as string & Location;
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
    class="{fill_width ? 'w-full' : ''} {compact
        ? 'text-sm h-8'
        : 'text-xl h-13'} text-inherits flex max-h-full cursor-pointer items-center justify-center gap-4 px-4 rounded-lg bg-blue-600 font-semibold text-white duration-200 not-dark:hover:bg-blue-800 dark:text-black dark:hover:bg-blue-500"
>
    {#if debounce === true}
        <span class="aspect-square h-1/2 flex justify-center items-center">
            <Loader />
        </span>
    {/if}
    {#if Icon && debounce == false}
        <span class="aspect-square h-1/2 flex justify-center items-center">
            <Icon />
        </span>
    {/if}

    {#if text}
        <p class="font-semibold">{text}</p>
    {/if}
</button>
