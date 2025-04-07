import { mount } from "svelte";
import Popup from "./Popup.svelte";
import config from "../config";

async function loginPage() {
    await browser.tabs.create({
        url: `${config.base}/register`,
    });
}

async function render() {
    const target = document.getElementById("app");
    if (!target) return;


    mount(Popup, { target, props: { loginPage } });
}

document.addEventListener("DOMContentLoaded", render);
