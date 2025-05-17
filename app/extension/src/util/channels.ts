import type { Vault } from "../background/vaultManager";
import type { PromptData } from "../components/types";
import type { Account } from "../user/account";
import { BrowserMessages } from "./browserMessages";

export const syncPopup = new BrowserMessages<Vault[]>("syncPopup");
export const initVault = new BrowserMessages("initVaultManager");

export const openSavePrompt = new BrowserMessages<PromptData | undefined>(
  "openSavePrompt",
  true,
);
export const pollSavePrompt = new BrowserMessages("pollSavePrompt");
export const pollAccountsForSite = new BrowserMessages<{ url: string }>(
  "pollAccountsForSite",
);
export const sendAccountsList = new BrowserMessages<Account[]>(
  "sendAccountsList",
);

export const submitSavePrompt = new BrowserMessages<boolean>(
  "submitSavePrompt",
);
export const newSavePrompt = new BrowserMessages<PromptData>("newSavePrompt");

export const createAccount = new BrowserMessages<{
  vault: string;
  title: string | undefined;
  email: string | undefined;
  username: string | undefined;
  password: string | undefined;
  urls: string[];
}>("createAccount");
export const trashAccount = new BrowserMessages<{
  account: string;
}>("trashAccount");

export const deleteAccount = new BrowserMessages<{
  account: string;
}>("deleteAccount");
