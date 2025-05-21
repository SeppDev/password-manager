<script lang="ts">
    import type { Account } from "../../user/account";

    const {
        accounts,
        select,
    }: {
        accounts: Account[];
        select: (account: Account) => void;
    } = $props();
</script>

<div class="w-full gap-2 mt-0 text-left">
    <p class="pt-2 text-neutral-500 pb-1 pl-3 m-0">Saved accounts</p>

    <div class="flex flex-col gap-0">
        {#each accounts as account}
            <button
                onclick={() => select(account)}
                class="text-left px-4 py-1 rounded-lg bg-inherit cursor-pointer hover:bg-neutral-800 duration-200"
            >
                <p class="text-lg">{account.username}</p>
            </button>
        {/each}
    </div>
</div>
