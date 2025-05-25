<script lang="ts">
    import { getToken, deleteToken, IsAuthenticated } from "../user/userData";
    import Loader from "../components/Loader.svelte";
    import type { Account } from "../user/account";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import { writable, type Writable } from "svelte/store";
    import AccountEditor from "./AccountEditor.svelte";

    import { TOTP } from "totp-generator";

    import { Pencil, Trash2 } from "@lucide/svelte";

    import type { Vault } from "../background/vaultManager";
    import type { Snippet } from "svelte";
    import {
        createAccount,
        editAccount,
        syncPopup,
        trashAccount,
    } from "../util/channels";
    import { sleep } from "../util/sleep";
    import removeItem from "../util/removeItem";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();

    let selectedAccount: Account | undefined = $state(undefined);
    let synced = $state(false);

    let searchFilter = $state("");
    const filteredVaults: Writable<Vault[]> = writable([]);
    const vaults: Writable<Vault[]> = writable([]);
    function filterVault(vault: Vault): { [key: string]: Account } {
        let list: Account[] = Object.values(vault.accounts).filter(
            (account) =>
                account.username?.includes(searchFilter) ||
                account.title?.includes(searchFilter) ||
                account?.urls?.some((url) => url.includes(searchFilter)),
        );

        let dict: { [key: string]: Account } = {};
        list.forEach((account) => {
            if (!account.id) return;
            dict[account.id] = account;
        });
        return dict;
    }
    function filter() {
        let clone = $state.snapshot($vaults) as Vault[];
        if (searchFilter.length === 0) {
            filteredVaults.set(clone);
            return;
        }
        filteredVaults.set(
            clone.map((vault) => {
                vault.accounts = filterVault(vault);
                return vault;
            }),
        );
    }

    // Ping the background script
    browser.runtime.connect();

    syncPopup.onMessage((v) => {
        vaults.set(v);
        filter();
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
        totp: string | undefined,
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
            totp,
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
        totp: string | undefined,
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
            totp,
            urls,
        });
        synced = false;

        createAccount.sendMessage(info);
        await untilSynced();
        closeOverlay();
    }

    type Notification = {
        title?: string;
        description: string;
    };

    let notifications: Notification[] = $state([]);
    function notify(description: string, title?: string) {
        notifications.push({
            title,
            description,
        });
        setTimeout(() => {
            notifications = removeItem(notifications, 0);
        }, 1000);
    }
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
            <div class="w-full absolute z-10 bottom-4 pointer-events-none">
                <div
                    class="flex justify-end items-center h-full gap-4 flex-col pointer-events-none"
                >
                    {#each notifications as notification}
                        <div class="bg-neutral-800 p-4 rounded-xl">
                            {notification.description}
                        </div>
                    {/each}
                </div>
            </div>
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
            <!-- <Button prevent_default compact text="" /> -->
            <Input
                compact
                onInput={filter}
                placeholder="search"
                grow
                bind:value={searchFilter}
            />
            <Button
                prevent_default
                compact
                text="create account"
                onclick={() => createItemPrompt()}
            />
        </header>
        <div class="grow flex-row flex overflow-hidden max-w-full">
            <div
                class="flex flex-col overflow-y-auto overlow-x-hidden h-full grow"
            >
                {#each $filteredVaults as vault}
                    <p class="pl-3 text-lg">{vault.label}</p>
                    {#each Object.values(vault.accounts) as account}
                        {@render Account(account)}
                    {/each}
                {/each}
            </div>
            {#if selectedAccount}
                <span class="w-0 border-[1px] border-neutral-600 h-full"></span>
                <div
                    class="h-full min-w-2/3 max-w-2/3 overflow-y-auto overflow-x-hidden flex flex-col justify-start items-start gap-4 px-4"
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
                                    accountId: id,
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
                    {#if selectedAccount.totp}
                        {@render AccountDetail(
                            "totp",
                            (() => {
                                if (!selectedAccount?.totp) return;
                                const { otp, expires } = TOTP.generate(
                                    selectedAccount?.totp,
                                );
                                return otp;
                            })(),
                        )}
                    {/if}
                    <div
                        class="py-2 px-3 outline-1 outline-neutral-500 w-full flex-col rounded-xl"
                    >
                        <p class="text-neutral-500 text-sm">urls</p>
                        {#each selectedAccount.urls || [] as url}
                            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <p
                                class="cursor-pointer text-blue-400"
                                onclick={() => {
                                    browser.tabs.create({
                                        url: `https://${url}`,
                                    });
                                }}
                            >
                                {url}
                            </p>
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
                notify(`copied ${title}`);
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
        class="min-h-14 max-h-14 overflow-x-hidden bg-neutral-900 px-3 cursor-pointer gap-x-4 hover:bg-neutral-800 duration-200 flex flex-row justify-start items-center"
    >
        {#if (account.urls || []).length > 0}
            <img
                onerror={(event: Event) => {
                    const target = event.target as HTMLImageElement;
                    target.style.display = "none";
                }}
                src="https://www.google.com/s2/favicons?domain={(account.urls ||
                    [])[0]}&sz=128"
                alt="favicon"
                class="bg-transparent h-3/5 aspect-square"
            />
        {/if}
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
