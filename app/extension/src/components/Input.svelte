<script lang="ts">
    let {
        id,
        placeholder = "input",
        type = "text",
        value = $bindable(),
        compact = false,
        fill_width = false,
        grow = false,
        onInput,
    }: {
        id?: string;
        placeholder?: string;
        type?: "text" | "password" | "email" | "url";
        value?: string;
        compact?: boolean;
        fill_width?: boolean;
        grow?: boolean;
        onInput?: (event: Event) => Promise<void> | void;
    } = $props();
</script>

<input
    oninput={onInput}
    {placeholder}
    bind:value
    {id}
    {type}
    class="{fill_width ? 'w-full' : ''} {compact
        ? 'text-sm h-8 indent-3'
        : 'text-xl h-13 indent-4'} {grow &&
        'grow'} not-dark:black dark:text-white z-10 rounded-lg text-base border-1 border-neutral-700 focus:border-1 focus:border-blue-500 focus:outline-none outline-none duration-200"
/>
