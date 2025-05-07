async function deriveKey(password) {
    // Convert password to a Uint8Array
    const encoder = new TextEncoder();
    const passwordBytes = encoder.encode(password);

    // Generate a salt (could be a random value stored in local storage)
    const salt = crypto.getRandomValues(new Uint8Array(16));

    // Derive a key using PBKDF2
    const key = await crypto.subtle.importKey(
        "raw", 
        passwordBytes, 
        { name: "PBKDF2" },
        false, 
        ["deriveKey"]
    );

    const derivedKey = await crypto.subtle.deriveKey(
        {
            name: "PBKDF2",
            salt: salt,
            iterations: 100000,
            hash: "SHA-256"
        },
        key,
        { name: "AES-GCM", length: 256 },
        false, 
        ["encrypt", "decrypt"]
    );

    return { derivedKey, salt };
}

async function encryptData(derivedKey, data) {
    const iv = crypto.getRandomValues(new Uint8Array(12)); // Initialization vector
    const encoder = new TextEncoder();
    const dataBytes = encoder.encode(data);

    // Encrypt the data
    const encryptedData = await crypto.subtle.encrypt(
        {
            name: "AES-GCM",
            iv: iv
        },
        derivedKey,
        dataBytes
    );

    return { encryptedData, iv }; // Return both encrypted data and IV
}

async function main() {
    const password = "your-login-password"; // This is the password entered by the user
    const seedPhrase = "your-seed-phrase"; // The seed phrase you want to encrypt

    // Derive a key from the login password
    const { derivedKey, salt } = await deriveKey(password);

    // Encrypt the seed phrase
    const { encryptedData, iv } = await encryptData(derivedKey, seedPhrase);

    // Store encrypted data and IV in local storage or IndexedDB (salt should be stored as well)
    console.log(new Uint8Array(encryptedData));  // Encrypted data
    console.log(new Uint8Array(iv));  // Initialization Vector (IV)
    console.log(new Uint8Array(salt));  // Salt used for PBKDF2
}

// main();


 async function getFavIcon(url: string) {
    let page = await fetch(url);
    let text = await page.text();
    console.log(text);
  }
  getFavIcon("google.com")
