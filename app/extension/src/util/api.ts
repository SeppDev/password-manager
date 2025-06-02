import type { VaultData } from "../background/vaultManager";
import config from "../config";
import { getEncryptionPassword, getToken } from "../user/userData";
import { decrypt, encrypt } from "./crypto";

export async function fetchUserData(
  token: string,
): Promise<VaultData | undefined> {
  let password = await getEncryptionPassword();
  if (!password) return;

  const response = await fetch(`${config.api}/userdata`, {
    method: "get",
    headers: {
      token,
    },
  });
  if (response.status !== 200) return;

  let data = await response.text();
  if (data.length === 0) return;


  let decrypted = await decrypt(password, data);
  return JSON.parse(decrypted);
}

export async function updateUserData(vault: string) {
  const token = await getToken();
  if (!token) return;

  let password = await getEncryptionPassword();
  if (!password) return;

  let encrypted = await encrypt(password, vault);

  await fetch(`${config.api}/userdata`, {
    method: "post",
    headers: {
      token,
    },
    body: encrypted
  });
}
