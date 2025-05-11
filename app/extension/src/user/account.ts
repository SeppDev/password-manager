export interface Account {
  id?: string;
  title?: string;
  email?: string;
  username?: string;
  password?: string;
  totp?: string;
  urls?: string[];
}
