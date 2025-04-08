import type { Account } from "../types/Account";

export class Storage {
    private session_token = String;
    private authenticated = Boolean;
    private accounts: Account[];

    constructor() {
        this.accounts = [];
    }
    
};

let storage = new Storage();
