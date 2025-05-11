import { writable, type Writable } from "svelte/store";
import type { Account } from "../user/account";
import { fetchUserData, updateUserData, type Vault } from "../util/api";
import { BrowserMessages } from "../util/browserMessages";
import { sleep } from "../util/sleep";

let synced = false;
let cachedVaults: Vault[] | undefined = undefined;

const savePrompts: Writable<{ [key: string]: string }[]> = writable([]);
export const savePrompt = new BrowserMessages<{ [key: string]: string }>(
  "SavePrompt",
);
savePrompt.onMessage((data) => {
  savePrompts.update((values) => {
    values.push(data);
    return values;
  });
});
savePrompts.subscribe(() => {});

export const notifyBackground = new BrowserMessages<string>("Notify");
export const syncVaults = new BrowserMessages<Vault[]>("SyncBackgroundVaults");
syncVaults.onMessage((data) => {
  synced = false;
  cachedVaults = data;
});

notifyBackground.onMessage(async (message) => {
  if (message === "Sync") {
    cachedVaults = await fetchUserData();
  }
});

browser.runtime.onConnect.addListener(async (c) => {
  while (synced === false) await sleep(10);
  c.postMessage(cachedVaults);
});

async function main() {
  cachedVaults = await fetchUserData();
  synced = true;

  while (true) {
    if (synced === true || !cachedVaults) {
      await sleep(10);
      continue;
    }
    synced = true;
    await updateUserData(cachedVaults);
  }
}
main();
