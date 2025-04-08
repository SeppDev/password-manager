import type { Account } from "../types/Account";
import config from "../config";
import { Storage } from "./storageHandler";

// const storage = {
//     session_token: "",
//     authenticated: false,
//     accounts: [] as Account[]
// };

const storage = new Storage();

browser.storage.session.set({
    storage
});


// export type Storage = typeof storage;

// async function sendStorage() {
//     await browser.runtime.sendMessage({
//         type: "storage-update",
//         storage,
//     });
// }

// async function fetchAccounts(): Promise<Account[] | undefined> {
//     if (!storage) return;

//     let response = await fetch(`${config.api}/userdata`, {
//         credentials: "same-origin",
//         headers: {
//             "Cookie": `token=${storage.session_token}`,
//         },
//     });

//     if (response.status !== 200) return;

//     const text = await response.text();
//     const data = atob(text);

//     return JSON.parse(data);
// }

// async function updateAccounts() {
//     if (!storage) return;

//     let data = btoa(JSON.stringify(storage));
//     console.log(data);

//     let response = await fetch(`${config.api}/userdata`, {
//         method: "POST",
//         credentials: "same-origin",
//         headers: {
//             "Cookie": `token=${storage.session_token}`,
//         },
//         body: data
//     });
// }

// function createAccount() {
//     storage.accounts.push({
//         Title: "cool",
//         URLs: []
//     });
// }


// browser.runtime.onMessage.addListener(async (msg, _, response) => {
//     if (msg.type === "session-token") {
//         storage.session_token = msg.token;
//     } else if (msg.type === "sync-storage") {
//         await sendStorage();
//     } else if (msg.type === "update-storage") {
//         await updateAccounts();
//     } else if (msg.type === "fetch-storage") {
//         let accounts = await fetchAccounts();
//         if (!accounts) return;
//         storage.accounts = accounts;
//     } else if (msg.type === "create-account") {
//         createAccount();
//         sendStorage();
//         await updateAccounts();
//     }
// });