// const DOMAIN = "aurapass.nl";
const DOMAIN = "localhost:5173";
const URI = `http://${DOMAIN}`;

export default {
  register_page: "*://localhost/*",
  domain: DOMAIN,
  base: URI,
  // api: `${URI}/api`
  api: `http://localhost:8001/api`,
} as const;
