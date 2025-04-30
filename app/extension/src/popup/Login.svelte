<script lang="ts">
    import Input from "../components/Input.svelte";
    import Button from "../components/Button.svelte";
    import Loader from "../components/Loader.svelte";
    import config from "../config";

</script>

<span class="flex flex-col">
    <span class="bg-gray-900 w-full flex p-3 items-center">
        <p>AuraPass</p>
    </span>
    <span
        class="flex grow-1 h-90 w-80 flex-col items-center justify-center m-4 gap-10"
    >
        <p class="font-bold text-xl">Login</p>
        <!-- <form class="size-full flex flex-col gap-4">
            {#if loginState.errorMessage !== undefined}
                <div
                    class="w-full rounded-lg bg-red-200 not-dark:text-red-800 not-dark:outline-1 not-dark:outline-red-400 dark:bg-red-500 dark:text-red-100"
                >
                    <p class="px-4 py-2">{loginState.errorMessage}</p>
                </div>
            {/if}

            {#if loginState.phase === "continue"}
                <Input
                    value={loginState.username}
                    onInput={onInputUsername}
                    title="username"
                />
            {:else if loginState.phase === "login" || loginState.phase == "create account"}
                <p class="text-2xl font-bold">{loginState.username}</p>
                <Input
                    title="password"
                    value={loginState.password}
                    onInput={onInputPassword}
                    type="password"
                />
            {/if}
            <Button
                onclick={continueButton}
                text={loginState.phase}
                type="submit"
            />

            {#if loginState.phase !== "continue"}
                <Button
                    fill_width={false}
                    onclick={() => {
                        loginState.phase = "continue";
                    }}
                    text="back"
                    type="button"
                />
            {/if}
        </form> -->
    </span>
</span>

<!-- <script lang="ts">
    import { onMount } from "svelte";
    import Loader from "../shared/Loader.svelte";
    import Config from "../config";
    import Button from "../shared/Button.svelte";

    type Response = {
        message?: string;
        token?: string;
    };

    let errorMessage: string | undefined = loginState(undefined);
    let loading = loginState(false);
    let buttonState: "login" | "signup" = loginState("login");

    function switchButtonState() {
        buttonState =
            buttonState === "login"
                ? (buttonState = "signup")
                : (buttonState = "login");
    }

    async function handleResponse(response: Response) {
        errorMessage = response.message;
        if (!response.token) return;
        page = "loggedin";
        document.cookie = `token=${response.token};`;
    }

    async function login(event: Event) {
        event.preventDefault();

        if (loading) return;
        loading = true;

        const username = document.getElementById(
            "Username",
        ) as HTMLInputElement;
        const password = document.getElementById(
            "Password",
        ) as HTMLInputElement;

        try {
            const response = await fetch(`${Config.api}/login`, {
                method: "POST",
                headers: {
                    username: username.value,
                    password: password.value,
                },
            });

            const json = (await response.json()) as Response;
            await handleResponse(json);
        } catch {
            errorMessage = "Something went wrong";
        }

        loading = false;
    }

    async function signup(event: Event) {
        event.preventDefault();

        if (loading) return;
        loading = true;

        const username = document.getElementById(
            "Username",
        ) as HTMLInputElement;
        const password = document.getElementById(
            "Password",
        ) as HTMLInputElement;

        try {
            const response = await fetch(`${Config.api}/signup`, {
                method: "POST",
                headers: {
                    username: username.value,
                    password: password.value,
                },
            });

            const json = (await response.json()) as Response;
            await handleResponse(json);
        } catch {
            errorMessage = "Something went wrong";
        }

        loading = false;
    }

    function signout() {
        document.cookie = "token=;expires=Thu, 01 Jan 1970 00:00:01 GMT";
        page = "register";
    }
</script>

<div class="flex h-100 w-70 flex-col items-center justify-center gap-10">
    {#if page === "register"}
        <form
            class="flex w-100 flex-col items-center justify-center gap-3 rounded-xl p-6 shadow-xl not-sm:size-full not-sm:rounded-none not-dark:bg-white dark:bg-neutral-900 dark:shadow-black"
        >
            <p class="w-full text-left text-2xl font-bold">Login</p>
            {#if errorMessage !== undefined}
                <div
                    class="w-full rounded-lg bg-red-200 not-dark:text-red-800 not-dark:outline-1 not-dark:outline-red-400 dark:bg-red-500 dark:text-red-100"
                >
                    <p class="px-4 py-2">{errorMessage}</p>
                </div>
            {/if}

            {@render input("Username", "text")}
            {@render input("Password", "password")}
            <hr />
            {#if buttonState === "login"}
                {@render loaderButton("Login", login, "Don't have an account?")}
            {:else}
                {@render loaderButton(
                    "Sign up",
                    signup,
                    "Already have an account?",
                )}
            {/if}
        </form>
    {:else if page === "loading"}
        <div class="aspect-square w-20">
            <Loader />
        </div>
    {:else if page === "loggedin"}
        <p class="text-4xl">Logged in</p>
        <div class="flex flex-col items-center gap-4">
            <Button onclick={signout} text="Signout"></Button>
            <div
                class="NotDetected flex flex-col items-center gap-2 rounded-xl bg-neutral-800 p-4"
            >
                <p class="font-2xl font-bold text-red-500">
                    Extension not detected
                </p>
                <Button href="download" text="Download"></Button>
            </div>
        </div>
    {:else}
        <p class="text-3xl font-black">Oops!</p>
        <p>Something went wrong</p>
    {/if}
</div>

{#snippet loaderButton(
    text: string,
    onclick: (event: Event) => Promise<void>,
    other: string,
)}
    <Button type="submit" {onclick} {text} fill_width={true} />
    <button
        onclick={switchButtonState}
        class="cursor-pointer text-center text-sm text-blue-500 duration-100 hover:text-blue-600"
        >{other}</button
    >
{/snippet}

{#snippet input(title: string, type: "text" | "password" | "email")}
    <div class="w-full">
        <p
            class="relative top-2 left-4 w-fit px-2 py-0 text-sm text-neutral-400 not-dark:bg-white dark:bg-neutral-900"
        >
            {title}
        </p>
        <input
            id={title}
            {type}
            class="not-dark:black z-10 w-full rounded-lg px-4 py-3 text-base ring-blue-400 outline-1 outline-neutral-400 duration-100 focus:ring-2 focus:outline-blue-500 dark:text-white"
        />
    </div>
{/snippet} -->
