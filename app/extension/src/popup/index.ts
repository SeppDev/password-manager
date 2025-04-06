import { mount } from "svelte";
import Popup from "./Popup.svelte";
import config from "../config";
import type { Storage } from "../background";

async function loginPage() {
    await browser.tabs.create({
        url: `${config.base}/register`,
    });
}

let storage: Storage | undefined = undefined;

async function fetchUserData(): Promise<String | undefined> {
    if (!storage) return;

    console.log(storage.session_token);

    let response = await fetch(`${config.api}/userdata`, {
        credentials: "same-origin",
        headers: {
            "Cookie": `token=${storage.session_token}`,
        },
    });

    const text = await response.text();
    console.log(text);

    const data = atob(text);

    return "yes";
}

browser.runtime.onMessage.addListener((msg) => {
    if (msg.type !== "storage-update") return;
    storage = msg.storage as Storage;
})

const port = browser.runtime.connect({ name: "popup-connection" });
port.onMessage.addListener((msg) => {
    storage = msg as Storage;
});

async function render() {
    const target = document.getElementById("app");
    if (!target) return;

    await browser.runtime.sendMessage({type: "sync-storage"});

    mount(Popup, { target, props: { loginPage, fetchUserData } });
}

document.addEventListener("DOMContentLoaded", render);
