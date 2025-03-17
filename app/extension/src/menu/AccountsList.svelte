<script lang="ts">
    type Account = {
      username: string | undefined,
      email: string | undefined
    }

    const { accounts, selected }: {
      accounts: string[],
      selected: (username: string) => void
    } = $props();
</script>

<div class="w-full gap-2 mt-0 mb-3">
    <p class="pt-2 pb-1 pl-3 m-0 text-sm text-left text-white">
        Saved accounts
    </p>

    <div class="flex flex-col gap-0">
        {#each accounts as account}
            <button
                onclick={() => selected(account)}
                class="h-10 text-base font-bold text-white duration-200 border-none cursor-pointer bg-none hover:bg-blue-700"
                ><p class="ml-5 text-left">{account}</p></button
            >
        {/each}
    </div>
</div>
