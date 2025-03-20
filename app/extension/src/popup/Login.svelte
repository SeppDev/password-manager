<script lang="ts">
    import Loader from "../components/Loader.svelte";
    import Config from "../config";

    let loading = $state(false);

    async function login(event: Event) {
        event.preventDefault();
        if (loading) return;
        loading = true;

        const username = document.getElementById("Username") as HTMLInputElement;
        const password = document.getElementById("Password") as HTMLInputElement;

        const response = await fetch(`${Config.api}/signup`, {
            method: "POST",
            headers: {
                "username": username.value,
                "password": password.value
            }
        });

        loading = false;
    
    }
</script>

<div class="flex items-center justify-center w-full h-100 max-m-screen">
    <div class="bg-gray-900 w-80 rounded-2xl">
        <form class="flex flex-col w-full h-full gap-4 p-3">
            {@render input("Username", "text")}
            {@render input("Password", "password")}
            <hr class="text-gray-800" />
            <button
                type="submit"
                onclick={login}
                class="w-auto p-2 font-bold duration-200 bg-indigo-900 cursor-pointer h-11 hover:bg-indigo-950 rounded-xl"
            >
                {#if loading}
                    <div class="flex items-center justify-center h-full">
                        <Loader />
                    </div>
                {:else}
                    <p>Login</p>
                {/if}
            </button>
            <a
                href="/skibidi"
                class="text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-700"
                >Don't have an account yet?</a
            >
        </form>
    </div>
</div>

{#snippet input(title: string, type: "text" | "password" | "email")}
    <div class="flex flex-col gap-1">
        <p class="text-base">{title}</p>
        <input
            id={title}
            name={title}
            {type}
            class="px-2.5 py-1.5 outline-1 outline-gray-800 prl rounded-xl"
        />
    </div>
{/snippet}
