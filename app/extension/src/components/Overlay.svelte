<script lang="ts">
    import type { Component, Snippet } from "svelte";
    import Button from "./Button.svelte";
    import type { FunctionEvent } from "./types";
    import { StepForward, X } from "@lucide/svelte";

    let isClosing = false;

    const {
        title = "overlay",
        submitText = "submit",
        onsubmit,
        onclose,
        children,
    }: {
        title?: string;
        submitText?: string;
        onclose: FunctionEvent;
        onsubmit: FunctionEvent;
        [key: string]: any;
    } = $props();
</script>

<form
    onsubmit={(event) => {
        event.preventDefault();
        if (isClosing) return;
        isClosing = true;
        onsubmit;
    }}
    class="size-full z-100 bg-black/50 absolute flex justify-center items-center p-3"
>
    <div
        class="min-w-10 min-h-10 max-w-full max-h-full p-4 bg-neutral-800 rounded-xl flex flex-col gap-4"
    >
        <div class="w-full flex flex-row space-x-2">
            <p class="text-2xl font-bold w-full">{title}</p>
            <Button
                Icon={X}
                onclick={() => {
                    if (isClosing) return;
                    onclose();
                }}
                compact
            />
        </div>
        <div class="max-h-full overflow-y-auto">
            {@render children()}
        </div>
        <div class="flex space-x-2">
            <Button
                onclick={onsubmit}
                fill_width
                type="submit"
                Icon={StepForward}
                text={submitText}
            />
        </div>
    </div>
</form>
