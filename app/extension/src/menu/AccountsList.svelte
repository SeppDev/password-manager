<script lang="ts">
    import type { Account } from "../types/account";

    const { accounts, selected }: {
      accounts: string[],
      selected: (username: string) => void
    } = $props();
</script>

<div class="w-full gap-2 mt-0 mb-3">
    <p class="pt-2 pb-1 pl-3 m-0 text-sm text-left text-black">
        Saved accounts
    </p>

    <div class="flex flex-col gap-0">
        {#each accounts as account}
            <button
                onclick={() => selected(account)}
                class="h-10 text-base font-bold text-black duration-200 border-none cursor-pointer bg-none hover:bg-blue-600"
                ><p class="ml-5 text-left">{account}</p></button
            >
        {/each}
    </div>
</div>
