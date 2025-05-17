<script lang="ts">
    import { RefreshCcw } from "@lucide/svelte";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import Overlay from "../components/Overlay.svelte";
    import type { FunctionEvent } from "../components/types";
    import { randomPassword } from "../util/crypto";
    import { sleep } from "../util/sleep";

    let {
        mode = "create",
        title = $bindable(""),
        email = $bindable(""),
        username = $bindable(""),
        password = $bindable(randomPassword()),
        urls = $bindable([]),

        onclose,
        onsave,
    }: {
        onclose: FunctionEvent;
        onsave: (
            title: string | undefined,
            email: string | undefined,
            username: string | undefined,
            password: string | undefined,
            urls: string[],
        ) => Promise<void>;

        mode: "create" | "edit";
        title?: string;
        username?: string;
        email?: string;
        password?: string;
        urls?: string[];
    } = $props();

    browser.tabs.query({ currentWindow: true, active: true }).then((tabs) => {
        let activeTab = tabs[0];
        let tabURL = activeTab?.url;
        if (!tabURL) return;
        let activeURL = new URL(tabURL);
        title = activeURL.hostname;
        urls.push(activeURL.origin);
    });

    async function onsubmit() {
        await onsave(title, email, username, password, urls);
    }
</script>

<Overlay
    title={mode === "create" ? "create account" : "edit account"}
    submitText={mode}
    {onsubmit}
    {onclose}
>
    <div class="flex flex-col w-150 gap-2 max-w-full">
        <Input placeholder="title" type="text" bind:value={title} />
        <span class="h-2"></span>
        <Input placeholder="email" type="email" bind:value={email} />
        <Input placeholder="username" type="text" bind:value={username} />
        <div class="flex flex-row w-full gap-2">
            <Input
                placeholder="password"
                fill_width
                bind:value={password}
                type="text"
            />
            <Button
                Icon={RefreshCcw}
                onclick={() => {
                    password = randomPassword();
                }}
            />
        </div>
        <span class="h-2"></span>
        <div class="w-full">
            {#each urls as url}
                <Input placeholder="urls" type="url" fill_width value={url} />
            {/each}
        </div>
    </div>
</Overlay>
