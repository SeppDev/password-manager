import type { Account } from "../user/account";
import { updateUserData, type Vault } from "../util/api";
import { BrowserMessages } from "../util/browserMessages";
import { sleep } from "../util/sleep";

let synced = false;
let cachedVaults: Vault[] | undefined = undefined;

export const syncVaults = new BrowserMessages<Vault[]>("SyncBackgroundVaults");
syncVaults.onMessage((data) => {
  cachedVaults = data;
  synced = false;
});

browser.runtime.onConnect.addListener((c) => {
  c.postMessage(cachedVaults);
});

async function main() {
  while (true) {
    if (synced === false || !cachedVaults) {
      await sleep(100);
      continue;
    }
    console.log("SYNCING TO SERVER");
    updateUserData(cachedVaults);
    synced = true;
  }
}
main();
