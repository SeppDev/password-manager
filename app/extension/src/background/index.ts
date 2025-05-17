import { writable, type Writable } from "svelte/store";
import type { Account } from "../user/account";
import { fetchUserData, updateUserData } from "../util/api";
import { BrowserMessages } from "../util/browserMessages";
import { sleep } from "../util/sleep";
import { generateId, getToken } from "../user/userData";
import "./vaultManager";
import { VaultManager } from "./vaultManager";
import {
  createAccount,
  deleteAccount,
  initVault,
  newSavePrompt,
  openSavePrompt,
  pollAccountsForSite,
  pollSavePrompt,
  sendAccountsList,
  submitSavePrompt,
  syncPopup,
  trashAccount,
} from "../util/channels";
import type { Inputs, PromptData } from "../components/types";

let vaultManager: VaultManager | undefined = undefined;
let connected = false;

createAccount.onMessage((info) => {
  if (!vaultManager) return;

  const account: Account = {
    id: generateId(),
    title: info.title,
    email: info.email,
    username: info.username,
    password: info.password,
    urls: info.urls,
  };

  vaultManager.pushAccount(info.vault, account);
});

trashAccount.onMessage((data) => {
  vaultManager?.trashAccount(data.account);
  updatePopup();
});
deleteAccount.onMessage((data) => {
  vaultManager?.deleteAccount(data.account);
  updatePopup();
});

function updatePopup() {
  if (!connected) return;
  if (!vaultManager) return;
  if (!vaultManager.synced) return;
  try {
    syncPopup.sendMessage(vaultManager.vaultsList());
  } catch {}
}

browser.runtime.onConnect.addListener((port) => {
  connected = true;
  updatePopup();

  port.onDisconnect.addListener(() => {
    connected = false;
  });
});

async function initVaultManager() {
  const token = await getToken();
  if (!token) return;
  const userData = await fetchUserData(token);
  vaultManager = new VaultManager(token);
  if (userData) {
    vaultManager.init(userData);
    updatePopup();
    return;
  }
  vaultManager.createVault();
  updatePopup();
}

initVault.onMessage(() => {
  initVaultManager;
});
initVaultManager();

setTimeout(async () => {
  while (true) {
    await sleep(100);
    if (!vaultManager) continue;
    if (vaultManager.synced) continue;

    try {
      updateUserData(vaultManager.stringify());
    } catch (e) {
      console.warn(e);
    }
    vaultManager.synced = true;
    updatePopup();
  }
});

let promptQueu: PromptData[] = [];
function checkQueu() {
  openSavePrompt.sendMessage(promptQueu[0]);
}

pollSavePrompt.onMessage(checkQueu);

submitSavePrompt.onMessage((shouldSave) => {
  if (!vaultManager) return;
  let data = promptQueu.pop();
  if (!data) throw "No data found";
  checkQueu();
  if (!shouldSave) return;
  let foundAccount = vaultManager.find(data.inputs.username);
  if (foundAccount) {
    if (data.inputs.password) {
      foundAccount.password = data.inputs.password;
    }
    return;
  }

  vaultManager?.pushAccount(undefined, {
    email: data.inputs.email,
    username: data.inputs.username || data.inputs.text,
    password: data.inputs.password,
    urls: [data.url],
  });
});

newSavePrompt.onMessage((data) => {
  if (!vaultManager) return;

  let url = new URL(data.url);
  promptQueu = promptQueu.filter(
    (d) => new URL(d.url).hostname !== url.hostname,
  );
  data.inputs = {
    email: data.inputs.email,
    username: data.inputs.username || data.inputs.text,
    password: data.inputs.password,
  };

  let exists = vaultManager.find(data.inputs.username) !== undefined;
  data.edit = exists;

  promptQueu.push(data);
  checkQueu();
});

async function updateAccountList(tabId: number) {
  if (!vaultManager) return;

  let tab = await browser.tabs.get(tabId);
  if (!tab.url) return;
  if (!tab.id) return;

  let url = new URL(tab.url);

  let accounts = vaultManager.getAllAccounts().filter((account) => {
    let includes = false;
    for (const acc_url of account.urls || []) {
      if (includes) break;
      let a_url = new URL(acc_url);
      includes = a_url.hostname === url.hostname;
    }
    return includes;
  });

  sendAccountsList.sendMessage(accounts, tab.id);
}

pollAccountsForSite.onMessage(async (data) => {
  let tabs = await browser.tabs.query(data);
  tabs.map((tab) => {
    const id = tab.id;
    if (!id) return;
    updateAccountList(id);
  });
});

browser.tabs.onUpdated.addListener(updateAccountList);
browser.tabs.onCreated.addListener((tab) => {
  const id = tab.id;
  if (!id) return;
  updateAccountList(id);
});
