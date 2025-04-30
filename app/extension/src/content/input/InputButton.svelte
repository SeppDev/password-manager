<script lang="ts">
    import Loader from "../../components/Loader.svelte";
    import LockIcon from "../../assets/LockIcon.svelte";
    import { FillAccount } from "../fillForm";
    import AccountsList from "./AccountsList.svelte";

    type Page = "loading" | "notsigned" | "home";

    let visible = $state(false);
    let page: Page = $state("loading");

    const accounts = ["SeppDev", "SkibidiToiletFan", "student"];
    function clicked() {
        visible = !visible;
    }

    function selected(name: string) {
        visible = false;
        FillAccount(`${name}@supercoolmail.com`, name, "Password123");
    }
    setTimeout(() => {
        page = "home";
    }, 1);
</script>

<div class="absolute z-10 w-full h-full not-dark:text-white dark:text-black">
    <style>
        @import "tailwindcss";
    </style>

    <button
        onclick={(_) => clicked()}
        class="w-full h-full duration-200 bg-blue-600 rounded-full cursor-pointer stroke-white dark:stroke-black dark:hover:bg-blue-500 not-dark:hover:bg-blue-800"
        aria-label=" "
    >
        <LockIcon />
    </button>

    <div class="h-1"></div>

    {#if visible}
        <div
            class="absolute right-0 text-white bg-blue-600 min-w-50 min-h-20 rounded-xl dark:text-black"
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
                <AccountsList {accounts} {selected} />
            {/if}
        </div>
    {/if}
</div>
