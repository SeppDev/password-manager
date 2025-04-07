import config from "../config";
import type { Account } from "../types/Account";

const storage = {
    session_token: "",
    authenticated: false,
    accounts: [] as Account[]
};

export type Storage = typeof storage;

async function sendStorage() {
    await browser.runtime.sendMessage({
        type: "storage-update",
        storage,
    });
}

async function fetchAccounts(): Promise<Storage | undefined> {
    if (!storage) return;

    let response = await fetch(`${config.api}/userdata`, {
        credentials: "same-origin",
        headers: {
            "Cookie": `token=${storage.session_token}`,
        },
    });

    const text = await response.text();
    const data = atob(text);

    return JSON.parse(data);
}

async function updateAccounts() {
    if (!storage) return;

    let response = await fetch(`${config.api}/userdata`, {
        method: "POST",
        credentials: "same-origin",
        headers: {
            "Cookie": `token=${storage.session_token}`,
        },
        body: btoa(JSON.stringify(storage))
    });
}

function createAccount() {
    storage.accounts.push({
        Title: "cool",
        URLs: []
    });
}


browser.runtime.onMessage.addListener(async (msg, _, _response) => {
    if (msg.type === "session-token") {
        storage.session_token = msg.token;
    } else if (msg.type === "sync-storage") {
        await sendStorage();
    } else if (msg.type === "update-storage") {
        await updateAccounts();
    } else if (msg.type === "create-account") {
        createAccount();
        await updateAccounts();
    }
});