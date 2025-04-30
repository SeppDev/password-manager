<script lang="ts">
    import { IsAuthenticated, logout } from "../user/userData";

    // import { writable } from "svelte/store";
    // import type { Writable } from "svelte/store";

    import Loader from "../components/Loader.svelte";
    import Button from "../components/Button.svelte";

    // import type { Account } from "../user/account.ts";
    // import { onMount } from "svelte";
    // import Login from "./Login.svelte";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();

    async function main() {
        const authenticated = await IsAuthenticated();
        if (!authenticated) {
            authenticate();
            return;
        }
        page = "home";
    }
    main();

    // let activeAccount: Account | undefined = $state(undefined);
    // let accounts: Account[] = $state([]);

    // onMount(async () => {
    //     const authenticated = await IsAuthenticated();
    //     page.set(authenticated ? "home" : "login");

    //     async function updateAccounts() {
    //         const accs = getAccounts();
    //     }
    //     browser.storage.onChanged.addListener(updateAccounts);
    //     updateAccounts();
    // });
</script>

<div class="overflow-hidden min-h-20 min-w-20">
    <p>Popup</p>
    {#if page === "loading"}
        <Loading />
    {:else if page === "home"}
        <p>Home</p>
        <Button
            text="logout"
            onclick={() => {
                logout();
                authenticate();
                window.close();
            }}
        />
    {:else}
        <div class="flex items-center justify-center">
            <p>Something went wrong!</p>
        </div>
    {/if}
</div>

{#snippet Loading()}
    <div class="flex items-center justify-center w-20 h-20">
        <div class="h-10">
            <Loader />
        </div>
    </div>
{/snippet}

<!--

{#snippet Home()}
    <div class="flex flex-col w-auto h-auto">
        <p>Home</p>
        <Topbar />
        <Acounts />
    </div>
{/snippet} -->

<!-- {#snippet Acounts()}
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
{/snippet} -->
