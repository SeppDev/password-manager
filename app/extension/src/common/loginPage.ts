import config from "../config";

export async function openLoginPage() {
    await browser.tabs.create({
        url: `${config.base}/register`,
    });
}