import type { Account } from "../user/account";
import { generateId } from "../user/userData";

export type VaultData = { trash: Vault; vaults: Vault[] };

export class VaultManager {
  readonly token: string;
  private trash: Vault;
  private vaults: { [key: string]: Vault };
  synced: boolean;

  constructor(token: string) {
    this.synced = true;
    this.token = token;
    this.vaults = {};

    this.trash = new Vault(undefined);
    this.trash.label = "trash";
  }
  init(vaultData: VaultData) {
    for (const data of vaultData.vaults) {
      let vault = new Vault(data.id);
      vault.init(data);
      this.vaults[vault.id] = vault;
    }
    this.trash.init(vaultData.trash);
  }
  stringify(): string {
    let object: VaultData = {
      trash: this.trash,
      vaults: Object.values(this.vaults),
    };

    return JSON.stringify(object);
  }
  createVault(): Vault {
    let vault = new Vault(undefined);
    this.vaults[vault.id] = vault;
    return vault;
  }
  pushAccount(vault: string | undefined, account: Account) {
    this.synced = false;
    if (vault) {
      this.vaults[vault].pushAccount(account);
      return;
    }
    let first = Object.values(this.vaults)[0];
    first.pushAccount(account);
  }
  getVault(id: string): Vault | undefined {
    return this.vaults[id];
  }
  vaultsList(): Vault[] {
    return Object.entries(this.vaults).map(([id, vault]) => {
      vault.id = id;
      return vault;
    });
  }
  trashAccount(id: string) {
    this.synced = false;
    for (const vault of Object.values(this.vaults)) {
      let account = vault.removeAccount(id);
      if (!account) continue;
      this.trash.pushAccount(account);
      break;
    }
  }
  deleteAccount(id: string) {
    this.synced = false;
    this.trash.removeAccount(id);
  }
  getAllAccounts(): Account[] {
    let accounts: Account[] = [];

    for (const vault of Object.values(this.vaults)) {
      accounts.push(...vault.accountsList());
    }

    return accounts;
  }
  find(username: string): Account | undefined {
    for (const vault of Object.values(this.vaults)) {
      let account = vault.find(username);
      if (account) return account;
    }
    return;
  }
  merge(data: VaultManager) {}
}

export class Vault {
  accounts: { [key: string]: Account };
  id: string;
  label?: string;

  constructor(id: string | undefined) {
    this.id = id || generateId();
    this.accounts = {};
  }
  init(json: Vault) {
    this.id = json.id;
    this.accounts = json.accounts || {};
    this.label = json.label;
  }
  find(username: string): Account | undefined {
    for (const account of Object.values(this.accounts)) {
      if (account.username === username) return account;
    }
    return;
  }
  pushAccount(account: Account) {
    if (account.id === undefined) account.id = generateId();
    this.accounts[account.id] = account;
  }
  async updateAccount(
    id: string,
    updater: (account: Account) => Promise<Account>,
  ) {
    const account: Account | undefined = this.accounts[id];
    if (!account) throw `Account ${id} was not found`;
    account.id = id;
    this.accounts[id] = await updater(account);
  }
  accountsList(): Account[] {
    return Object.entries(this.accounts).map(([id, account]) => {
      account.id = id;
      return account;
    });
  }
  createAccount(): string {
    let account: Account = {
      urls: [],
    };

    let id;
    while (!id) {
      let generated = generateId();
      if (this.accounts[generated]) continue;
      id = generated;
    }

    this.accounts[id] = account;
    return id;
  }
  removeAccount(id: string): Account | undefined {
    const clone = this.accounts[id];
    delete this.accounts[id];
    return clone;
  }
}
