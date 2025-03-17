import { mount } from "svelte";
import Popup from "./Popup.svelte";

function test() {
  console.log("yes");
}

function render() {
  const target = document.getElementById("app");
  if (!target) return

  mount(Popup, { target, props: {test} });
}

document.addEventListener("DOMContentLoaded", render);
