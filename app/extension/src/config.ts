// const DOMAIN = "aurapass.nl";
const DOMAIN = "localhost:5173";
const URI = `http://${DOMAIN}`;

export default {
  register_page: "*://localhost/*",
  domain: DOMAIN,
  base: URI,
  // api: `${URI}/api`
  api: `http://localhost:8000/api`,
} as const;
