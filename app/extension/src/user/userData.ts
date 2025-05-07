import config from "../config";
import type { Account } from "./account";

// TODO keep me signed in
const storage = browser.storage.local;

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

export async function IsAuthenticated(token?: string): Promise<boolean> {
  token = token || (await getToken());
  if (!token) {
    deleteToken();
    return false
  };

  let response = await fetch(`${config.api}/authenticated`, {
    headers: { token },
  });

  return response.status === 200;
}

export async function syncUserData(): Promise<boolean> {
  const token = await getToken();
  if (!token) return false;

  const response = await fetch(`${config.api}/userdata`, {
    headers: {
      token,
    },
  });

  const json = await response.text();
  // console.log(json);

  return true;
}

export async function getAccounts(): Promise<Account[]> {
  return [];
}
