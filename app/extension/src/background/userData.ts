import { mount } from "svelte";
import { openLoginPage } from "../common/loginPage";
import config from "../config";
import type { Account } from "../types/account";
import Popup from "../popup/Popup.svelte";
import { BrowserMessages } from "../common/browserMessages";

export const accountsSync = new BrowserMessages<Accounts>("accountsSync");


let accounts = [] as Account[];
export type Accounts = typeof accounts;

accountsSync.onMessage(async () => {
    
})

let authenticated = false;
let session_token: String | undefined = undefined;

(async () => {
    let data = await browser.storage.local.get(["session_token"]);
    let token = data["session_token"];

    if (!token) {
        return;
    }

    session_token = token;
})();

export type UserDataSync = {
    type: "sync";
    accounts: Accounts;
};

browser.runtime.onConnect.addListener(async (port) => {
    if (port.name !== "userdata") return;
    return accounts;
});

async function createAccount() {
    let accounts = await getAccounts();

    accounts.push({
        Title: "String",
        URLs: [],
    });

    await overrideAccounts(accounts);
}

async function getAccounts(): Promise<Account[]> {
    let data = (await browser.storage.session.get(["accounts"]))["accounts"];
    return data ? data : [];
}

function overrideAccounts(accounts: Account[]) {
    browser.storage.session.set({ accounts });
}

async function fetchAccounts(): Promise<boolean> {
    if (!session_token) return false;

    let response = await fetch(`${config.api}/userdata`, {
        credentials: "same-origin",
        headers: {
            Cookie: `token=${session_token}`,
        },
    });

    if (response.status !== 200) return false;

    const text = await response.text();
    let data = JSON.parse(atob(text));

    if (!data.isArray) {
        data = [];
    }

    accounts = data;
    return true;
}

async function checkAuthenticated() {
    if (!session_token) return false;

    let response = await fetch(`${config.api}/authenticated`, {
        credentials: "same-origin",
        headers: {
            Cookie: `token=${session_token}`,
        },
    });

    return response.status === 200
}
