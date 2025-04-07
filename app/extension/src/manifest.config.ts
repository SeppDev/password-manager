import { defineManifest } from "@crxjs/vite-plugin";
import packageJson from "../package.json";
import config from "./config";
// import config from "./config";

const { version } = packageJson;

// Convert from Semver (example: 0.1.0-beta6)
const [major, minor, patch] = version
    // can only contain digits, dots, or dash
    .replace(/[^\d.-]+/g, "")
    // split into version parts
    .split(/[.-]/);

export default defineManifest(async () => ({
    manifest_version: 3,
    name: "Password Manager",
    description: "Epic password manager",
    version: `${major}.${minor}.${patch}`,
    version_name: version,
    icons: {
        "128": "src/assets/icons/icon-128.png",
    },
    content_scripts: [
        {
            matches: ["<all_urls>"],
            js: ["src/content/index.ts"],
        },
        {
            matches: [`${config.register_pages}`],
            js: ["src/content/getToken.ts"]
        }
    ],
    background: {
        "scripts": ["src/background/index.ts"],
        type: "module",
    },
    // options_ui: {
    //     page: "src/options/options.html",
    //     open_in_tab: false,
    // },
    // side_panel: {
    //     default_path: "src/sidepanel/sidepanel.html",
    // },
    action: {
        default_popup: "src/popup/popup.html",
        default_icon: {
            "128": "src/assets/icons/icon-128.png",
        },
    },
    host_permissions: ["http://localhost:8000/*"],
    permissions: [],
}));
