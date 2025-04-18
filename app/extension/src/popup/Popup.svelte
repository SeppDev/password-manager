<script lang="ts">
    import { writable } from "svelte/store";

    import MenuIcon from "../assets/MenuIcon.svelte";
    import PlusIcon from "../assets/PlusIcon.svelte";
    import Loader from "../components/Loader.svelte";
    import EditIcon from "../assets/EditIcon.svelte";

    import type { Account } from "../types/account.ts";
    import EllipsisVertical from "../assets/EllipsisVertical.svelte";
    import Button from "../components/Button.svelte";
    import { openLoginPage } from "../common/loginPage";
    import { type Accounts, accountsSync } from "../background/userData";

    let page: "loading" | "home" | "login" | "error" = $state("loading");

    let activeAccount: Account | undefined = $state(undefined);

    let accounts: Accounts | undefined = $state(undefined);
    accountsSync.onMessage(async (value) => {
        accounts = value;
        page = "home";
    });
</script>

<div class="overflow-hidden min-h-20 min-w-20">
    {#if page === "loading"}
        <Loading />
    {:else if page === "login"}
        <Login />
    {:else if page === "home"}
        <Home />
    {:else if page === "error"}
        <div class="flex items-center justify-center">
            <p>Something went wrong!</p>
        </div>
    {/if}
</div>

{#snippet Login()}
    <div class="flex flex-col items-center justify-center gap-4 h-50 w-100">
        <p class="text-4xl font-medium">Aurapass</p>
        <Button onclick={openLoginPage}>Register</Button>
    </div>
{/snippet}

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
    <div class="flex overflow-y-auto bg-gray-950 w-150 h-80">
        <div class="h-full overflow-y-auto w-50">
            {#if accounts}
                {#each accounts as account}
                    {@render AccountItem(account)}
                {/each}
            {/if}
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
                class="flex items-center justify-center gap-3 p-4 duration-200 bg-blue-600 bg-opacity-50 rounded-full cursor-pointer hover:bg-blue-700"
            >
                <div class="h-4 aspect-square">
                    <EditIcon />
                </div>
                <p>Edit</p>
            </button>
            <button
                class="flex items-center justify-center duration-200 rounded-full cursor-pointer aspect-square hover:bg-blue-900"
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
        class="flex flex-row items-center justify-start w-full gap-3 p-2 text-center duration-200 cursor-pointer hover:bg-blue-800"
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
            class="w-full h-full font-bold text-white border border-none rounded-full outline-white bg-none"
        >
            <input
                class="w-full h-full p-3 font-bold border-none outline-none bg-none"
                placeholder="Search"
            />
        </div>
        <button
            class="flex items-center justify-center h-full duration-200 bg-blue-500 rounded-full cursor-pointer aspect-square hover:bg-blue-600"
            ><PlusIcon /></button
        >
    </div>
{/snippet}
