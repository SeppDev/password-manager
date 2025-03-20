import { mount } from "svelte";
import Popup from "./Popup.svelte";
import Login from "./Login.svelte";

const Storage = {
    session_token: ""
}

async function loginPage() {
    await browser.tabs.create({
        url: browser.runtime.getURL("/src/popup/popup.html?login"),
    });
}

function render() {
    const target = document.getElementById("app");
    if (!target) return;

    if (document.URL.endsWith("?login")) {
      mount(Login, { target });
      return
    }

    mount(Popup, { target, props: { loginPage } });
}

document.addEventListener("DOMContentLoaded", render);
