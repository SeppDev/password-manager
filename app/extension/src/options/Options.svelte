<script lang="ts">
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import config from "../config";
    import { setToken } from "../user/userData";

    type Response = {
        message?: string;
        token?: string;
    };

    let errorMessage: string | undefined = $state(undefined);
    let username = $state("");
    let password = $state("");
    let page: "continue" | "create account" | "login" | "done" =
        $state("continue");

    let closeInterval = $state(4);
    async function startCloseInterval() {
        setInterval(() => {
            closeInterval--;
            if (closeInterval < 1) {
                window.close();
            }
        }, 1000);
    }

    async function handleResponse(response: Response) {
        errorMessage = response.message;
        if (!response.token) return;
        await setToken(response.token);
        page = "done";
        startCloseInterval();
    }

    async function usernameSubmit() {
        if (username.length < 1) {
            errorMessage = "please provide a username";
            return;
        }
        const response = await fetch(`${config.api}/user/exists/${username}`);

        const exists: boolean = (await response.json()).message == "true";
        page = exists ? "login" : "create account";
    }

    async function loginSubmit() {
        const response = await fetch(`${config.api}/login`, {
            method: "POST",
            headers: {
                username,
                password,
            },
        });

        const json = (await response.json()) as Response;
        await handleResponse(json);
    }

    async function signUpSubmit() {
        const response = await fetch(`${config.api}/signup`, {
            method: "POST",
            headers: {
                username,
                password,
            },
        });

        const json = (await response.json()) as Response;
        await handleResponse(json);
    }

    async function continueButton() {
        errorMessage = undefined;
        switch (page) {
            case "continue":
                await usernameSubmit();
                break;
            case "create account":
                await signUpSubmit();
                break;
            case "login":
                await loginSubmit();
                break;
        }
    }
</script>

<div class="h-dvh w-dvw flex flex-col items-center justify-center p-4">
    {#if page !== "done"}
        <form class="w-full flex flex-col gap-4 items-left">
            <p class="text-3xl font-bold text-center">Log in to Aurapass</p>

            {#if errorMessage !== undefined}
                <div
                    class="w-full rounded-lg bg-red-200 not-dark:text-red-800 not-dark:outline-1 not-dark:outline-red-400 dark:bg-red-500 dark:text-red-100"
                >
                    <p class="px-4 py-2">{errorMessage}</p>
                </div>
            {/if}
            {#if page === "continue"}
                <Input bind:value={username} title="username" />
            {:else if page === "login" || page == "create account"}
                <p class="text-2xl font-bold">{username}</p>
                <Input title="password" bind:value={password} type="password" />
            {/if}
            <Button type="submit" onclick={continueButton} text={page} />

            {#if page !== "continue"}
                <Button
                    onclick={() => {
                        page = "continue";
                    }}
                    text="back"
                    type="button"
                />
            {/if}
        </form>
    {:else}
        <span class="flex flex-col justify-center items-center gap-6">
            <p class="text-3xl font-bold text-center">Logged in</p>
            <Button
                text="close ({closeInterval})"
                onclick={() => {
                    window.close();
                }}
            />
        </span>
    {/if}
</div>
