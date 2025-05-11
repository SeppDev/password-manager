<script lang="ts">
    import {
        getToken,
        deleteToken,
        IsAuthenticated,
        generateId,
    } from "../user/userData";
    import Loader from "../components/Loader.svelte";
    import type { Account } from "../user/account";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import { fetchUserData, type Vault } from "../util/api";
    import { syncVaults } from "../background";
    import { writable, type Writable } from "svelte/store";
    import { randomString } from "../util/crypto";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();
    let synced = $state(false);
    let search = $state("");

    let displayingAccounts: Writable<Account[]> = writable([]);
    let selectedAccount: Account | undefined = $state(undefined);

    let vaults: Writable<Vault[]> = writable([]);
    async function updateVaults() {
        syncVaults.sendMessage($vaults);
    }

    function createAccount(username: string) {
        if (!synced) return;
        const vault = $vaults[0];
        if (!vault) return;

        let account: Account = {
            email: `${username}@email.com`,
            username,
            password: "password123",
            urls: ["auth.google.com/"],
        };
        let id;
        while (!id) {
            let generated = generateId();
            if (vault.accounts[generated]) continue;
            id = generated;
        }

        vault.accounts[id] = account;
        updateVaults();
    }

    let port = browser.runtime.connect();
    port.onMessage.addListener((data) => {
        const cachedVaults = data as Vault[];
        vaults.set(cachedVaults);
        page = "home";
    });

    async function main() {
        const token = await getToken();
        if (!token || !(await IsAuthenticated())) {
            authenticate();
            window.close();
            return;
        }

        // const userData = await fetchUserData();
        // if (!userData) {
        //     authenticate();
        //     window.close();
        //     return;
        // }
        setTimeout(() => {
            synced = true;
        }, 200);

        // vaults = userData;
        // updateVaults();
    }
    main();

    setInterval(() => {
        let first: Vault | undefined = $vaults[0];
        if (!first) return;
        console.log(first.accounts);
        displayingAccounts.set(
            Object.entries(first.accounts).map(([id, account]) => {
                account.id = id;
                return account;
            }),
        );
    }, 100);

    async function logout() {
        deleteToken();
        authenticate();
        window.close();
    }

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
    // <Button text="logout" onclick={logout} />
</script>

<div class="overflow-hidden min-h-20 min-w-20">
    {#if page === "loading"}
        <Loading />
    {:else if page === "home"}
        <div class="w-160 bg-neutral-900 flex flex-col relative">
            <div class="flex flex-row w-full h-100">
                <!-- <div class="bg-green-500 h-full w-12"></div> -->
                {@render Accounts()}
            </div>
            {#if synced === false}
                <div
                    class="w-full min-h-10 gap-x-6 flex justify-center items-center bg-black"
                >
                    <p>Syncing data</p>
                    <div class="h-6 w-6">
                        <Loader />
                    </div>
                </div>
            {/if}
        </div>
    {:else}
        <div class="flex items-center justify-center">
            <p>Something went wrong!</p>
        </div>
    {/if}
</div>

{#snippet Menu()}
    <div class="h-full w-10 flex flex-col justify-center items-center">
        {@render Option("Vault", logout)}
        {@render Option("Settings", logout)}
        {@render Option("Logout", logout)}
    </div>
{/snippet}

{#snippet Accounts()}
    <div class="grow flex flex-col h-full">
        <header
            class="gap-x-2 p-2 flex justify-center items-center center w-full"
        >
            <Button prevent_default compact text="" />
            <Input compact title="search" grow value={search} />
            <Button
                prevent_default
                compact
                text="create item"
                onclick={() => createAccount("account")}
            />
        </header>
        <div class="grow flex-row flex overflow-hidden max-w-full">
            <div class="flex flex-col overflow-y-auto h-full grow">
                {#each $displayingAccounts as account}
                    {@render Account(account)}
                {/each}
            </div>
            {#if selectedAccount}
                <div
                    class="h-full w-2/3 overflow-y-auto overflow-x-hidden flex flex-col justify-start items-start gap-4 p-4"
                >
                    {@render AccountDetail("email", selectedAccount?.email)}
                    {@render AccountDetail(
                        "username",
                        selectedAccount?.username,
                    )}
                    {@render AccountDetail(
                        "password",
                        selectedAccount?.password,
                        true,
                    )}
                    <div
                        class="py-2 px-3 outline-1 outline-neutral-500 w-full flex-col rounded-xl"
                    >
                        <p class="text-neutral-500 text-sm">urls</p>
                        {#each selectedAccount.urls || [] as url}
                            <p>{url}</p>
                        {/each}
                    </div>
                    <div
                        class="py-2 px-3 outline-neutral-500 w-full overflow-y-auto flex-col flex items-start justify-start"
                    >
                        <p class="text-neutral-500 text-sm">details</p>
                        <p
                            class="text-sm text-neutral-600 max-w-full text-ellipsis whitespace-nowrap"
                        >
                            ID: {selectedAccount.id}
                        </p>
                    </div>
                </div>
            {/if}
        </div>
    </div>
{/snippet}

{#snippet AccountDetail(
    title: string,
    value: string | undefined,
    dotted?: boolean,
)}
    {#if value !== undefined}
        <button
            onclick={() => {
                navigator.clipboard.writeText(value);
            }}
            class="outline-1 outline-neutral-500 w-full flex-col flex justify-center items-start rounded-xl py-2 px-3 duration-300 cursor-pointer hover:bg-neutral-800"
        >
            <p class="text-neutral-500 text-sm">{title}</p>
            {#if dotted}
                <div
                    class="size-full flex flex-row justify-start items-center space-x-1 py-2"
                >
                    {#each { length: 10 } as _}
                        <span class="bg-white size-2 rounded-full"> </span>
                    {/each}
                </div>
            {:else}
                <p>{value}</p>
            {/if}
        </button>
    {/if}
{/snippet}

{#snippet Account(account: Account)}
    <button
        onclick={() => {
            selectedAccount = account;
        }}
        class="min-h-14 bg-neutral-900 px-3 cursor-pointer gap-x-4 hover:bg-neutral-800 duration-200 flex flex-row justify-start items-center"
    >
        <div class="bg-white rounded-full h-3/5 aspect-square"></div>
        <div class="grow-1 flex flex-col items-start">
            <p>{account.username || account.email}</p>
            {#if account.urls && account.urls.length > 0}
                <p class="text-neutral-400 font-sm text-nowrap text-sm">
                    {account.urls[0]}
                </p>
            {/if}
        </div>
    </button>
{/snippet}

{#snippet Option(name: string, action: (event: Event) => Promise<void> | void)}
    <button
        onclick={action}
        class="h-full px-4 cursor-pointer hover:bg-blue-700 duration-200"
    >
        {name}
    </button>
{/snippet}

{#snippet Loading()}
    <div class="flex items-center justify-center w-20 h-20">
        <div class="h-10">
            <Loader />
        </div>
    </div>
{/snippet}
