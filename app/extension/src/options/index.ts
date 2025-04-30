import { mount } from "svelte";
import Options from "./Options.svelte";
import config from "../config";

async function render() {
  const target = document.getElementById("app");
  if (!target) return;

  mount(Options, { target });
}

document.addEventListener("DOMContentLoaded", render);
