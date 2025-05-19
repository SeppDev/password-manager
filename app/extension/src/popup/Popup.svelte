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
    import { writable, type Writable } from "svelte/store";
    import AccountEditor from "./AccountEditor.svelte";

    import { Pencil, Trash2 } from "@lucide/svelte";

    import type { Vault } from "../background/vaultManager";
    import type { Component, Snippet } from "svelte";
    import {
        createAccount,
        editAccount,
        syncPopup,
        trashAccount,
    } from "../util/channels";
    import { sleep } from "../util/sleep";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();
    let search = $state("");

    let displayingAccounts: Writable<Account[]> = writable([]);
    let selectedAccount: Account | undefined = $state(undefined);
    let synced = $state(false);

    const vaults: Writable<Vault[]> = writable([]);
    browser.runtime.connect();

    syncPopup.onMessage((v) => {
        vaults.set(v);
        page = "home";
        synced = true;
    });

    async function untilSynced() {
        while (!synced) await sleep(10);
    }

    async function main() {
        const token = await getToken();
        if (!token || !(await IsAuthenticated())) {
            authenticate();
            window.close();
            return;
        }
    }
    main();

    async function logout() {
        deleteToken();
        authenticate();
        window.close();
    }

    let activeOverlays: Snippet[] = $state([]);
    function closeOverlay() {
        activeOverlays.pop();
    }

    function createItemPrompt() {
        activeOverlays.push(CreateAccount);
    }
    function openEditAccountPrompt() {
        activeOverlays.push(EditAccount);
    }

    async function onAccountEdit(
        title: string | undefined,
        email: string | undefined,
        username: string | undefined,
        password: string | undefined,
        urls: string[],
    ) {
        if (!selectedAccount) throw "No active account found";
        if (!selectedAccount.id) throw "No id for account";

        synced = false;
        let account: Account = {
            id: selectedAccount.id,
            title,
            email,
            username,
            password,
            urls,
        };
        selectedAccount = account;
        editAccount.sendMessage(account);
        await untilSynced();
        closeOverlay();
    }

    async function onAccountCreate(
        title: string | undefined,
        email: string | undefined,
        username: string | undefined,
        password: string | undefined,
        urls: string[],
    ) {
        const vault = $vaults[0];
        if (!vault) throw "Failed to find a vault";
        const vaultId = vault.id;

        const info = $state.snapshot({
            vault: vaultId,
            title,
            email,
            username,
            password,
            urls,
        });
        synced = false;

        createAccount.sendMessage(info);
        await untilSynced();
        closeOverlay();
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

{#snippet CreateAccount()}
    <AccountEditor
        mode="create"
        onsave={onAccountCreate}
        onclose={closeOverlay}
    />
{/snippet}
{#snippet EditAccount()}
    <AccountEditor
        mode="edit"
        account={selectedAccount}
        onsave={onAccountEdit}
        onclose={closeOverlay}
    />
{/snippet}

<div class="overflow-hidden min-h-20 min-w-20">
    {#if page === "loading"}
        <Loading />
    {:else if page === "home"}
        <div class="w-160 bg-neutral-900 flex flex-col relative">
            <div class="flex flex-row w-full h-130">
                {#each activeOverlays as overlay}
                    {@render overlay()}
                {/each}
                <!-- <div class="bg-green-500 h-full w-12"></div> -->
                {@render Accounts()}
            </div>
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
            <Input compact placeholder="search" grow value={search} />
            <Button
                prevent_default
                compact
                text="create item"
                onclick={() => createItemPrompt()}
            />
        </header>
        <div class="grow flex-row flex overflow-hidden max-w-full">
            <div
                class="flex flex-col overflow-y-auto overlow-x-hidden h-full grow"
            >
                {#each $vaults as vault}
                    {#each Object.values(vault.accounts) as account}
                        {@render Account(account)}
                    {/each}
                {/each}
            </div>
            {#if selectedAccount}
                <span class="w-0 border-[1px] border-neutral-600 h-full"></span>
                <div
                    class="h-full w-2/3 overflow-y-auto overflow-x-hidden flex flex-col justify-start items-start gap-4 px-4"
                >
                    <div class="flex justify-end w-full h-8 gap-2">
                        <Button
                            onclick={() => {
                                const id = selectedAccount?.id;
                                if (!id) return;
                                openEditAccountPrompt();
                            }}
                            Icon={Pencil}
                            theme="secondary"
                            compact
                            text="edit"
                        />
                        <Button
                            onclick={() => {
                                const id = selectedAccount?.id;
                                if (!id) return;
                                selectedAccount = undefined;
                                trashAccount.sendMessage({
                                    account: id,
                                });
                            }}
                            Icon={Trash2}
                            theme="secondary"
                            text="trash"
                            compact
                        />
                    </div>
                    <p class="text-xl font-bold">{selectedAccount.title}</p>
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
    {#if value !== undefined && value.length > 0}
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
            if (selectedAccount && selectedAccount.id === account.id) {
                selectedAccount = undefined;
                return;
            }
            selectedAccount = account;
        }}
        class="min-h-14 overflow-x-hidden bg-neutral-900 px-3 cursor-pointer gap-x-4 hover:bg-neutral-800 duration-200 flex flex-row justify-start items-center"
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
