import config from "../config";
import type { Account } from "../user/account";
import { getEncryptionPassword, getToken } from "../user/userData";
import { decrypt, encrypt } from "./encryption";

export type Vault = {
  label?: String;
  accounts: Account[];
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

  let decrypted = await decrypt(password, text).catch(() => "{}");
  let json = JSON.parse(decrypted) as Vault;
  if (json.accounts === undefined) json.accounts = [];

  return [json];
}

export async function updateUserData(vaults: Vault[]) {
  const token = await getToken();
  if (!token) return;

  let password = await getEncryptionPassword();
  if (!password) return;

  let vault = vaults[0];
  let text = JSON.stringify(vault);
  let data = await encrypt(password, text);

  const response = await fetch(`${config.api}/userdata`, {
    method: "post",
    headers: {
      token,
      data,
    },
  });
}
