// const DOMAIN = "aurapass.nl";
const DOMAIN = "localhost:8000";
const URI = `http://${DOMAIN}`;

export default {
    domain: DOMAIN,
    base: URI,
    api: `${URI}/api`
} as const