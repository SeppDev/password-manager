<script lang="ts">
    import { Dices, KeyRound } from "@lucide/svelte";
    import Loader from "../../components/Loader.svelte";
    import { FillAccount } from "../fillForm";
    import AccountsList from "./AccountsList.svelte";
    import type { Account } from "../../user/account";
    import { writable, type Writable } from "svelte/store";
    import {
        pollAccountsForSite,
        sendAccountsList,
        sendUsed,
    } from "../../util/channels";
    import { randomPassword } from "../../util/crypto";
    import Button from "../../components/Button.svelte";
    import Logo from "../../assets/icons/logo.png";

    type Page = "loading" | "notsigned" | "home";

    let visible = $state(false);
    let page: Page = $state("home");

    let accounts: Account[] = $state([]);
    function clicked() {
        visible = !visible;
        pollAccountsForSite.sendMessage({ url: document.URL });
    }

    function select(account: Account) {
        visible = false;
        FillAccount(account.email, account.username, account.password);
        if (!account.id) return;
        sendUsed.sendMessage({ accountId: account.id });
    }

    sendAccountsList.onMessage((accs) => {
        page = "home";
        accounts = accs;
    });
    pollAccountsForSite.sendMessage({ url: document.URL });
</script>

<div class="absolute z-10 w-full h-full not-dark:text-black dark:text-white">
    <button
        onclick={(_) => clicked()}
        class="w-full h-full flex justify-center items-center cursor-pointer"
        aria-label=" "
    >
        <img src={Logo} alt="logo" />
    </button>

    <div class="h-1"></div>

    {#if visible}
        <div
            class="absolute p-2 right-0 bg-neutral-900 w-[300px] min-h-[10px] rounded-xl"
        >
            {#if page === "loading"}
                <div class="flex items-center justify-center h-20">
                    <div class="h-8">
                        <Loader />
                    </div>
                </div>
            {:else if page === "notsigned"}
                <p>Please sign in first</p>
            {:else if page == "home"}
                <Button
                    compact
                    fill_width
                    Icon={Dices}
                    text="generate password"
                    onclick={() => {
                        visible = false;
                        const password = randomPassword();
                        FillAccount(undefined, undefined, password);
                    }}
                />
                {#if accounts.length > 0}
                    <AccountsList {accounts} {select} />
                {:else}
                    <div class="flex justify-center items-center h-12">
                        <p>No saved accounts</p>
                    </div>
                {/if}
            {/if}
        </div>
    {/if}
</div>
