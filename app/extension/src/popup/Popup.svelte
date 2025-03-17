<script lang="ts">
    import Loader from "../components/Loader.svelte";

    const { test } = $props();

    let page: "loading" | "home" = $state("loading");

    setTimeout(() => {
        page = "home";
    }, 500);
</script>

<div class="overflow-hidden h-50 w-100">
    {#if page === "loading"}
        <Loading />
    {:else if page == "home"}
        <Home />
    {/if}
</div>

{#snippet Loading()}
    <div class="flex items-center justify-center w-full h-full">
        <div class="h-10">
            <Loader />
        </div>
    </div>
{/snippet}

{#snippet Home()}
    <div class="flex items-center justify-center w-full h-full m-8">
        <input type="text" class="w-full">
    </div>
{/snippet}
