<script lang="ts">
    import { getToken, IsAuthenticated, deleteToken } from "../user/userData";
    import Loader from "../components/Loader.svelte";
    import type { Account } from "../user/account";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";

    let page: "loading" | "home" | "error" = $state("loading");
    let { authenticate } = $props();

    let search: string = $state("");

    let accounts: Account[] = $state([]);
    function createAccount() {
        let account: Account = {
            username: "Account",
            urls: ["google.com"],
        };
        accounts.push(account);
    }
    for (let i = 0; i < 10; i++) {
        createAccount();
    }

    async function main() {
        const token = await getToken();
        if (!token) {
            authenticate();
            window.close();
        }

        page = "home";
        const authenticated = await IsAuthenticated();
        if (!authenticated) {
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
        <div class="w-120 h-80 flex flex-row relative">
            {@render Accounts()}
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
    <div class="w-full grow-1 flex-col overflow-hidden flex">
        <div class="w-full m-2 gap-2 flex justify-center items-center center">
            <Input compact title="search" grow={1} value={search} />
            <Button prevent_default compact text="Create Item" />
        </div>
        <div class="grow-1 overflow-y-auto">
            {#each accounts as account}
                {@render Account(account)}
            {/each}
        </div>
        <div class="h-full min-w-40"></div>
    </div>
{/snippet}

{#snippet Account(account: Account)}
    <button
        class="w-full h-14 bg-neutral-900 px-4 cursor-pointer hover:bg-neutral-800 duration-200"
    >
        <div class="grow-1 flex flex-col items-start">
            <p>{account.username || account.email}</p>
            {#if account.urls.length > 0}
                <p class="text-neutral-400 font-sm">{account.urls[0]}</p>
            {/if}
        </div>
        <div class="grow-1"></div>
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
