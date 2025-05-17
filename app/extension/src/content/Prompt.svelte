<script lang="ts">
    import { Save, X } from "@lucide/svelte";
    import {
        openSavePrompt,
        pollSavePrompt,
        submitSavePrompt,
    } from "../util/channels";

    let opened = $state(false);
    let edit = $state(false);
    let email: string | undefined = $state(undefined);
    let username: string | undefined = $state(undefined);

    openSavePrompt.onMessage((info) => {
        if (!info) {
            opened = false;
            return;
        }
        edit = info.edit === true;
        opened = true;
        email = info.inputs.email;
        username = info.inputs.username;
    });

    pollSavePrompt.sendMessage();
</script>

{#if opened}
    <div
        class="bg-neutral-900 outline-1 outline-black w-[400px] rounded-[12px] p-2 text-white"
    >
        <p class="font-bold text-[22px]">
            {edit ? "Edit account" : "Save account"}
        </p>
        <div class="flex flex-row gap-[8px] items-end">
            <div
                class="grow px-[8px] text-white flex flex-col justify-start items-start"
            >
                {#if email}
                    <p class="text-[16px] text-neutral-500">{email}</p>
                {/if}
                {#if username}
                    <p class="text-[16px] text-neutral-500">{username}</p>
                {/if}
            </div>

            <button
                onclick={() => submitSavePrompt.sendMessage(false)}
                class="cursor-pointer h-full flex justify-center items-center bg-red-600 text-black hover:bg-red-500 rounded-[10px] p-[12px] duration-200"
            >
                <X />
            </button>
            <button
                onclick={() => submitSavePrompt.sendMessage(true)}
                class="cursor-pointer h-full flex justify-center items-center bg-blue-600 text-black hover:bg-blue-500 rounded-[10px] p-[12px] duration-200"
            >
                <Save />
            </button>
        </div>
    </div>
{/if}
