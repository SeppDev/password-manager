<script lang="ts">
    const {
        accounts,
        select,
    }: {
        accounts: string[];
        select: (index: number) => void;
    } = $props();
</script>

<div class="w-full gap-2 mt-0 mb-3 text-left">
    <p class="pt-2 text-neutral-500 pb-1 pl-3 m-0">Saved accounts</p>

    <div class="flex flex-col gap-0">
        {#each accounts as account, i}
            <button
                onclick={() => select(i)}
                class="text-left px-4 py-1 bg-inherit cursor-pointer hover:bg-neutral-800"
            >
                <p class="text-lg">{account}</p>
            </button>
        {/each}
    </div>
</div>
