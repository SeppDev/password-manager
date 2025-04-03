// const URI = "https://aurapass.nl"

// export default {
//     base: URI,
//     api: `api.${URI}`
// } as const

export const DOMAIN = "aurapass.nl";
export const URI = "http://localhost:8000";

export default {
    base: URI,
    api: `${URI}/api`
} as const