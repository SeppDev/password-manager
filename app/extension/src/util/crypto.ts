import { assert } from "./assert";

// Helper: convert between string and ArrayBuffer
const encoder = new TextEncoder();
const decoder = new TextDecoder();

export function randomBytes(length: number): number[] {
  let array: number[] = [];
  array.push(...crypto.getRandomValues(new Uint8Array(length)));
  return array;
}
export function randomString(length: number): string {
  let buffer = String.fromCharCode(...randomBytes(length));
  return btoa(buffer);
}

const characters =
  "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-=!@#$%^&*()_+[]{};':,<.>/?";

export function randomPassword(length: number = 32): string {
  let string = "";
  for (let i = 0; i < length; i++) {
    let index = Math.floor(Math.random() * characters.length);
    let char = characters[index];
    assert(char !== undefined, `character is undefined ${index}`);
    string += char;
  }
  return string;
}

async function getKey(password: string, salt: Uint8Array): Promise<CryptoKey> {
  const keyMaterial = await crypto.subtle.importKey(
    "raw",
    encoder.encode(password),
    { name: "PBKDF2" },
    false,
    ["deriveKey"],
  );
  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt,
      iterations: 1_000_000,
      hash: "SHA-256",
    },
    keyMaterial,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"],
  );
}

export async function encrypt(password: string, data: string): Promise<string> {
  const salt = crypto.getRandomValues(new Uint8Array(16));
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const key = await getKey(password, salt);

  const encrypted = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv },
    key,
    encoder.encode(data),
  );

  // Combine salt + iv + ciphertext and base64 encode
  const combined = new Uint8Array(
    salt.length + iv.length + encrypted.byteLength,
  );
  combined.set(salt, 0);
  combined.set(iv, salt.length);
  combined.set(new Uint8Array(encrypted), salt.length + iv.length);

  return btoa(String.fromCharCode(...combined));
}

export async function decrypt(
  password: string,
  encryptedData: string,
): Promise<string> {
  const combined = Uint8Array.from(atob(encryptedData), (c) => c.charCodeAt(0));

  const salt = combined.slice(0, 16);
  const iv = combined.slice(16, 28);
  const data = combined.slice(28);

  const key = await getKey(password, salt);
  const decrypted = await crypto.subtle.decrypt(
    { name: "AES-GCM", iv },
    key,
    data,
  );

  return decoder.decode(decrypted);
}
