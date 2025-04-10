import { mount } from "svelte";
import Popup from "./Popup.svelte";
import config from "../config";

async function render() {
    const target = document.getElementById("app");
    if (!target) return;

    mount(Popup, { target });
}

// document.addEventListener("DOMContentLoaded", render);
