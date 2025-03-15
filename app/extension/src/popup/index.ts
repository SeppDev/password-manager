import { mount } from "svelte";
import Options from "../components/Options.svelte";

function render() {
  const target = document.getElementById("app");
  if (!target) return

  mount(Options, { target });
}

document.addEventListener("DOMContentLoaded", render);
