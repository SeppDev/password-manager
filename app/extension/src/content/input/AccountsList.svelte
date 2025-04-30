<script lang="ts">
    import type { Account } from "../types/account";

    const {
        accounts,
        selected,
    }: {
        accounts: string[];
        selected: (username: string) => void;
    } = $props();
</script>

<div class="w-full gap-2 mt-0 mb-3">
    <p class="pt-2 pb-1 pl-3 m-0 text-sm font-semibold text-left">
        Saved accounts
    </p>

    <div class="flex flex-col gap-0">
        {#each accounts as account}
            <button
                onclick={() => selected(account)}
                class="h-10 text-base font-bold duration-200 border-none cursor-pointer bg-none dark:hover:bg-blue-500 not-dark:hover:bg-blue-800"
                ><p class="ml-5 text-left">{account}</p></button
            >
        {/each}
    </div>
</div>
