import { mount } from "svelte";
import Popup from "./Popup.svelte";

async function authenticate() {
  const url = browser.runtime.getURL("src/options/options.html");
  const options = await browser.windows.create({
    url,
    type: "popup",
    height: 600,
    width: 500,
    allowScriptsToClose: true,
  });
  options.alwaysOnTop = true;

  // const options = await browser.tabs.create({
  //   url,
  // });
}

async function render() {
  const target = document.getElementById("app");
  if (!target) return;
  mount(Popup, { target, props: { authenticate } });
}

document.addEventListener("DOMContentLoaded", render);
