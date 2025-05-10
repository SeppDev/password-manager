<script lang="ts">
    import { getToken, deleteToken } from "../user/userData";
    import Loader from "../components/Loader.svelte";
    import type { Account } from "../user/account";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import { fetchUserData, type Vault } from "../util/api";
    import { syncVaults } from "../background";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();
    let synced = $state(false);
    let search = $state("");
    let displayingAccounts: Account[] = $state([]);

    let vaults: Vault[] = [];
    async function updateVaults() {
        syncVaults.sendMessage(vaults);
    }

    function createAccount(username: string) {
        if (!synced) return;

        let account: Account = {
            username,
            urls: ["auth.google.com/"],
        };
        vaults[0].accounts.push(account);
        updateVaults();
    }

    async function main() {
        const token = await getToken();
        if (!token) {
            authenticate();
            window.close();
            return;
        }

        let port = browser.runtime.connect();
        port.onMessage.addListener((data) => {
            const cachedVaults = data as Vault[] | undefined;
            if (!cachedVaults) return;
            vaults = cachedVaults;
        });
        page = "home";

        const userData = await fetchUserData();
        if (!userData) {
            authenticate();
            window.close();
            return;
        }
        synced = true;

        vaults = userData;
        updateVaults();
    }
    main();

    setInterval(() => {
        let first: Vault | undefined = vaults[0];
        if (!first) return;
        displayingAccounts = first.accounts;
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
        <div class="w-150 bg-neutral-900 flex flex-col relative">
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
            class="w-full gap-x-2 p-2 flex justify-center items-center center"
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
        <div class="flex-row flex grow overflow-hidden">
            <div class="flex flex-col overflow-y-auto grow-2">
                {#each displayingAccounts || [] as account}
                    {@render Account(account)}
                {/each}
            </div>
            <div
                class="h-full grow-3 overflow-y-auto flex flex-col items-center p-4 gap-4"
            >
                <Input fill_width title="email" />
                <Input fill_width title="username" />
                <Input fill_width type="password" title="password" />
            </div>
        </div>
    </div>
{/snippet}

{#snippet Account(account: Account)}
    <button
        class="min-h-14 bg-neutral-900 px-3 cursor-pointer gap-x-4 hover:bg-neutral-800 duration-200 flex flex-row justify-start items-center"
    >
        <div class="bg-white rounded-full h-3/5 aspect-square"></div>
        <div class="grow-1 flex flex-col items-start">
            <p>{account.username || account.email}</p>
            {#if account.urls.length > 0}
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
