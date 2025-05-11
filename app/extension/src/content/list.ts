import { mount } from "svelte";
import InputButton from "./input/InputButton.svelte";
import tailwindStyles from "../tailwind.css?inline";
import { writable } from "svelte/store";
import { savePrompt } from "../background";

const id = "PASSWORD_MANAGER";

const config = {
  buttonScale: 0.7,
  maxButtonSize: 50,

  inputCheckInterval: 500,
  inputQueryInterval: 1000,
};

const inputs: { [key: string]: string } = {};

const found = document.getElementById(id);
if (found) {
  found.remove();
}

const shadowHost = document.createElement("span");
shadowHost.id = id;
shadowHost.style.position = "absolute";
shadowHost.style.zIndex = "100";

const shadowRoot = shadowHost.attachShadow({ mode: "open" });
const style = document.createElement("style");
style.textContent = tailwindStyles;
shadowRoot.appendChild(style);

const offsetDiv = document.createElement("span");
offsetDiv.style.position = "absolute";
offsetDiv.style.width = "0";
offsetDiv.style.height = "0";
offsetDiv.style.zIndex = "1000";

shadowRoot.appendChild(offsetDiv);

mount(InputButton, { target: offsetDiv });

let activeInput: HTMLElement | undefined = undefined;

function updateActiveInput() {
  if (!activeInput) return;

  if (shadowHost.parentElement != activeInput.parentElement) {
    activeInput.parentElement?.insertBefore(shadowHost, activeInput);
  }

  const inputBounds = activeInput.getBoundingClientRect();
  const shadowBounds = shadowHost.getBoundingClientRect();

  const buttonSize = Math.min(
    inputBounds.height * config.buttonScale,
    config.maxButtonSize,
  );

  const desiredPosition = {
    x:
      inputBounds.x +
      inputBounds.width -
      buttonSize -
      buttonSize * (1 - config.buttonScale),
    y: inputBounds.y + inputBounds.height / 2 - buttonSize / 2,
  };

  const offset = {
    x: desiredPosition.x - shadowBounds.x,
    y: desiredPosition.y - shadowBounds.y,
  };

  offsetDiv.style.height = `${buttonSize}px`;
  offsetDiv.style.width = `${buttonSize}px`;

  offsetDiv.style.left = `${offset.x}px`;
  offsetDiv.style.top = `${offset.y}px`;
}

const knownElements: Array<HTMLElement> = [];

function handleInput(input: HTMLInputElement) {
  if (knownElements.includes(input)) return;
  knownElements.push(input);

  const inputType = input.getAttribute("type");
  if (!activeInput && inputType == "password") {
    activeInput = input;
    updateActiveInput();
  }

  if (!inputs[input.type]) {
    inputs[input.type] = input.value;
  }

  input.addEventListener("mouseenter", () => {
    activeInput = input;
    updateActiveInput();
  });
}

function handleElements(elements: NodeListOf<HTMLInputElement>) {
  elements.forEach((element) => handleInput(element));
}
setInterval(updateActiveInput, config.inputCheckInterval);

const observer = new MutationObserver((mutations) => {
  for (const mutation of mutations) {
    for (const node of mutation.addedNodes) {
      if (!(node instanceof HTMLElement)) continue;
      if (node.tagName !== "INPUT") continue;
      handleInput(node as HTMLInputElement);
    }
  }
});

function queryElements() {
  handleElements(document.querySelectorAll("input[type='text']"));
  handleElements(document.querySelectorAll("input[type='email']"));
  handleElements(document.querySelectorAll("input[type='password']"));
}

queryElements();
setTimeout(queryElements, 1000);

observer.observe(document.body, {
  childList: true,
  subtree: true,
});

function submit() {
  savePrompt.sendMessage(inputs);
}

document.addEventListener("submit", submit);
document.addEventListener("input", (event) => {
  const target = event.target as HTMLInputElement;
  if (target.tagName !== "INPUT") return;
  inputs[target.type] = target.value;
});
