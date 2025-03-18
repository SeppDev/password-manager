<script lang="ts">
    import MenuIcon from "../assets/MenuIcon.svelte";
    import PlusIcon from "../assets/PlusIcon.svelte";
    import Loader from "../components/Loader.svelte";
    import EditIcon from "../assets/EditIcon.svelte";

    import type { Account } from "../types/Account.ts";
    import EllipsisVertical from "../assets/EllipsisVertical.svelte";

    const { test } = $props();

    let page: "loading" | "home" = $state("loading");

    let activeAccount: Account | undefined = $state(undefined);
    let accounts: Array<Account> = $state([]);

    for (let i = 1; i <= 10; i++) {
        const account: Account = {
            Title: `Account-${i}`,
            Username: `User-${i}`,
            Email: "coolemail@mail.com",
            Password: undefined,
            URLs: [],
        };

        accounts.push(account);
    }

    activeAccount = accounts[0];

    setTimeout(() => {
        page = "home";
    }, 50);
</script>

<div class="overflow-hidden">
    {#if page === "loading"}
        <Loading />
    {:else if page == "home"}
        <Home />
    {/if}
</div>

{#snippet Loading()}
    <div class="flex items-center justify-center w-20 h-20">
        <div class="h-10">
            <Loader />
        </div>
    </div>
{/snippet}

{#snippet Home()}
    <div class="flex flex-col w-auto h-auto">
        <Topbar />
        <Acounts />
    </div>
{/snippet}

{#snippet Acounts()}
    <div class="flex overflow-y-auto bg-indigo-950 w-150 h-80">
        <div class="h-full overflow-y-auto w-50">
            {#each accounts as account}
                {@render AccountItem(account)}
            {/each}
        </div>
        {#if activeAccount}
            {@render AccountInfo(activeAccount)}
        {:else}
            <div class="flex items-center justify-center h-full grow-1">
                <p class="text-gray-500">No account selected</p>
            </div>
        {/if}
    </div>
{/snippet}

{#snippet AccountInfo(account: Account)}
    <div class="flex flex-col justify-start m-4 grow-1">
        <div class="flex w-full h-8 gap-3">
            <b class="grow-1">{account.Title}</b>
            <button
                class="flex items-center justify-center gap-3 p-4 duration-200 bg-indigo-600 bg-opacity-50 rounded-full cursor-pointer hover:bg-indigo-700"
            >
                <div class="h-4 aspect-square">
                    <EditIcon />
                </div>
                <p>Edit</p>
            </button>
            <button
                class="flex items-center justify-center duration-200 rounded-full cursor-pointer aspect-square hover:bg-indigo-900"
            >
            <div class="flex items-center justify-center w-4 h-4">
                <EllipsisVertical />

            </div>
            </button>
        </div>
        <div class="w-full p-1 grow-1 bg-pink-450">
            <p>{account.Username}</p>
        </div>
    </div>
{/snippet}

{#snippet AccountItem(account: Account)}
    <button
        onclick={() => (activeAccount = account)}
        class="flex flex-row items-center justify-start w-full gap-3 p-2 text-center duration-200 cursor-pointer hover:bg-indigo-800"
    >
        <div class="h-8 p-1 bg-black rounded-full aspect-square"></div>
        <div class="flex flex-col items-start justify-center h-full">
            <b class="text-sm">{account.Title}</b>
            <p class="text-xs text-gray-400">{account.Username}</p>
        </div>
    </button>
{/snippet}

{#snippet Topbar()}
    <div class="flex w-full gap-2 p-3 h-14">
        <button class="flex items-center justify-center h-full cursor-pointer"
            ><MenuIcon /></button
        >
        <div
            class="w-full h-full font-bold border border-none rounded-full outline-none bg-indigo-950"
        >
            <input
                class="w-full h-full p-3 font-bold border-none outline-none bg-none"
                placeholder="Search"
            />
        </div>
        <button
            class="flex items-center justify-center h-full duration-200 rounded-full cursor-pointer bg-indigo-950 aspect-square hover:bg-indigo-800"
            ><PlusIcon /></button
        >
    </div>
{/snippet}
