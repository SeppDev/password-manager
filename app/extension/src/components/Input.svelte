<script lang="ts">
    let {
        title = "title",
        type = "text",
        value = $bindable(),
        onInput,
    }: {
        title?: string;
        type?: "text" | "password";
        value?: string;
        onInput?: (event: Event) => Promise<void> | void;
    } = $props();
</script>

<input
    oninput={onInput}
    placeholder={title}
    bind:value
    id={title}
    {type}
    class="not-dark:black z-10 w-full rounded-lg px-4 py-3 text-base ring-blue-400 outline-1 outline-neutral-400 duration-100 focus:ring-2 focus:outline-blue-600 dark:text-white"
/>
