<script lang="ts">
    import Menu from "../assets/Menu.svelte";
    import Plus from "../assets/Plus.svelte";
    import Loader from "../components/Loader.svelte";

    const { test } = $props();

    let page: "loading" | "home" = $state("loading");

    let accounts: Array<String> = $state([]);

    for (let i=0; i < 100; i++) {
        accounts.push(`account-${i}`);
    }

    setTimeout(() => {
        page = "home";
    }, 500);
</script>

<div class="m-2 overflow-hidden">
    {#if page === "loading"}
        <Loading />
    {:else if page == "home"}
        <Home />
    {/if}
</div>

{#snippet Loading()}
    <div class="flex items-center justify-center w-20 h-20">
        <div class="h-10">
            <Loader />
        </div>
    </div>
{/snippet}

{#snippet Home()}
    <div class="flex flex-col w-auto h-auto gap-2">
        <Topbar />
        <Acounts />
    </div>
{/snippet}

{#snippet Acounts()}
    <div class="flex m-2 overflow-y-auto w-120 h-60 bg-indigo-950 rounded-xl">
        <div class="w-40 h-full overflow-y-auto">
            {#each accounts as account}
                <Account {account} />
            {/each}
        </div>
        <div class="w-full h-full bg-green-500">
            <p>E-mail</p>
            <p>Username</p>
            <p>Password</p>
            <p>URLs</p>
            <p>Notes</p>
        </div>
    </div>
{/snippet}

{#snippet Account(username: String)}
    <div class="flex items-center justify-center w-full h-5 pt-4 pb-4 text-center bg-red-500">
        <p>{username}</p>
    </div>
{/snippet}

{#snippet Topbar()}
    <div class="flex w-full h-8 gap-2">
        <button class="flex items-center justify-center h-full cursor-pointer"
            ><Menu /></button
        >
        <div
            class="w-full h-full font-bold border border-none rounded-full outline-none bg-indigo-950"
        >
            <input
                class="w-full h-full p-3 font-bold border-none outline-none bg-none"
                placeholder="Search"
            />
        </div>
        <button
            onclick={() => accounts.push("yes")}
            class="flex items-center justify-center h-full duration-200 rounded-full cursor-pointer bg-indigo-950 aspect-square hover:bg-indigo-800"
            ><Plus /></button
        >
    </div>
{/snippet}
