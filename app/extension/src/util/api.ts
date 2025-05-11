import config from "../config";
import type { Account } from "../user/account";
import { generateId, getEncryptionPassword, getToken } from "../user/userData";
import { decrypt, encrypt } from "./crypto";

export type Vault = {
  label?: string;
  accounts: { [key: string]: Account };
};

export async function fetchUserData(): Promise<Vault[] | undefined> {
  const token = await getToken();
  if (!token) return;
  let password = await getEncryptionPassword();
  if (!password) return;

  const response = await fetch(`${config.api}/userdata`, {
    method: "get",
    headers: {
      token,
    },
  });
  let text = await response.text();
  let encryptedData = atob(text);
  let decrypted = await decrypt(password, encryptedData).catch(() => "{}");
  let json = JSON.parse(decrypted || "{}") as Vault;
  if (json.accounts === undefined) json.accounts = {};

  // json.accounts = json.accounts.map((account) => {
  //   if (account.id) return account;
  //   account.id = generateId();
  //   return account
  // });

  return [json];
}

export async function updateUserData(vaults: Vault[]) {
  const token = await getToken();
  if (!token) return;

  let password = await getEncryptionPassword();
  if (!password) return;

  let vault = vaults[0];
  let text = JSON.stringify(vault);
  let encrypted = await encrypt(password, text);
  let data = btoa(encrypted);

  const response = await fetch(`${config.api}/userdata`, {
    method: "post",
    headers: {
      token,
      data,
    },
  });
}
