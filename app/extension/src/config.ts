// const DOMAIN = "aurapass.nl";
const DOMAIN = "localhost:5173";
const URI = `http://${DOMAIN}`;

export default {
  domain: DOMAIN,
  base: URI,
  // api: `${URI}/api`
  api: `http://localhost:8002/api`,
} as const;
