<script lang="ts">
    import Loader from "./Loader.svelte";
    import LockIcon from "../assets/LockIcon.svelte";
    import AccountsList from "../menu/AccountsList.svelte";
    import { FillAccount, Submit } from "../content/FillForm";
    
    type Page = "loading" | "notsigned" | "home";

    let visible = $state(true);
    let page: Page = $state("loading");

    const accounts = ["SeppDev", "SkibidiToiletFan", "student2"];
    function clicked(button: HTMLElement) {
        visible = !visible;
    }
    function selected(name: string) {
        visible = false;
        FillAccount(`${name}@supercoolmail.com`, name, "Password123");
        // Submit();
    }
    setTimeout(() => {
        page = "home";
        visible = true;
    }, 1    );
</script>   

<div class="w-full h-full z-100">
    <style>
        @import "tailwindcss";
    </style>

    <button
        onclick={(event) => clicked(event.currentTarget)}
        class="w-full h-full rounded-full cursor-pointer bg-indigo-950 hover:outline-2 hover:outline-indigo-600"
        aria-label=" "
    >
        <LockIcon />
    </button>

    <div class="h-1"></div>

    {#if visible}
        <div class="absolute right-0 bg-indigo-950 min-w-50 min-h-20 rounded-xl">
            {#if page === "loading"}
                <div class="flex items-center justify-center h-20">
                    <div class="h-8">
                        <Loader />
                    </div>
                </div>
            {:else if page === "notsigned"}
                <p>Please sign in first</p>
            {:else if page == "home"}
                <AccountsList {accounts} {selected} />
            {/if}
        </div>
    {/if}
</div>
