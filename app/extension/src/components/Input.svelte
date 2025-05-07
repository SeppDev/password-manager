<script lang="ts">
    let {
        title = "title",
        type = "text",
        value = $bindable(),
        compact = false,
        fill_width = false,
        grow = 0,
        onInput,
    }: {
        title?: string;
        type?: "text" | "password";
        value?: string;
        compact?: boolean;
        fill_width?: boolean;
        grow?: number;
        onInput?: (event: Event) => Promise<void> | void;
    } = $props();
</script>

<input
    oninput={onInput}
    placeholder={title}
    bind:value
    id={title}
    {type}
    class="{fill_width ? 'w-full' : ''} {compact
        ? 'text-sm h-8 indent-3'
        : 'text-xl h-13 indent-4'} grow-{grow} not-dark:black z-10 rounded-lg text-base ring-blue-400 border-1 border-neutral-400 duration-100 focus:ring-2 focus:outline-none focus:border-blue-600 dark:text-white"
/>
