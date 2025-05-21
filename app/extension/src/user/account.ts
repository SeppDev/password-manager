export type Account = {
  lastUsed?: number,
  id?: string;
  title?: string;
  email?: string;
  username?: string;
  password?: string;
  totp?: string;
  urls?: string[];
}
