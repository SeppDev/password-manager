import config from "../config";
import type { Account } from "../types/Account";

let accounts: Account[] = [];
let session_token: String | undefined = undefined;
let authenticated: boolean = false;

export class StorageSyncer {
    async fetch() {
        let response = await fetch(`${config.api}/userdata`, {
            credentials: "same-origin",
            headers: {
                Cookie: `token=${session_token}`,
            },
        });
        
        if (response.status !== 200) return;
        
        const text = await response.text();
        let data = JSON.parse(atob(text));

        if (!data.isArray) {
           data = []; 
        }

        accounts = data;    
    }

    sync(): Account[] {
        return accounts;
    }

    createAccount() {
        accounts.push({
            Title: "String",
            URLs: [],
        });
    }
}
