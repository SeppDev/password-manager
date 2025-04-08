import type { Account } from "../types/Account";

export class Storage {
    session_token = String;
    authenticated = Boolean;
    accounts: Account[];

    constructor() {
        this.accounts = [];
    }
}


export class StorageSyncer extends Storage {
    constructor() {
        
    }
};