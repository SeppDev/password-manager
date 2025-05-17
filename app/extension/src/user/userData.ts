import config from "../config";
import { randomString } from "../util/crypto";
import type { Account } from "./account";

// Change to memory on release
const storage = browser.storage.local;

export function generateId(): string {
  return randomString(12);
}

export async function getToken(): Promise<string | undefined> {
  const data = await storage.get("token");
  return data.token;
}

export async function setToken(token: string) {
  await storage.set({ token });
}

export async function deleteToken() {
  await storage.set({ token: undefined });
}

export async function getEncryptionPassword(): Promise<string | undefined> {
  const data = await storage.get("password");
  return data.password;
}

export async function IsAuthenticated(token?: string): Promise<boolean> {
  token = token || (await getToken());
  if (!token) {
    deleteToken();
    return false;
  }

  let response;
  while (true) {
    try {
      response = await fetch(`${config.api}/authenticated`, {
        headers: { token },
      });
      break;
    } catch {}
  }

  return response.status === 200;
}

// export async function syncUserData(): Promise<boolean> {
//   const token = await getToken();
//   if (!token) return false;

//   const response = await fetch(`${config.api}/userdata`, {
//     headers: {
//       token,
//     },
//   });

//   const text = await response.text();
//   console.log(text);

//   return true;
// }
