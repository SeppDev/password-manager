<script lang="ts">
    import { Delete, Plus, RefreshCcw, X } from "@lucide/svelte";
    import Button from "../components/Button.svelte";
    import Input from "../components/Input.svelte";
    import Overlay from "../components/Overlay.svelte";
    import type { FunctionEvent } from "../components/types";
    import { randomPassword } from "../util/crypto";
    import { sleep } from "../util/sleep";
    import type { Account } from "../user/account";
    import removeItem from "../util/removeItem";
    import { writable, type Writable } from "svelte/store";

    let {
        mode = "create",
        account,
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
        account?: Account;
    } = $props();

    let title = $state(account?.title);
    let email = $state(account?.email);
    let username = $state(account?.username);
    let password = $state(account?.password);

    async function defaultURL(): Promise<string[]> {
        let tabs = await browser.tabs.query({
            currentWindow: true,
            active: true,
        });
        let activeTab = tabs[0];
        let tabURL = activeTab?.url;
        if (!tabURL) return [];
        let activeURL = new URL(tabURL);
        title = activeURL.host;
        return [activeURL.host];
    }

    let urls: string[] = $state((account?.urls || []).map((a) => a));
    (async () => {
        if (urls.length === 0) urls = await defaultURL();
    })();

    function filterDuplicates<T>(array: T[]): T[] {
        return Array.from(new Set(array.map((item) => item)));
    }

    async function onsubmit() {
        await onsave(title, email, username, password, filterDuplicates(urls));
    }
</script>

<Overlay
    title={mode === "create" ? "create account" : "edit account"}
    submitText={mode}
    {onsubmit}
    {onclose}
>
    <div class="flex flex-col w-150 gap-2 max-w-full">
        <Input compact placeholder="title" type="text" bind:value={title} />
        <span class="h-2"></span>
        <Input compact placeholder="email" type="email" bind:value={email} />
        <Input
            compact
            placeholder="username"
            type="text"
            bind:value={username}
        />
        <div class="flex flex-row w-full gap-2">
            <Input
                compact
                placeholder="password"
                fill_width
                bind:value={password}
                type="text"
            />
            <Button
                compact
                Icon={RefreshCcw}
                onclick={() => {
                    password = randomPassword();
                }}
            />
        </div>
        <span class="h-2"></span>
        <div class="w-full gap-2 flex flex-col">
            {#each urls as url, index}
                <div class="w-full flex gap-2">
                    <Input
                        placeholder="url"
                        type="url"
                        compact
                        fill_width
                        value={url}
                        onInput={(event) => {
                          const target = event.target as HTMLInputElement | null;
                          if (!target) return;
                            urls[index] = target.value;
                        }}
                    />
                    <Button
                        compact
                        Icon={X}
                        onclick={() => {
                            urls = removeItem(
                                urls.map((a) => a),
                                index,
                            );
                        }}
                    />
                </div>
            {/each}
            <Button
                compact
                Icon={Plus}
                onclick={() => {
                    urls.push("");
                }}
            />
        </div>
    </div>
</Overlay>
